extern crate bindgen;
use bindgen::CargoCallbacks;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib[KIND:dylib]=vJoyInterface");
    println!("cargo:rerun-if-changed=vjoy/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("vjoy/wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .dynamic_library_name("vJoyInterface")
        .allowlist_type("vJoyInterface")
        .allowlist_function("GetvJoyVersion")
        .allowlist_function("vJoyEnabled")
        .allowlist_function("GetvJoyProductString")
        .allowlist_function("GetvJoyManufacturerString")
        .allowlist_function("GetvJoySerialNumberString")
        .allowlist_function("DriverMatch")
        .allowlist_function("RegisterRemovalCB")
        .allowlist_function("vJoyFfbCap")
        .allowlist_function("GetvJoyMaxDevices")
        .allowlist_function("GetNumberExistingVJD")
        .allowlist_function("GetVJDButtonNumber")
        .allowlist_function("GetVJDDiscPovNumber")
        .allowlist_function("GetVJDContPovNumber")
        .allowlist_function("GetVJDAxisExist")
        .allowlist_function("GetVJDAxisMax")
        .allowlist_function("GetVJDAxisMin")
        .allowlist_function("GetVJDStatus")
        .allowlist_function("isVJDExists")
        .allowlist_function("GetOwnerPid")
        .allowlist_function("AcquireVJD")
        .allowlist_function("RelinquishVJD")
        .allowlist_function("UpdateVJD")
        .allowlist_function("ResetVJD")
        .allowlist_function("UpdateVJD")
        .allowlist_function("ResetAll")
        .allowlist_function("UpdateVJD")
        .allowlist_function("ResetButtons")
        .allowlist_function("UpdateVJD")
        .allowlist_function("ResetPovs")
        .allowlist_function("SetAxis")
        .allowlist_function("SetBtn")
        .allowlist_function("SetDiscPov")
        .allowlist_function("SetContPov")
        .allowlist_function("FfbGetEffect")
        .allowlist_function("FfbRegisterGenCB")
        .allowlist_function("FfbStart")
        .allowlist_function("FfbStop")
        .allowlist_function("IsDeviceFfb")
        .allowlist_function("IsDeviceFfbEffect")
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
