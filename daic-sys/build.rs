#![allow(warnings)]

use cmake::Config;
use once_cell::sync::Lazy;
use std::{
    env, fs::{self, File}, io::{self, Read, Write}, path::{Path, PathBuf}, process::{Command, ExitStatus, Output}, sync::RwLock, vec
};
use walkdir::WalkDir;
use pkg_config::Config as PkgConfig;
use zip_extensions as zip;

static PROJECT_ROOT: Lazy<PathBuf> = Lazy::new(|| {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| {
        env::current_dir().unwrap().to_str().unwrap().to_string()
    }))
});

static BUILD_FOLDER_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("builds"));

static GEN_FOLDER_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated"));

static DEPTHAI_CORE_ROOT: Lazy<RwLock<PathBuf>> = Lazy::new(|| {
    RwLock::new(PathBuf::from(env::var("DEPTHAI_CORE_ROOT").unwrap_or_else(|_| {
        BUILD_FOLDER_PATH
            .join("depthai-core")
            .to_str()
            .unwrap()
            .to_string()
    })))
});

const DEPTHAI_CORE_REPOSITORY: &str = "https://github.com/luxonis/depthai-core.git";

const DEPTHAI_CORE_BRANCH: &str = "v3.0.0-rc.4";

const DEPTHAI_CORE_WINPREBUILT_URL: &str = "https://github.com/luxonis/depthai-core/releases/download/v3.0.0-rc.4/depthai-core-v3.0.0-rc.4-win64.zip";

const OPENCV_WIN_PREBUILT_URL: &str = "https://github.com/opencv/opencv/releases/download/4.11.0/opencv-4.11.0-windows.exe";

