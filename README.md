[<img alt="Crates.io" src="https://img.shields.io/crates/v/vjoy-sys">](https://crates.io/crates/vjoy-sys)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/vjoy-sys">](https://docs.rs/vjoy-sys/latest)
<img alt="Crates.io" src="https://img.shields.io/crates/l/vjoy-sys">

## vjoy-sys
Rust bindings to [vJoy](https://github.com/BrunnerInnovation/vJoy) via bindgen.
Built against version 2.2.2.0

## About vJoy
vJoy simulates up to 16 input devices with up to 128 buttons, 8 axes, and 4 hat switches (4-way or continuous).
The virtual devices can be used to 
1) Emulate gamepads/joysticks for older games that require a specific kind of input.
2) Combine multiple physical devices into one virtual.
3) Apply transformations from a physical device to a virtual device (e.g. 2-button to axis rebind, software filtering etc.).

## Usage
The [vJoy driver](https://github.com/BrunnerInnovation/vJoy/releases/tag/v2.2.2.0) version 2.2.2.0 needs to be installed and is only available for Windows.

The vJoy shared library is loaded at runtime via libloading. See the [sanity test](tests/test.rs) for specifics.

## Updating bindings
For a new version of vJoy, the header files in /vjoy have be replaced and may have to be modified (e.g. redefine typedefs, insert "enum" keyword, remove unused includes etc.) to work with bindgen - as of this verison the original C headers resulted in various bindgen errors during build.
