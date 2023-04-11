#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

pub const AXES_DISPLAY_NAMES: [&str; 16] = [
    "X", 
    "Y", 
    "Z", 
    "Rx", 
    "Ry", 
    "Rz", 
    "Slider", 
    "Dial/Slider2", 
    "Wheel",
    "Accelerator",
    "Brake",
    "Clutch",
    "Steering",
    "Aileron",
    "Rudder",
    "Throttle",
];
pub const AXES_HID_USAGE: [u32; 16] = [
    0x30,
    0x31,
    0x32,
    0x33,
    0x34,
    0x35,
    0x36,
    0x37,
    0x38,
    0xC4,
    0xC5,
    0xC6,
    0xC8,
    0xB0,
    0xBA,
    0xBB,
];

#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(target_os = "windows"))]
/// This library is only available on Windows
struct WindowsOnlyLib{}