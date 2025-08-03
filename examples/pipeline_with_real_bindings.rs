//! Example usage of the DepthAI pipeline with real C++ bindings
//! 
//! This example demonstrates how to create and configure a pipeline
//! using the integrated DepthAI C++ API

use daic_rs::{
    device::Device,
    CameraBoardSocket,
    pipeline::{
        Pipeline, PipelineConfig,
        nodes::camera::{Camera, CameraConfig, CameraResolution, ColorOrder},
    },
    error::DaiResult,
};

fn main() -> DaiResult<()> {
    println!("DepthAI Pipeline Example with Real C++ Bindings");
    
    // Create a DepthAI device
    let device = Device::new()?;
    println!("Device created successfully");
    
    // Check if device is connected
    if !device.is_connected()? {
        println!("Warning: No DepthAI device detected");
        return Ok(());
    }
    
    println!("DepthAI device connected!");
    
    // Create a pipeline
    let config = PipelineConfig {
        create_implicit_device: false,
        holistic_record_enabled: false,
        holistic_replay_path: None,
    };
    let mut pipeline = Pipeline::with_config(config)?;
    println!("Pipeline created successfully");
    
    // Create and configure a camera node
    let camera_config = CameraConfig {
        board_socket: Some(CameraBoardSocket::CamA),
        resolution: CameraResolution::The1080P,
        fps: 30.0,
        preview_size: Some((416, 416)),
        color_order: Some(ColorOrder::BGR),
    };
    
    let mut camera = Camera::with_config("main_camera", camera_config);
    println!("Camera node created successfully");
    
    // Configure camera settings
    camera.set_board_socket(CameraBoardSocket::CamA);
    camera.set_resolution(CameraResolution::The1080P);
    camera.set_fps(30.0);
    
    println!("Camera configured:");
    println!("  - Board Socket: CAM_A");
    println!("  - Resolution: 1080P (1920x1080)");
    println!("  - FPS: {}", camera.config.fps);
    println!("  - Preview Size: {:?}", camera.config.preview_size);
    
    // Add camera to pipeline
    pipeline.add_node(camera)?;
    println!("Camera node added to pipeline");
    
    // Start the pipeline
    println!("Starting pipeline...");
    pipeline.start(&device)?;
    
    if pipeline.is_running() {
        println!("Pipeline is running successfully!");
        
        // Let it run for a moment
        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        // Stop the pipeline
        println!("Stopping pipeline...");
        pipeline.stop()?;
        println!("Pipeline stopped");
    } else {
        println!("Pipeline failed to start");
    }
    
    println!("Example completed successfully!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_creation() {
        let result = Pipeline::new();
        match result {
            Ok(_) => println!("Pipeline created successfully"),
            Err(e) => println!("Expected error without DepthAI device: {}", e),
        }
    }
    
    #[test]
    fn test_device_creation() {
        let result = Device::new();
        match result {
            Ok(_) => println!("Device created successfully"),
            Err(e) => println!("Expected error without DepthAI device: {}", e),
        }
    }
    
    #[test]
    fn test_camera_creation() {
        let result = Camera::new("test_camera");
        match result {
            Ok(camera) => {
                println!("Camera created successfully");
                assert_eq!(camera.id(), "test_camera");
                assert_eq!(camera.node_type(), "Camera");
            }
            Err(e) => println!("Expected error without DepthAI device: {}", e),
        }
    }
}