macro_rules! println_build {
    ($($tokens:tt)*) => {
        println!("cargo:warning=\r\x1b[32;1m   {}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper/");
    println!("cargo:rerun-if-changed=builds/depthai-core/include/");
    println_build!("Checking for depthai-core...");


    let depthai_core_lib = resolve_depthai_core_lib()
        .expect("Failed to resolve depthai-core path");
    let windows_static_lib = if cfg!(target_os = "windows") {
        Some(get_depthai_core_root()
        .join("lib")
        .join("depthai-core.lib"))
        
    } else {
        None
    };
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap();
    let deps_dir = target_dir.join("deps");

    if cfg!(target_os = "windows") {
        download_and_prepare_opencv();
    }

    // Build using autocxx instead of bindgen
    build_with_autocxx();

    if cfg!(target_os = "windows") {
        let dlls = [
            "depthai-core.dll",
            "libusb-1.0.dll",
            "opencv_world4110.dll",
        ];

    if windows_static_lib.clone().is_some_and(|p| p.exists()) {
        let lib_path = windows_static_lib.clone().unwrap();
        let lib_name = lib_path.file_name().unwrap().to_str().unwrap();
        println_build!("Found static library: {}", lib_path.display());

        println_build!("Copying {} to {:?}", lib_name, target_dir);
        fs::copy(&lib_path, target_dir.join(lib_name))
            .expect(&format!("Failed to copy {} to debug dir", lib_name));
    }

        for dll in dlls {
            let dll_path = get_depthai_core_root()
                .join("bin")
                .join(dll);

            if dll_path.exists() {

                
                println_build!("Copying {} to {:?}", dll, target_dir);
                //fs::create_dir_all(&target_dir).expect("Failed to create debug dir");
                fs::copy(&dll_path, target_dir.join(dll))
                    .expect(&format!("Failed to copy {} to debug dir", dll));

                println_build!("Copying {} to {:?}", dll, deps_dir);
                //fs::create_dir_all(&deps_dir).expect("Failed to create deps dir");
                fs::copy(&dll_path, deps_dir.join(dll))
                    .expect(&format!("Failed to copy {} to deps dir", dll));
            } else {
                println_build!("DLL not found: {:?}", dll_path);
            }
        }

        let bin_path = get_depthai_core_root().join("bin");

        println!(
            "cargo:rustc-env=PATH={}{}{}",
            bin_path.display(),
            ";",
            env::var("PATH").unwrap()
        );
    } else {
        fs::copy(
            &depthai_core_lib,
            target_dir.join("libdepthai-core.so"),
        );
        fs::copy(
            &depthai_core_lib,
            target_dir.join("deps").join("libdepthai-core.so"),
        );
        println_build!("Depthai-core library copied to: {} and {}", target_dir.to_string_lossy(), target_dir.join("deps").display());
        println_build!("Linux build configuration complete.");
    }
}

fn build_with_autocxx() {
    println_build!("Building with autocxx...");
    
    let includes = get_depthai_includes();
    
    // Create autocxx builder
    let mut builder = autocxx_build::Builder::new(
        "src/lib.rs",
        &[&PROJECT_ROOT.join("wrapper")]
    ).expect("Failed to create autocxx builder");

    // Add all include paths
    for include in &includes {
        println_build!("Adding include path: {}", include.display());
        builder.include(include);
    }

    // Add additional includes from deps
    let deps_includes_path = resolve_deps_includes();
    println_build!(
        "Walking through depthai-core deps directory: {}",
        deps_includes_path.display()
    );

    for entry in WalkDir::new(&deps_includes_path) {
        if let Ok(entry) = entry {
            if entry.file_type().is_dir() && entry.path().join("include").exists() {
                if let Ok(canonical) = entry.path().join("include").canonicalize() {
                    println_build!("Found include directory: {}", canonical.display());
                    builder.include(&canonical);
                }
            }
        }
    }

    // Set C++ standard
    builder.flag_if_supported("-std=c++17");
    if cfg!(target_os = "windows") {
        builder.flag_if_supported("/std:c++17");
    }

    // Build
    builder.build().expect("Failed to build with autocxx");
    
    println_build!("autocxx build completed successfully");
}

fn get_depthai_includes() -> Vec<PathBuf> {
    let mut includes = vec![
        get_depthai_core_root().join("include"),
        get_depthai_core_root().join("include").join("depthai"),
    ];

    let deps_path = BUILD_FOLDER_PATH.join("_deps");

    if deps_path.exists() {
        println_build!("Found depthai-core deps directory at: {}", deps_path.display());
            // Add the deps includes
            includes.push(deps_path.join("libnop-src").join("include"));
            includes.push(deps_path.join("nlohmann_json-src").join("include"));
            includes.push(deps_path.join("xlink-src").join("include"));
            includes.push(deps_path.join("xtensor-src").join("include"));
            includes.push(deps_path.join("xtl-src").join("include"));
    } else {
        println_build!("No depthai-core deps directory found, using core include.");
    }

    // Linux-only additional include
    if cfg!(target_os = "linux") {
        let bootloader = get_depthai_core_root()
            .join("shared")
            .join("depthai-bootloader-shared")
            .join("include");
        if bootloader.exists() {
            includes.push(bootloader);
        }
    }

    includes
}

fn strip_sfx_header(exe_path: &Path, out_7z_path: &Path) {
    let header_size = 6144;

    let mut file = File::open(exe_path)
        .expect("Failed to open OpenCV exe");

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .expect("Failed to read OpenCV exe");

    if buf.len() <= header_size {
        panic!("Exe file too small ({} bytes), cannot strip header. Expected size > {} bytes.", buf.len(), header_size);
    }

    let seven_z_data = &buf[header_size..];

    let mut out_file = File::create(out_7z_path)
        .expect("Failed to create .7z output file");
    out_file.write_all(seven_z_data)
        .expect("Failed to write stripped .7z file");
}

fn download_and_prepare_opencv() {
    if !cfg!(target_os = "windows") {
        return;
    }

    let opencv_dll_file = "opencv_world4110.dll";

    {let dll = get_depthai_core_root()
        .join("bin")
        .join("opencv_world4110.dll");


    if dll.exists() {
        println_build!("opencv_world4110.dll already present, skipping download.");
        return;
    }}

    println_build!("opencv_world4110.dll not found, proceeding to download OpenCV prebuilt binaries...");

    let extraction_dir = BUILD_FOLDER_PATH.join("opencv_download");
    let opencv_exe_path = extraction_dir.join(OPENCV_WIN_PREBUILT_URL.split('/').last().unwrap());
    let extract_path = extraction_dir.join("opencv");
    let dll_path = extract_path
        .join("build")
        .join("x64")
        .join("vc16")
        .join("bin")
        .join(opencv_dll_file);

    if dll_path.exists() {
        println_build!("{} already exists at {:?}",opencv_dll_file.clone() ,dll_path);
        return;
    }

    if !opencv_exe_path.exists() {
        println_build!("OpenCV exe is not downloaded {:?}", opencv_exe_path);

        if !extraction_dir.exists() {
            println_build!("Creating extraction directory: {:?}", extraction_dir);
            fs::create_dir_all(&extraction_dir)
                .expect("Failed to create temp dir for OpenCV download");
        } else {
            println_build!("Extraction directory already exists: {:?}", extraction_dir);
        }

        println_build!("Downloading OpenCV from {}", OPENCV_WIN_PREBUILT_URL);

        let downloaded = download_file(OPENCV_WIN_PREBUILT_URL, &extraction_dir)
            .expect("Failed to download OpenCV prebuilt binary");

        fs::rename(downloaded, &opencv_exe_path)
            .expect("Failed to rename downloaded OpenCV exe");
    } else {
        println_build!("OpenCV exe already downloaded at {:?}", opencv_exe_path);
    }

    if !extract_path.exists() && opencv_exe_path.exists() {
        println_build!("Attempting to extract OpenCV using silent installer...");
        
        let status = Command::new(&opencv_exe_path)
            .arg("-o")
            .arg(&extract_path)
            .arg("-y")
            .status();
            
        match status {
            Ok(exit_status) if exit_status.success() => {
                println_build!("OpenCV extracted successfully using silent installer");
            },
            _ => {
                println_build!("Silent installer failed, trying SFX header stripping...");
                let opencv_7z_path = extraction_dir.join("opencv.7z");
                
                let file_size = fs::metadata(&opencv_exe_path)
                    .expect("Failed to get file metadata")
                    .len();
                    
                if file_size > 10000 {
                    strip_sfx_header(&opencv_exe_path, &opencv_7z_path);
                    
                    println_build!("Extracting .7z payload to {:?}", extract_path);
                    zip::zip_extract(&opencv_7z_path, &extract_path)
                        .expect("Failed to extract OpenCV .7z payload");
                    fs::remove_file(&opencv_7z_path).expect("Failed to remove .7z payload");
                } else {
                    panic!("OpenCV file is too small and extraction methods failed. Please check the download.");
                }
            }
        }
    } else {
        println_build!("OpenCV already extracted at {:?}", extract_path);
    }

    if !dll_path.exists() {
        panic!("{:?} not found in extracted files at {:?}",&opencv_dll_file ,dll_path);
    }

    // Copy and rename to opencv_world4110.dll
    println_build!("Copying and renaming OpenCV DLL...");

    let dest_path = get_depthai_core_root()
        .join("bin")
        .join(&opencv_dll_file);

    fs::copy(&dll_path, &dest_path)
        .expect("Failed to copy OpenCV DLL");

    println_build!("OpenCV DLL copied to {:?}", dest_path);
}

fn resolve_deps_includes() -> PathBuf {
    let build_deps = BUILD_FOLDER_PATH.join("_deps");
    let core_include = get_depthai_core_root().join("include");

    if build_deps.exists() {
        println_build!("Found depthai-core deps directory at: {}", build_deps.display());
        build_deps
    } else if core_include.exists() {
        println_build!("Using depthai-core include directory at: {}", core_include.display());
        core_include
    } else {
        let fallback = PathBuf::from(
            env::var("DEPTHAI_CORE_DEPS_INCLUDE_PATH").unwrap_or_else(|_| {
                build_deps.to_str().unwrap().to_string()
            }),
        );
        println_build!("Using depthai-core deps path from environment variable: {}", fallback.display());
        fallback
    }
}

fn resolve_depthai_core_lib() -> Result<PathBuf, &'static str> {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap();
    let deps_dir = Path::new(&target_dir)
        .join("deps");

    println_build!("Searching for depthai-core library in target directory: {}", target_dir.display());
    if cfg!(target_os = "windows") && target_dir.join("depthai-core.dll").exists() && out_dir.join("depthai-core.lib").exists(){
        println_build!("Found depthai-core.dll in OUT_DIR: {}", target_dir.display());
        return Ok(target_dir.join("depthai-core.dll"));
    }else if target_dir.join("libdepthai-core.so").exists() {
        println_build!("Found libdepthai-core.so in OUT_DIR: {}", target_dir.display());
        return Ok(target_dir.join("depthai-core.so"));
    }
    
    if let Some(found_lib) = probe_depthai_core_lib(BUILD_FOLDER_PATH.clone()) {
        println_build!("Found depthai-core library at: {}", found_lib.display());

        if cfg!(target_os = "windows") {
            // Windows-specific handling
            if found_lib
                .extension()
                .and_then(|e| e.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("dll"))
                .unwrap_or(false)
            {
                let lib_path = if found_lib == get_depthai_core_root().join("bin").join("depthai-core.dll") {
                    found_lib
                        .parent() // bin
                        .and_then(|p| p.parent()) // depthai-core
                        .map(|p| p.join("lib").join("depthai-core.lib"))
                        .ok_or("Could not construct path to depthai-core.lib")?
                } else if found_lib == out_dir.join("depthai-core.lib") {
                    out_dir.join("depthai-core.lib")
                } else {
                    get_depthai_core_root().join("lib").join("depthai-core.lib")
                };

                if !lib_path.exists() {
                    panic!(
                        "Found depthai-core.dll but depthai-core.lib not found at expected location: {}",
                        lib_path.display()
                    );
                }

                println_build!(
                    "Using Windows import library for linking: {}",
                    lib_path.display()
                );
                println!(
                    "cargo:rustc-link-search=native={}",
                    lib_path.parent().unwrap().display()
                );
                println!("cargo:rustc-link-lib=depthai-core");

                
                return Ok(lib_path);
            } else if found_lib
                .extension()
                .and_then(|e| e.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("lib"))
                .unwrap_or(false)
            {
                
                println!(
                    "cargo:rustc-link-search=native={}",
                    found_lib.parent().unwrap().display()
                );
                println!("cargo:rustc-link-lib=depthai-core");
                return Ok(found_lib);
            } else {
                return Err("Unsupported library type found on Windows.");
            }
        } else {
            // Linux
            println!(
                "cargo:rustc-link-search=native={}",
                found_lib.parent().unwrap().display()
            );
            println!("cargo:rustc-link-lib=dylib=depthai-core");
            return Ok(found_lib);
        }
    }

    println_build!("Depthai-core library not found, proceeding to build or download...");

    if cfg!(target_os = "windows") {
        if !get_depthai_core_root().exists() {
            println_build!("DEPTHAI_CORE_ROOT not set, downloading prebuilt depthai-core...");

            let depthai_core_install = get_daic_windows_prebuilt_binary()
                .map_err(|_| "Failed to download prebuilt depthai-core.")?;

            // After extracting, check if the library exists
            if let Some(lib) = probe_depthai_core_lib(depthai_core_install.clone()) {
                return resolve_depthai_core_lib();
            } else {
                panic!("Failed to find depthai-core after downloading prebuilt binary.");
            }
        }
    } else if cfg!(target_os = "linux") {
        if !get_depthai_core_root().exists() {
            let clone_path = BUILD_FOLDER_PATH.join("depthai-core");

            println_build!(
                "Cloning depthai-core repository to {}...",
                clone_path.display()
            );

            clone_repository(DEPTHAI_CORE_REPOSITORY, &clone_path, Some(DEPTHAI_CORE_BRANCH))
                .expect("Failed to clone depthai-core repository");



            let mut new_path = DEPTHAI_CORE_ROOT.write().unwrap();
            *new_path = clone_path.clone();

            println_build!(
                "Updated DEPTHAI_CORE_ROOT to {}",
                new_path.display()
            );
        }
        println_build!("Building depthai-core via CMake for path: {}", BUILD_FOLDER_PATH.display());
        let built_lib =
            cmake_build_depthai_core(BUILD_FOLDER_PATH.clone())
                .expect("Failed to build depthai-core via CMake.");

        println_build!(
            "Built depthai-core dynamic library at: {}",
            built_lib.display()
        );

        println!(
            "cargo:rustc-link-search=native={}",
            built_lib.parent().unwrap().display()
        );
        println!("cargo:rustc-link-lib=dylib=depthai-core");

        return Ok(built_lib);
    }

    Err("Failed to resolve depthai-core library path.")
}


