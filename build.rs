use std::{env, path::PathBuf};

fn main() {
    // Ensure changes to vcpkg-installed libs re-trigger linkage when present.
    println!("cargo:rerun-if-env-changed=DEPTHAI_RPATH_DISABLE");

    if env::var("DEPTHAI_RPATH_DISABLE").ok().as_deref() == Some("1") {
        return;
    }

    // Embed an rpath for the internal vcpkg lib directory so examples can run
    // without setting LD_LIBRARY_PATH (needed for FFmpeg/libusb when OpenCV videoio is enabled).
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vcpkg_root = manifest_dir
        .join("depthai-sys")
        .join("builds")
        .join("vcpkg_installed");

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
        // Note: cargo:rustc-link-arg applies to this package's final link (bins/examples/tests).
        // Use a single argument with -Wl, to pass through the cc driver.
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            libdir.to_string_lossy()
        );
    }
}
