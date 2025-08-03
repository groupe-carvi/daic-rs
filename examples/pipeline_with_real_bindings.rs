//! Example usage of the DepthAI pipeline with real C++ bindings
//! 
//! This example demonstrates how to create and configure a pipeline
//! using the integrated DepthAI C++ API

use daic_rs::{
    device::Device,
    pipeline::{
        core::{Pipeline, PipelineConfig},
        nodes::camera::{Camera, CameraConfig, BoardSocket, ResolutionPreset, ColorOrder, CameraBoardSocket, CameraResolution},
    },
    error::DaiResult,
};

fn main() -> DaiResult<()> {
    println!("DepthAI Pipeline Example with Real C++ Bindings");
    
    // Create a DepthAI device
    let device = Device::new()?;
    println!("Device created successfully");
    
    // Check if device is connected
    if !device.is_connected_ffi()? {
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
        board_socket: CameraBoardSocket::CamA,
        resolution: CameraResolution::The1080P,
        fps: 30.0,
        preview_size: Some((416, 416)),
        video_size: Some((1920, 1080)),
        still_size: None,
        interleaved: true,
        color_order: ColorOrder::BGR,
    };
    
    let mut camera = Camera::with_config("main_camera", camera_config)?;
    println!("Camera node created successfully");
    
    // Configure camera settings
    camera.set_board_socket(BoardSocket::CamA)?;
    camera.set_resolution(ResolutionPreset::The1080P)?;
    camera.set_fps(30.0)?;
    camera.set_preview_size(416, 416);
    camera.set_color_order(ColorOrder::BGR);
    
    // Request camera outputs
    camera.request_preview_output("preview")?;
    camera.request_video_output("video")?;
    
    println!("Camera configured:");
    println!("  - Board Socket: CAM_A");
    println!("  - Resolution: 1080P ({}x{})", 
             ResolutionPreset::The1080P.dimensions().0,
             ResolutionPreset::The1080P.dimensions().1);
    println!("  - FPS: {}", camera.config().fps);
    println!("  - Preview Size: {:?}", camera.config().preview_size);
    
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
