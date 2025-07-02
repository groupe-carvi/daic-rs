#![allow(warnings)]

use bindgen::Bindings;
use cmake::{Config};
use std::fmt::format;
use std::fs::{self, File};
use std::sync::RwLock;
use std::{env, io, path};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output};
use walkdir::WalkDir;
use once_cell::sync::Lazy;
use zip_extensions as zip;

static BUILD_FOLDER_PATH: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(env::current_dir().unwrap()).join("builds")
});

static GEN_FOLDER_PATH: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(env::current_dir().unwrap()).join("generated")
});

static DEPTHAI_CORE_ROOT: Lazy<RwLock<PathBuf>> = Lazy::new(|| {
    RwLock::new(PathBuf::from(env::var("DEPTHAI_CORE_ROOT").unwrap_or_else(|_| {
        BUILD_FOLDER_PATH.join("depthai-core").to_str().unwrap().to_string()
    })))
});

const DEPTHAI_CORE_REPOSITORY: &str = "https://github.com/luxonis/depthai-core.git";

const DEPTHAI_CORE_WINPREBUILT_URL: &str = "https://github.com/luxonis/depthai-core/releases/download/v3.0.0-rc.2/depthai-core-v3.0.0-rc.2-win64.zip";

macro_rules! println_build {
    ($($tokens: tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
    }
}

fn main() {
    // Check if depthai-core is already available

    println_build!("Checking for depthai-core... ");

        match resolve_depthai_core_lib() {
            Ok(depthai_core_dlib) => {
                println_build!("Resolved depthai-core path: {}", depthai_core_dlib.display());
                   // Link the depthai-core as a dynamic library
                println!("cargo:rustc-link-search=native={}", &depthai_core_dlib.display());
                println!("cargo:rustc-link-lib=dylib=libdepthai-core.so");
            }
            Err(e) => {
                panic!("Failed to resolve depthai-core path: {}", e);
            }
        }

    let binding_needs_regen = GEN_FOLDER_PATH.join("bindings.rs").exists() &&
        env::var("DEPTHAI_FORCE_BINDING_REGEN").is_err();
    // Build bindings

    if binding_needs_regen == true{

         println_build!("Building bindings for depthai-core...");
        
        let daic_include_path_buff = get_depthai_core_root().clone().join("include");
        let daic_include_path = String::from(daic_include_path_buff.to_str().unwrap());
        println_build!("Including depthai-core headers to Bindgen from: {}", daic_include_path.clone());
        let daic_depthai_include_path = String::from(daic_include_path_buff.join("depthai").to_str().unwrap());
        println_build!("Including depthai-core depthai headers to Bindgen from: {}", daic_depthai_include_path.clone());
        let daic_header_path = String::from(daic_include_path_buff.join("depthai").join("depthai.hpp").to_str().unwrap());
        println_build!("Using depthai-core header for Bindgen: {}", daic_header_path.clone());
        
        
        let mut includes = vec![
        daic_include_path.clone(),
        daic_depthai_include_path.clone(),
        format!("{}/shared/depthai-bootloader-shared/include", get_depthai_core_root().clone().display()),
        ];

    // Walking through the depthai-core deps directory to find all include directories and add them to the bindings
    println_build!("Walking through depthai-core deps directory to find all include directories...");
    for entry in WalkDir::new(&BUILD_FOLDER_PATH.join("_deps")) {
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

    println_build!("Found the following include directories: {:?}", includes);

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

}

/// Build the depthai-core library using CMake.
/// /// # Arguments
/// * `path` - Optional path to the destination directory where the library will
///   be built and copied. If not provided, it defaults to the path_DIR environment variable
///   or the current directory if that is not set.
/// /// # Returns
/// * `Option<PathBuf>` - The path to the directory containing the built library if successful, or None
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
    println_build!("Building depthai-core into dynamic library in {}...", path.display());

     let mut parallel_builds = (num_cpus::get() as f32 * 0.80).ceil().to_string();

    if is_wsl() {
        println_build!("Running on WSL, limiting parallel builds to 4 to avoid issues with WSL file system performance.");
        parallel_builds = "4".to_string();
    }

    println_build!("Using {} parallel builds", parallel_builds);

        refresh_env();

    let ninja_available = is_tool_available("ninja", "--version");
    if ninja_available {
        println_build!("Ninja is available, using it for the build process.");
    } else {
        println_build!("Ninja is not available, falling back to Make.");
    }

    println_build!("Starting CMake configuration...");

    let mut cmd: Output;

    if ninja_available {

    }
    else if cfg!(target_os = "windows") {
        cmd = Command::new("cmake")
        .arg("-S")
        .arg(get_depthai_core_root().clone())
        .arg("-B")
        .arg(path.clone())
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DDEPTHAI_OPENCV_SUPPORT=OFF")
        .arg("-DBUILD_SHARED_LIBS=ON")
        .arg("-G")
        .arg("\"MinGW Makefiles\"")
        .output()
        .expect("Failed to run CMake configuration for depthai-core");
    }
    else {
        cmd = Command::new("cmake")
        .arg("-S")
        .arg(get_depthai_core_root().clone())
        .arg("-B")
        .arg(path.clone())
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DDEPTHAI_OPENCV_SUPPORT=OFF")
        .arg("-DBUILD_SHARED_LIBS=ON")
        .output()
        .expect("Failed to run CMake configuration for depthai-core");
    }

    println_build!("CMake configuration complete, building depthai-core...");

    // Build the project

    let status: ExitStatus;

    if ninja_available {
        status = Command::new("cmake")
        .arg("--build")
        .arg(path.clone())
        .arg("--parallel")
        .arg(parallel_builds.clone())
        .arg("--")
        .arg("-j")
        .arg(parallel_builds.clone())
        .status()
        .expect("Failed to build depthai-core with CMake");
        
    } else {
        status = Command::new("cmake")
        .arg("--build")
        .arg(path.clone())
        .arg("--parallel")
        .arg(parallel_builds.clone())
        .status()
        .expect("Failed to build depthai-core with CMake");
    }

        if !status.success() {
        panic!("Failed to build depthai-core with CMake: {}", String::from_utf8_lossy(status.to_string().as_ref()));
    }

    // Return the path to the built library

    let dst = path.clone();
    dst.join("libdepthai-core.so");
    println_build!("Built depthai-core library at: {}", dst.display());


    return Some(dst.parent().unwrap().to_path_buf());
}

fn resolve_depthai_core_lib() -> Result<PathBuf, &'static str> {

    // Check if depthai-core is already available
    match probe_depthai_core_dlib(BUILD_FOLDER_PATH.clone())
    {
        Some(dlib) => {
            println_build!("Found depthai-core dynamic library at: {}", dlib.display());
            return Ok(dlib);
        }
        None => {
            println_build!("Depthai-core dynamic library not found, proceeding to resolve depthai-core path.");
        }
    };

    if get_depthai_core_root().exists() {
        println_build!("Depthai-core found in : {}", get_depthai_core_root().display());
    }
    else if cfg!(target_os = "windows"){
        // Since we are on Windows, we will download the prebuilt depthai-core library
        println_build!("DEPTHAI_CORE_ROOT is not set, will try to download prebuilt depthai-core library.");
        match get_daic_windows_prebuilt_binary()
        {
            Ok(depthai_core_install) => {
                println_build!("Resolved depthai-core path: {}", depthai_core_install.display());
                
                match probe_depthai_core_dlib(depthai_core_install.clone())
                {
                    Some(dlib) => {
                        println_build!("Found depthai-core dynamic library at: {}", dlib.display());
                        return Ok(dlib);
                    }
                    None => {
                        panic!("Failed to find depthai-core dynamic library after downloading prebuilt binary.");
                    }
                }
            }
            Err(e) => {
                panic!("Failed to resolve depthai-core path: {}", e);
            }
        }
    }
    else if cfg!(target_os = "linux") {
        // If DEPTHAI_CORE_ROOT is not set, we will try to clone the depthai-core repository
            let daic_clone_dir = BUILD_FOLDER_PATH.join("depthai-core");

            println_build!("DEPTHAI_CORE_ROOT is not set and none found, will clone depthai-core to: {}", daic_clone_dir.display());
            clone_repository(DEPTHAI_CORE_REPOSITORY, &daic_clone_dir)
            .expect("Failed to clone depthai-core repository");
            if daic_clone_dir.exists() {
                println_build!("Successfully cloned depthai-core repository to: {}", daic_clone_dir.display());
                if daic_clone_dir != get_depthai_core_root(){
                let mut new_path = DEPTHAI_CORE_ROOT.write().unwrap();
                *new_path = daic_clone_dir.clone();
            }
            } else {
                println_build!("Failed to clone depthai-core repository, path does not exist: {}", daic_clone_dir
                    .display());
            }

            let depthai_core_dlib = cmake_build_depthai_core(BUILD_FOLDER_PATH.clone());
            println_build!("Built depthai-core dynamic library at: {}", depthai_core_dlib.clone().unwrap().display());

            return Ok(depthai_core_dlib.unwrap());
        }

    // Check if the depthai-core dynamic library is already available
    println_build!("Checking for depthai-core dynamic library...");

        return Err("Failed to resolve depthai-core dynamic library path");
}

fn probe_depthai_core_dlib(out: PathBuf) -> Option<PathBuf> {
    // Check if the depthai-core dynamic library is already available
    println_build!("Probing for depthai-core dynamic library in: {}", out.display());
    if !out.exists() {
        println_build!("Output directory does not exist: {}", out.display());
        return None;
    }
    let w = walkdir::WalkDir::new(&out)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| {
            let path = e.path();
            if path.is_dir() {
                println_build!("Scanning Directory: {}", path.display());
            }

            if path.is_file() && (path.file_name().map_or(false, |n| n == "libdepthai-core.so" || n == "depthai-core.dll")) {
                return true;
            }
            false
        });

    if let Some(entry) = w {
        let dlib_path = entry.path().to_path_buf();
        println_build!("Found depthai-core dynamic library at: {}", dlib_path.display());
        return Some(dlib_path);
    }

    return None;
}

