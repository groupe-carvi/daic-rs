// Basic example using only generated bindings
use daic_sys::{
    root::daic::{DeviceHandle, PipelineHandle},
    root::daic::{
        device_create, device_destroy, device_is_connected,
        pipeline_create, pipeline_destroy, pipeline_start, pipeline_stop, pipeline_is_running,
        dai_get_last_error, dai_clear_last_error
    }
};
use std::ffi::CStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a DepthAI device...");
    
    // Create the device
    let device = unsafe { device_create() };
    if device.is_null() {
        let error = unsafe { 
            let err_ptr = dai_get_last_error();
            if !err_ptr.is_null() {
                CStr::from_ptr(err_ptr).to_string_lossy().to_string()
            } else {
                "Unknown error during device creation".to_string()
            }
        };
        return Err(format!("Failed to create device: {}", error).into());
    }
    
    println!("Device created successfully!");
    
    // Check connection
    let is_connected = unsafe { device_is_connected(device) };
    println!("Device connected: {}", is_connected);
    
    // Create a pipeline
    let pipeline = unsafe { pipeline_create() };
    if pipeline.is_null() {
        unsafe { device_destroy(device) };
        return Err("Failed to create pipeline".into());
    }
    
    println!("Pipeline created successfully!");
    
    // Start the pipeline (will probably fail without physical device)
    let pipeline_started = unsafe { pipeline_start(pipeline, device) };
    println!("Pipeline started: {}", pipeline_started);
    
    if pipeline_started {
        let is_running = unsafe { pipeline_is_running(pipeline) };
        println!("Pipeline running: {}", is_running);
        
        // Stop the pipeline
        unsafe { pipeline_stop(pipeline); }
        println!("Pipeline stopped");
    }
    
    // Cleanup
    unsafe { 
        pipeline_destroy(pipeline);
        device_destroy(device);
        dai_clear_last_error();
    }
    
    println!("Cleanup terminated with success!");
    Ok(())
}
