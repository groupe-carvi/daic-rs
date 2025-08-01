// DepthAI camera_all.rs - Equivalent to C++ camera_all.cpp with stabilized API
// This example demonstrates continuous streaming like the C++ version

use daic_rs::{camera::Camera, device::Device};
use std::time::{Duration, Instant};
mod dev_visualization;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 DepthAI Stabilized API Test with Rerun Visualization");
    println!("═══════════════════════════════════════════════════════");
    
    let config = dev_visualization::CaptureConfig {
        app_name: "depthai_stabilized_api_test".to_string(),
        entity_path: "camera/rgb".to_string(),
        max_frames: None,  // Capture continuous frames for thorough testing
        fps_delay_ms: 50,  // 20 FPS
        stabilization_delay_ms: 1000,
        auto_launch_rerun: true,  // Auto-launch for ease of use
        restart_camera_every: None,
    };
    
    println!("📋 Configuration:");
    println!("   • Max frames: {:?}", config.max_frames);
    println!("   • FPS delay: {}ms", config.fps_delay_ms);
    println!("   • Auto-launch Rerun: {}", config.auto_launch_rerun);
    println!("   • Restart every: {:?} frames", config.restart_camera_every);
    println!();
    
    dev_visualization::run_camera_capture_with_visualization(config)?;
    
    println!("\n✅ Stabilized API test completed successfully!");
    Ok(())
}