fn is_wsl() -> bool {
    // Check if the current environment is WSL
    if cfg!(target_os = "linux") {
        if let Ok(wsl) = std::env::var("WSL_DISTRO_NAME") {
            println_build!("Running on WSL: {}", wsl);
            return true;
        }
    }
    false
}

fn is_tool_available(tool: &str, vers_cmd: &str) -> bool {
    let output = Command::new(tool)
        .arg(vers_cmd)
        .output();
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn refresh_env() {
    // Refresh the environment variables
    if cfg!(target_os = "windows") {
        // On Windows, use the refresh_env.cmd script
        Command::new("cmd")
            .args(["/c", "'./scripts/refresh_env.cmd'"])
            .status()
            .expect("Failed to refresh environment variables");
    } else {
        // On Unix-like systems, we can use the `env` command
        Command::new("env")
            .status()
            .expect("Failed to refresh environment variables");
    }
}

fn get_depthai_core_root() -> PathBuf {
    DEPTHAI_CORE_ROOT.read().unwrap().to_path_buf()
}

fn clone_repository(repo_url: &str, dest_path: &Path) -> Result<(), String> {
    // Clone the repository to the specified destination path
    let status = Command::new("git")
        .args(["clone", "--recurse-submodules", repo_url])
        .arg(dest_path)
        .status()
        .map_err(|e| format!("Failed to clone repository: {}", e))?;

    if !status.success() {
        return Err(format!("Failed to clone repository: {}", status));
    }

    Ok(())
}

/// Download a file from the specified URL to the destination path
/// # Arguments
/// * `url` - The URL of the file to download
/// * `dest_dir` - The path where the downloaded file will be saved
/// # Returns
/// * `Ok(PathBuf)` - The path to the downloaded file if successful
/// * `Err(String)` - An error message if the download fails
/// # Example
/// ```
/// let url = "https://example.com/file.zip";
/// let dest_path = PathBuf::from("path/to/save/file.zip");
/// let result = download_file(url, &dest_path);
/// if let Ok(path) = result {
///     println!("File downloaded successfully to: {}", path.display());
/// } else {
///    println!("Failed to download file: {}", result.err().unwrap());
/// }
/// /// # Note
/// This function will create the destination directory if it does not exist.
/// If the destination file already exists, it will return an error.
/// /// # Panics
/// This function will panic if the destination path has no parent directory or if the file cannot be
/// created.
/// It will also panic if the download fails or if the file cannot be written.
fn download_file(url: &str, dest_dir: &Path) -> Result<PathBuf, String> {
    
    // Ensure the destination directory exists\
    if !dest_dir.exists() {
        
        fs::create_dir_all(dest_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Download a file from the specified URL to the destination path
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to download file: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()));
    }

    let file_name = url.split('/').last().unwrap_or("downloaded_file");
    let dest_path = dest_dir.join(file_name);

    println_build!("Saving downloaded file to: {}", dest_path.display());

    let mut file = File::create(dest_path.clone())
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(dest_path.to_path_buf())
}

