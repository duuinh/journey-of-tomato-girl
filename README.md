# Journey of Tomato Girl

[<img src="./docs/img/thumbnail.png" width=40% height=40% alt="A video tutorial for bevy_new_2d, formerly known as bevy_quickstart"/>](https://www.youtube.com/watch?v=ESBRyXClaYc)

Turn your productivity into adventures!

**Demo:** [itch.io](https://itch.io)

## Run

To run the game locally:

- Use `cargo run` to run a native dev build.
- Use [`trunk serve`](https://trunkrs.dev/) to run a web dev build.

<details>
  <summary>Run release builds</summary>

- Use `cargo run --profile release-native --no-default-features` to run a native release build.
- Use `trunk serve --release --no-default-features` to run a web release build.

</details>

<details>
  <summary>Linux dependencies</summary>

If you are using Linux, make sure you take a look at Bevy's [Linux dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).
Note that this template enables Wayland support, which requires additional dependencies as detailed in the link above.
Wayland is activated by using the `bevy/wayland` feature in the [`Cargo.toml`](./Cargo.toml).

</details>

<details>
    <summary>(Optional) Improve your compile times</summary>

[`.cargo/config_fast_builds.toml`](./.cargo/config_fast_builds.toml) contains documentation on how to set up your environment to improve compile times.
After you've fiddled with it, rename it to `.cargo/config.toml` to enable it.

</details>

## Credits
- [Bevy New 2D template](https://github.com/thebevyflock/bevy-new-2d)
- To do.