fn probe_depthai_core_lib(out: PathBuf) -> Option<PathBuf> {

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap();
    let deps_dir = Path::new(&target_dir)
        .join("deps");

    let lib_path = if cfg!(target_os = "windows") {
        deps_dir.join("depthai-core.dll")
    } else {
        deps_dir.join("libdepthai-core.so")
        
    };

    println_build!("Searching for depthai-core library in: {}", deps_dir.display());
    let win_static_lib_path = if cfg!(target_os = "windows") && deps_dir.join("depthai-core.lib").exists() {
        Some(deps_dir.join("depthai-core.lib"))
    }else {
        None
    };

    if lib_path.exists() && win_static_lib_path.is_some_and(|p| p.exists()) {
        println_build!("Found depthai-core library at: {}", lib_path.display());
        return Some(lib_path);
    }

    // Check if pkg-config can find depthai-core
    // This is only applicable for Linux and macOS, as Windows does not use pkg-config
    if cfg!(target_os = "linux") || cfg!(target_os = "macos")
    {
        let mut cfg = PkgConfig::new();
        let prob_res = cfg.atleast_version("3.0.0")
        .cargo_metadata(true)
        .probe("depthai-core")
        .ok();

    match prob_res {
        Some(_) => {
            println_build!("Found depthai-core via pkg-config.");
            return Some(out.join("libdepthai-core.so"));
        },
        None => {
            println_build!("depthai-core not found via pkg-config.");
            }
        }
    }

    println_build!("Probing for depthai-core library in: {}", out.display());
    if !out.exists() {
        return None;
    }

    let w = WalkDir::new(&out)
        .into_iter()
        .filter_entry(|entry| {
            entry.file_name() != ".git"
                && entry.file_name() != "include"
                && entry.file_name() != "tests"
                && entry.file_name() != "examples"
                && entry.file_name() != "bindings"
        })
        .filter_map(|e| e.ok())
        .find(|e| {
            let path = e.path();
            path.is_file()
                && matches!(
                    path.file_name().and_then(|n| n.to_str()),
                    Some("libdepthai-core.a")
                        | Some("libdepthai-core.so")
                        | Some("depthai-core.lib")
                        | Some("depthai-core.dll")
                )
        });

    w.map(|entry| entry.path().to_path_buf())
}