fn get_daic_windows_prebuilt_binary() -> Result<PathBuf, String> {
    let mut prebuilt_binary_archive: Option<PathBuf> = None;
    if(!BUILD_FOLDER_PATH.join("depthai-core.zip").exists()) {
        //Downloading prebuilt depthai-core
        match download_file(DEPTHAI_CORE_WINPREBUILT_URL, BUILD_FOLDER_PATH.as_path()) {
            Ok(path) => {
                prebuilt_binary_archive = Some(path);
                println_build!("Downloaded prebuilt depthai-core to: {}", prebuilt_binary_archive.clone().unwrap().display());
            }
            Err(e) => {
                return Err(format!("Failed to download prebuilt depthai-core: {}", e));
            }
        }
    }

    // Extract the prebuilt depthai-core library
    println_build!("Extracting prebuilt depthai-core library...");
    if !BUILD_FOLDER_PATH.join("depthai-core").exists() {
        zip_extensions::zip_extract(
            &BUILD_FOLDER_PATH.clone().join(prebuilt_binary_archive.clone().unwrap()),
            &BUILD_FOLDER_PATH.clone(),
        )
        .expect("Failed to extract prebuilt depthai-core library");

        let extracted_path = BUILD_FOLDER_PATH.join(prebuilt_binary_archive.clone().unwrap().file_stem().unwrap());

        println_build!("Extracted prebuilt depthai-core library to: {}", extracted_path.display());

        // Remove the prebuilt depthai-core zip file
        println_build!("Removing prebuilt depthai-core zip file...");
        fs::remove_file(BUILD_FOLDER_PATH.join(prebuilt_binary_archive.clone().unwrap()))
            .expect("Failed to remove prebuilt depthai-core zip file");

        // Rename the extracted directory to depthai-core
        println_build!("Renaming {} to depthai-core...", extracted_path.clone().display());
        fs::rename(&extracted_path, BUILD_FOLDER_PATH.join("depthai-core"))
            .expect("Failed to rename extracted depthai-core directory");

        println_build!("Renamed {} to depthai-core", extracted_path.clone().display());
        // Set the DEPTHAI_CORE_ROOT constant to the new path
        let mut new_path = DEPTHAI_CORE_ROOT.write().unwrap();
        *new_path = BUILD_FOLDER_PATH.join("depthai-core");

        return Ok(BUILD_FOLDER_PATH.join("depthai-core"));
    }

    return Err("Failed to extract prebuilt depthai-core library".to_string());
}
