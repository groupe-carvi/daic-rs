#![allow(warnings)]

use bindgen::Bindings;
use cmake::{Config};
use std::fmt::format;
use std::os::unix::process::CommandExt;
use std::{env, path};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

macro_rules! println_info {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
    }
}

fn main() {
    // Check if depthai-core is already available
    let depthai_core_path: PathBuf;

    println_info!("Checking for depthai-core... ");

    // If depthai-core is not available, we need to build it
    // Build depthai-core library

    // Build bindings
    let path_dir: PathBuf = PathBuf::from(env::var("OUT_DIR").unwrap());
    

    if let Some(path) = resolve_depthai_core() {
        depthai_core_path = path;
    } else {
        panic!("Failed to get depthai-core path");
    }

    let build_dir = depthai_core_path.join("build");
    if !build_dir.exists() {
        println_info!("Creating build directory at: {}", build_dir.display());
        std::fs::create_dir_all(&build_dir).expect("Failed to create build directory");
    } else {
        println_info!("Build directory already exists at: {}", build_dir.display());
    }

    let mut depthai_core_dlib: Option<PathBuf> = probe_depthai_core_dlib(build_dir.clone());

    if depthai_core_dlib.is_some() {
        println_info!("Found depthai-core dynamic library at: {}", build_dir.display());
    } else {
        depthai_core_dlib = cmake_build_depthai_core(build_dir.clone());
        println_info!("Built depthai-core dynamic library at: {}", depthai_core_dlib.clone().unwrap().display());
    }



    // Link the depthai-core as a dynamic library
    println!("cargo:rustc-link-search=native={}", &depthai_core_dlib.clone().unwrap().parent().unwrap().display());
    println!("cargo:rustc-link-lib=dylib=libdepthai-core.so");

    let daic_include_path_buff = depthai_core_path.clone().join("include");
    let daic_include_path = String::from(daic_include_path_buff.to_str().unwrap());
    println_info!("Including depthai-core headers to Bindgen from: {}", daic_include_path.clone());
    let daic_depthai_include_path = String::from(daic_include_path_buff.join("depthai").to_str().unwrap());
    println_info!("Including depthai-core depthai headers to Bindgen from: {}", daic_depthai_include_path.clone());
    let daic_header_path = String::from(daic_include_path_buff.join("depthai").join("depthai.hpp").to_str().unwrap());
    println_info!("Using depthai-core header for Bindgen: {}", daic_header_path.clone());

    let mut includes = vec![
        daic_include_path.clone(),
        daic_depthai_include_path.clone(),
        format!("{}/shared/depthai-bootloader-shared/include", depthai_core_path.clone().display()),
    ];

    // Walking through the depthai-core deps directory to find all include directories and add them to the bindings
    println_info!("Walking through depthai-core deps directory to find all include directories...");
    for entry in WalkDir::new(&build_dir.join("_deps")) {
        let entry = entry.expect("Failed to read entry in depthai-core deps directory");
        if entry.file_type().is_dir() {
            let path = entry.path();
            if path.join("include").exists() {
                let include_path = path.join("include");

                let canonical = include_path.canonicalize()
                    .expect("Failed to canonicalize include path");

                includes.push(canonical.to_str().unwrap().to_string());
            }
        }
    }

    println_info!("Found the following include directories: {:?}", includes);

    let bindings: Bindings = bindgen::Builder::default()
        .header((daic_header_path.clone()))
        .clang_arg(format!("-I{}", daic_include_path.clone()))
        .clang_arg(format!("-I{}", daic_depthai_include_path.clone()))
        .clang_arg("-std=c++17")
        .clang_args(includes.iter().map(|s| format!("-I{}", s)))
        .generate()
        .expect("Unable to generate bindings");


    bindings
        .write_to_file(PathBuf::from("./wrapper").join("bindings.rs"))
        .expect("Couldn't write bindings!");

}

