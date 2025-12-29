use std::{env, path::Path};

fn selected_depthai_core_tag() -> String {
    // This mirrors `depthai-sys/build.rs`'s version-selection logic.
    // Feature naming note: Cargo features can't contain '.', so users select `v3-2-1`
    // to mean DepthAI-Core tag `v3.2.1`.
    if env::var_os("CARGO_FEATURE_V3_2_1").is_some() {
        return "v3.2.1".to_string();
    }
    if env::var_os("CARGO_FEATURE_V3_2_0").is_some() {
        return "v3.2.0".to_string();
    }
    if env::var_os("CARGO_FEATURE_V3_1_0").is_some() {
        return "v3.1.0".to_string();
    }

    // Default to the crate version (workspace version is kept aligned with the
    // latest supported DepthAI-Core tag).
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "3.2.1".to_string());
    format!("v{}", pkg_version)
}

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
    let tag = selected_depthai_core_tag();
    let vcpkg_root = target_dir
        .join("dai-build")
        .join(&tag)
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
        // dynamic_calibration is built as a shared library in the depthai-core build tree.
        // It is not part of vcpkg_installed, so we must add it to RUNPATH as well.
        let dcl_dir = target_dir
            .join("dai-build")
            .join(&tag)
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
