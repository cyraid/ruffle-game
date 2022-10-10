# Ruffle Game

This 'game version' is meant to make it easier for Game Developers wanting to have gamepad / platform support (eg. Steam).
If no SWF is provided on the command-line, Ruffle Game will automatically run the Embedded SWF.

Please see the [Companion App](https://github.com/cyraid/rgcompanion) to replace the Title, Icon, and
Embedded SWF. This will result in having a single standalone executable (like Adobe AIR but for Ruffle).

## Project status

* Gamepad support working.
* ExternalInterface support working.
* Steam (Under consideration)

## Using Ruffle

* ExternalInterface
    * ExternalInterface.call("print", [1, "2"])
    * ExternalInterface.addCallback("gamepad.onChange", function(gamepadId, buttonOrAxis, value) {})
        * Look at desktop/src/gamepad.rs for buttonOrAxis mappings.
        * Value is true/false for buttons, and -1.0 to 1.0 for axis.

## Building from source

[Follow the official guide](https://www.rust-lang.org/tools/install) to install Rust for your platform.

You must also have Java installed, and available on your PATH as `java`.

"ruffle_desktop" will be the Game Player.

### OS

If you are building for a Linux platform, make sure that the GTK 3 development packages are
installed on your system. (Ubuntu: `libgtk-3-dev`, Fedora: `gtk3-devel`)

Use the following command to build and run the desktop app:

`cargo run --release --package=ruffle_desktop`

To run a specific SWF file, pass the SWF path as an argument:

`cargo run --release --package=ruffle_desktop -- test.swf`

To build in debug mode, simply omit `--release` from the command.

## Structure

- `desktop` contains the desktop client (uses `wgpu-rs`)

## License

Ruffle Game is licensed under either of

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (http://opensource.org/licenses/MIT)

at your option.

Ruffle Game depends on third-party libraries under compatible licenses. See [LICENSE.md](LICENSE.md) for full information.

### Contribution

Ruffle Game welcomes contribution from everyone. See [CONTRIBUTING.md](CONTRIBUTING.md) for help getting started.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

The entire Ruffle community, including the chat room and GitHub project, is expected to abide by the [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) that the Rust project itself follows.