/// Build the depthai-core library using CMake.
/// /// # Arguments
/// * `path` - Optional path to the destination directory where the library will
///   be built and copied. If not provided, it defaults to the path_DIR environment variable
///   or the current directory if that is not set.
////// # Panics
/// This function will panic if the CMake build process fails or if the
/// library cannot be copied to the destination directory.
////// # Example
/// ```
/// cmake_build_depthai_core(Some(PathBuf::from("/path/to/destination")));
/// ```
///
/// # Note
/// This function assumes that the CMakeLists.txt file is present in the
/// `src/depthai-core` directory and that the necessary build tools are installed
/// on the system.
fn cmake_build_depthai_core(path: PathBuf) -> Option<PathBuf> {
    // Build depthai-core dynamically
    println_info!("Building depthai-core into dynamic library in {}...", path.display());

     let mut parallel_builds = (num_cpus::get() as f32 + 0.75).to_string();

    if is_wsl::is_wsl() {
        println_info!("Running on WSL, limiting parallel builds to 4 to avoid issues with WSL file system performance.");
        parallel_builds = "4".to_string();
    }

    println_info!("Using {} parallel builds", parallel_builds);

    let mut cmd = Command::new("cmake")
        .arg("-S")
        .arg("wrapper/depthai-core")
        .arg("-B")
        .arg(path.clone())
        .arg("-G")
        .arg("Ninja")
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DDEPTHAI_OPENCV_SUPPORT=OFF")
        .arg("-DBUILD_SHARED_LIBS=ON")
        .output()
        .expect("Failed to run CMake configuration for depthai-core");

    println_info!("CMake configuration complete, building depthai-core...");
    // Build the project
    let status = Command::new("cmake")
        .arg("--build")
        .arg(path.clone())
        .arg("--parallel")
        .arg(parallel_builds.clone())
        .arg("--")
        .arg("-j")
        .arg(parallel_builds.clone())
        .status()
        .expect("Failed to build depthai-core with CMake");

        if !status.success() {
        panic!("Failed to build depthai-core with CMake: {}", String::from_utf8_lossy(status.to_string().as_ref()));
    }

    // let mut config = Config::new("wrapper/depthai-core");
    // config.build_target("depthai-core");
    // config.profile("Release");
    // config.define("CMAKE_BUILD_TYPE", "Release");
    // config.define("DEPTHAI_OPENCV_SUPPORT", "OFF");
    // config.define("BUILD_SHARED_LIBS", "ON");
    // config.define("CMAKE_INSTALL_PREFIX",path.to_str().unwrap());
    // config.build_arg("-j");
    // config.build_arg(parallel_builds);
    // config.generator("Ninja");

    // // Build the project
    // let dst = config.build();

    // Return the path to the built library

    let dst = path.clone();
    dst.join("libdepthai-core.so");
    println_info!("Built depthai-core library at: {}", dst.display());


    return Some(dst);
}

fn resolve_depthai_core() -> Option<PathBuf> {
    // Check if depthai-core is already available
    let res = env::var("DEPTHAI_CORE_ROOT");
    match res {
        Ok(path) => {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() {
                return Some(path_buf)
            } else {
                println_info!("DEPTHAI_CORE_ROOT is set but the path does not exist: {}", path_buf.display());
            }
        }
        Err(_) => {

            let daic_wrapper_dir = PathBuf::from("wrapper/depthai-core");
            if(daic_wrapper_dir.exists()) {
                println_info!("DEPTHAI_CORE_ROOT is not set, found existing depthai-core at: {}", daic_wrapper_dir.display());
                return Some(daic_wrapper_dir);
            }

            println_info!("DEPTHAI_CORE_ROOT is not set and none found, will clone depthai-core to: {}", daic_wrapper_dir.display());
            let clone_cmd = Command::new("git")
                .args(["clone","--recurse-submodules","https://github.com/luxonis/depthai-core.git"])
                .arg("wrapper/depthai-core")
                .status()
                .expect("Failed to clone depthai-core repository");
            if !clone_cmd.success() {
                panic!("Failed to clone depthai-core repository");
            }
            if daic_wrapper_dir.exists() {
                println_info!("Successfully cloned depthai-core repository to: {}", daic_wrapper_dir.display());
                return Some(daic_wrapper_dir);
            } else {
                println_info!("Failed to clone depthai-core repository, path does not exist: {}", daic_wrapper_dir
                    .display());
            }
        }

    }
   return None;
}

fn probe_depthai_core_dlib(out: PathBuf) -> Option<PathBuf> {
    // Check if the depthai-core dynamic library is already available
    let dlib_path = out.join("libdepthai-core.so");
    println_info!("Checking for depthai-core dynamic library at: {}", dlib_path.display());
    if dlib_path.exists() {
        println_info!("Found depthai-core dynamic library at: {}", dlib_path.display());
        return Some(dlib_path);
    }
    else {
        println_info!("depthai-core dynamic library not found at: {}", dlib_path.display());
    }

    return None;
}
