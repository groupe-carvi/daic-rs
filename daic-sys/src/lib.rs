// Use autocxx to generate C++ bindings
use autocxx::prelude::*;

// Include the C++ headers and generate bindings
include_cpp! {
    // Security: Allow all functions and types for now, but limit includes
    
    // Include our wrapper header
    #include "autocxx_wrapper.h"
    
    // Generate bindings for version functions
    generate!("daic::get_build_version")
    generate!("daic::get_version_major")
    generate!("daic::get_version_minor")
    generate!("daic::get_version_patch")
    generate!("daic::get_pre_release_type")
    generate!("daic::get_pre_release_version")
    generate!("daic::get_commit")
    generate!("daic::get_commit_datetime")
    generate!("daic::get_build_datetime")
    generate!("daic::get_device_version")
    generate!("daic::get_bootloader_version")
    generate!("daic::get_device_rvc3_version")
    generate!("daic::get_device_rvc4_version")
    
    // Generate bindings for dai namespace types
    generate!("dai::Device")
    generate!("dai::Pipeline")
    generate!("dai::DeviceInfo")
    generate!("dai::CameraBoardSocket")
    
    // Generate node types
    generate_ns!("dai::node")
    
    // Safety settings
    safety!(unsafe_ffi)
}

pub mod string_utils;

// Simple test module to verify autocxx works
#[cfg(test)]
mod simple_test;

// Re-export for convenience
pub use ffi::*;
