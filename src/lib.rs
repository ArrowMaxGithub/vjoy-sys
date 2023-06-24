#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

pub const AXES_DISPLAY_NAMES: [&str; 8] =
    ["X", "Y", "Z", "Rx", "Ry", "Rz", "Slider", "Dial/Slider2"];
pub const AXES_HID_USAGE: [u32; 8] = [0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37];

pub use self::JOYSTICK_POSITION_V2 as JOYSTICK_POSITION;

#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(target_os = "windows"))]
/// This library is only available on Windows
struct WindowsOnlyLib {}
