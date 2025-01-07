mod asset_tracking;
pub mod audio;
mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod screens;
mod theme;

use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};

use bevy_parallax::{
    LayerSpeed, LayerData, ParallaxCameraComponent, ParallaxMoveEvent, ParallaxPlugin, ParallaxSystems, CreateParallaxEvent
};

#[derive(Resource)]
pub struct CameraMovement {
    is_moving: bool,
}
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Journey of Tomato Girl".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            demo::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);

        app.add_plugins(ParallaxPlugin)
            .add_systems(Startup, initialize_camera_system) // Spawn the main camera with parallax event
            .insert_resource(CameraMovement { is_moving: true }) // Start moving camera on startup
            .add_systems(Update, move_camera_system.before(ParallaxSystems));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn initialize_camera_system(
    mut commands: Commands,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    let camera = commands
        .spawn((
            Name::new("Camera"),
            Camera2dBundle::default(),
            // Render all UI to this camera.
            // Not strictly necessary since we only use one camera,
            // but if we don't use this component, our UI will disappear as soon
            // as we add another camera. This includes indirect ways of adding cameras like using
            // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
            // for debugging. So it's good to have this here for future-proofing.
            IsDefaultUiCamera,
        ))
        .insert(ParallaxCameraComponent::default())
        .id();
    let event = CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Horizontal(0.9),
                path: "images/bg/sky_lightened.png".to_string(),
                tile_size: UVec2::new(384, 216),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(3.5),
                z: -4.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.8),
                path: "images/bg/far_mountains_fc.png".to_string(),
                tile_size: UVec2::new(384, 216),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(3.5),
                z: -3.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.6),
                path: "images/bg/grassy_mountains_fc.png".to_string(),
                tile_size: UVec2::new(384, 216),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(3.5),
                z: -2.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.2),
                path: "images/bg/clouds_mid_t_fc.png".to_string(),
                tile_size: UVec2::new(384, 216),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(3.5),
                z: -1.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.5),
                path: "images/bg/hill.png".to_string(),
                tile_size: UVec2::new(384, 216),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(3.5),
                z: 0.0,
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Horizontal(0.1),
                path: "images/bg/clouds_front_t_fc.png".to_string(),
                tile_size: UVec2::new(384, 216),
                cols: 1,
                rows: 1,
                scale: Vec2::splat(3.5),
                z: 3.0,
                ..default()
            }
        ],
        camera: camera,
    };
    create_parallax.send(event);
}

fn move_camera_system(
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
    camera_query: Query<Entity, With<Camera>>,
    camera_movement: Res<CameraMovement>,
) {
    let camera = camera_query.get_single().unwrap();
    if camera_movement.is_moving {
        move_event_writer.send(ParallaxMoveEvent {
            translation: Vec2::new(10.0, 0.0),
            rotation: 0.,
            camera: camera,
        });
    } else {
        move_event_writer.send(ParallaxMoveEvent {
            translation: Vec2::new(0.0, 0.0),
            rotation: 0.,
            camera: camera,
        });
    }
}

pub fn move_camera(mut camera_movement: ResMut<CameraMovement>) {
    camera_movement.is_moving = true;
}

pub fn stop_camera(mut camera_movement: ResMut<CameraMovement>) {
    camera_movement.is_moving = false;
}