fn cmake_build_depthai_core(path: PathBuf) -> Option<PathBuf> {
    println_build!("Building depthai-core with source in {} and target in {}...",get_depthai_core_root().display(), path.display());

    let mut parallel_builds =
        (num_cpus::get() as f32 * 0.80).ceil().to_string();

    if is_wsl() {
        println_build!("Running on WSL, limiting parallel builds to 4.");
        parallel_builds = "4".to_string();
    }

    let ninja_available = is_tool_available("ninja", "--version");
    let generator = if ninja_available { "Ninja" } else { "Unix Makefiles" };

    let mut cmd = Command::new("cmake");
    cmd.arg("-S")
        .arg(get_depthai_core_root().clone())
        .arg("-B")
        .arg(&path)
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DBUILD_SHARED_LIBS=ON")
        .arg("-DDEPTHAI_OPENCV_SUPPORT=OFF")
        .arg("-G")
        .arg(generator);

    let output = cmd
        .output()
        .expect("Failed to run CMake configuration");

    println_build!(
        "CMake output:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    let status = Command::new("cmake")
        .arg("--build")
        .arg(&path)
        .arg("--parallel")
        .arg(&parallel_builds)
        .status()
        .expect("Failed to build depthai-core with CMake");

    if !status.success() {
        panic!("Failed to build depthai-core.");
    }

    let dst = path.join("libdepthai-core.so");
    println_build!("Built depthai-core library at: {}", dst.display());

    Some(dst)
}

fn get_daic_windows_prebuilt_binary() -> Result<PathBuf, String> {
    let mut zip_path = BUILD_FOLDER_PATH.join("depthai-core.zip");

    if !zip_path.exists() {
        let downloaded = download_file(DEPTHAI_CORE_WINPREBUILT_URL, BUILD_FOLDER_PATH.as_path())?;
        zip_path.set_file_name(downloaded.file_name().unwrap());
        fs::rename(&downloaded, &zip_path);
        println_build!("Downloaded prebuilt depthai-core to: {}", downloaded.display());
    }

    println_build!("Extracting prebuilt depthai-core...");
    let extracted_path = BUILD_FOLDER_PATH.join("depthai-core");

    if !extracted_path.exists() {
        zip::zip_extract(
            &zip_path,
            &BUILD_FOLDER_PATH,
        )
        .expect("Failed to extract prebuilt depthai-core");

        let inner_folder = BUILD_FOLDER_PATH.join(
            zip_path
                .file_stem()
                .expect("zip has no stem")
                .to_str()
                .unwrap(),
        );

        fs::rename(&inner_folder, &extracted_path)
            .expect("Failed to rename extracted folder");

        fs::remove_file(&zip_path).expect("Failed to remove zip archive");
    }

    let mut new_path = DEPTHAI_CORE_ROOT.write().unwrap();
    *new_path = extracted_path.clone();

    Ok(extracted_path)
}

fn download_file(url: &str, dest_dir: &Path) -> Result<PathBuf, String> {
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    println_build!("Downloading from: {}", url);
    let response = reqwest::blocking::get(url)
        .map_err(|e| format!("Failed to download file: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()));
    }

    let content_length = response.content_length().unwrap_or(0);
    println_build!("Content length: {} bytes", content_length);

    if content_length == 0 {
        return Err("Downloaded file is empty (0 bytes)".to_string());
    }

    let file_name = url.split('/').last().unwrap_or("downloaded_file");
    let dest_path = dest_dir.join(file_name);

    println_build!("Saving downloaded file to: {}", dest_path.display());

    let bytes = response.bytes().map_err(|e| format!("Failed to read response bytes: {}", e))?;
    
    if bytes.is_empty() {
        return Err("Downloaded content is empty".to_string());
    }

    fs::write(&dest_path, &bytes).map_err(|e| format!("Failed to write file: {}", e))?;

    let written_size = fs::metadata(&dest_path)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();
        
    println_build!("Successfully downloaded {} bytes to {}", written_size, dest_path.display());

    Ok(dest_path)
}

fn clone_repository(repo_url: &str, dest_path: &Path, branch:Option<&str> ) -> Result<(), String> {
    let clone_cmd = if let Some(branch_name) = branch {
        vec!["clone", "--recurse-submodules", "--branch", branch_name, repo_url]
    } else {
        vec!["clone", "--recurse-submodules", repo_url]
    };
    println_build!("Cloning repository {} to {}", repo_url, dest_path.display());
    let status = Command::new("git")
        .args(clone_cmd)
        .arg(dest_path)
        .status()
        .map_err(|e| format!("Failed to clone repository: {}", e))?;

    if !status.success() {
        return Err(format!("Failed to clone repository: {}", status));
    }

    Ok(())
}

fn is_tool_available(tool: &str, vers_cmd: &str) -> bool {
    Command::new(tool)
        .arg(vers_cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn is_wsl() -> bool {
    if cfg!(target_os = "linux") {
        if let Ok(wsl) = std::env::var("WSL_DISTRO_NAME") {
            println_build!("Running on WSL: {}", wsl);
            return true;
        }
    }
    false
}

fn get_depthai_core_root() -> PathBuf {
    DEPTHAI_CORE_ROOT.read().unwrap().to_path_buf()
}
