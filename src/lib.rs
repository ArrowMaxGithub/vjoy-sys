#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(target_os = "windows"))]
/// This library is only available on Windows
struct WindowsOnlyLib{}