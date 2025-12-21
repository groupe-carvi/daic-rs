use std::{env, path::Path};

fn main() {
    // Ensure changes to vcpkg-installed libs re-trigger linkage when present.
    println!("cargo:rerun-if-env-changed=DEPTHAI_RPATH_DISABLE");

    if env::var("DEPTHAI_RPATH_DISABLE").ok().as_deref() == Some("1") {
        return;
    }

    // Embed an rpath for the internal vcpkg lib directory so examples can run
    // without setting LD_LIBRARY_PATH (needed for FFmpeg/libusb when OpenCV videoio is enabled).
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(4).unwrap();
    let vcpkg_root = target_dir.join("dai-build").join("vcpkg_installed");

    let target = env::var("TARGET").unwrap_or_default();
    let triplet = if target.contains("aarch64") {
        "arm64-linux"
    } else if target.contains("x86_64") {
        // depthai-core's internal vcpkg commonly uses x64-linux.
        "x64-linux"
    } else {
        "x64-linux"
    };

    let libdir = vcpkg_root.join(triplet).join("lib");
    if libdir.exists() {
        // dynamic_calibration is built as a shared library in the depthai-core build tree.
        // It is not part of vcpkg_installed, so we must add it to RUNPATH as well.
        let dcl_dir = target_dir
            .join("dai-build")
            .join("_deps")
            .join("dynamic_calibration-src")
            .join("lib");

        let mut runpath = libdir.to_string_lossy().to_string();
        if dcl_dir.join("libdynamic_calibration.so").exists() {
            runpath = format!("{}:{}", dcl_dir.to_string_lossy(), runpath);
        }

        // Note: cargo:rustc-link-arg applies to this package's final link (bins/examples/tests).
        // Use a single argument with -Wl, to pass through the cc driver.
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", runpath);
    }
}
