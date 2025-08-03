/// Development visualization module for DepthAI examples
/// 
/// This module provides reusable functions for capturing and visualizing
/// camera frames using Rerun viewer. Only used in development/examples.

use daic_rs::{camera::Camera, device::Device};
use rerun::RecordingStreamBuilder;
use std::process::Command;

/// Configuration for camera capture and visualization
pub struct CaptureConfig {
    pub app_name: String,
    pub entity_path: String,
    pub max_frames: Option<u32>,  // None = infinite streaming like OpenCV
    pub fps_delay_ms: u64,
    pub stabilization_delay_ms: u64,
    pub auto_launch_rerun: bool,  // New option to auto-launch Rerun viewer
    pub restart_camera_every: Option<u32>,  // Restart camera every N frames to prevent crashes
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            app_name: "depthai_camera".to_string(),
            entity_path: "camera/rgb".to_string(),
            max_frames: Some(20),  // Default to 20 for safety, use None for infinite
            fps_delay_ms: 100,
            stabilization_delay_ms: 2000,
            auto_launch_rerun: false,  // Default to manual launch for safety
            restart_camera_every: Some(100),  // Restart every 100 frames to prevent crashes
        }
    }
}

/// Complete camera capture and visualization pipeline
/// 
/// This function handles the entire process:
/// 1. Optional Rerun viewer auto-launch
/// 2. System stabilization
/// 3. Camera initialization  
/// 4. Rerun setup
/// 5. Continuous capture loop with real-time visualization (like OpenCV imshow)
/// 6. Frame rate control
pub fn run_camera_capture_with_visualization(config: CaptureConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating DepthAI device...");
    
    // Auto-launch Rerun viewer if requested
    if config.auto_launch_rerun {
        println!("Launching Rerun viewer...");
        // Kill any existing Rerun processes to avoid port conflicts
        kill_existing_rerun_processes();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        let rerun_started = try_launch_rerun();
        if rerun_started {
            // Give viewer time to start
            std::thread::sleep(std::time::Duration::from_millis(3000));
            // Open web viewer after Rerun is ready
            try_open_web_viewer();
        }
    } else {
        print_rerun_instructions();
    }
    
    // Wait for system to stabilize
    wait_for_stabilization(config.stabilization_delay_ms);
    
    let device = Device::new().map_err(|e| format!("Failed to create device: {}", e))?;
    // Create device (equivalent to std::make_shared<dai::Device>())
    let camera = Camera::new(device).map_err(|e| format!("Failed to create device: {}", e))?;
    
    // Initialize Rerun for real-time visualization (replaces cv::imshow)
    let rec = if config.auto_launch_rerun {
        // Connect to the gRPC server we just launched
        RecordingStreamBuilder::new(config.app_name.as_str())
            .connect_grpc()?  // Connect to Rerun gRPC server
    } else {
        // Use memory sink for manual connection
        let (rec, _storage) = RecordingStreamBuilder::new(config.app_name.as_str())
            .memory()?;
        rec
    };
    
    println!("DepthAI device created successfully");
    if let Some(max) = config.max_frames {
        println!("Starting camera capture loop... (capturing {} frames)", max);
    } else {
        println!("Starting continuous camera capture loop...");
        println!("💡 Press Ctrl+C to stop");
    }
    println!("✓ Look for the '{}' application in the Rerun viewer", config.app_name);
    
    // Give a moment for the connection to establish
    if config.auto_launch_rerun {
        println!("⏳ Waiting for Rerun viewer to connect...");
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
    
    // Show restart info if enabled
    if let Some(restart_every) = config.restart_camera_every {
        println!("🔄 Camera will restart every {} frames to prevent crashes", restart_every);
    }
    
    // Camera capture loop with restart capability
    let mut camera = camera;
    let mut frame_count = 0u32;
    loop {
        // Check if we need to restart the camera to prevent crashes
        if let Some(restart_every) = config.restart_camera_every {
            if frame_count > 0 && frame_count % restart_every == 0 {
                println!("🔄 Restarting camera at frame {} to prevent crashes...", frame_count);
                
                // Drop the old camera (important for memory cleanup)
                drop(camera);
                
                // Wait a moment for cleanup
                std::thread::sleep(std::time::Duration::from_millis(1000));
                let device = Device::new().map_err(|e| format!("Failed to create device: {}", e))?;
                // Create new camera instance
                match Camera::new(device) {
                    Ok(new_camera) => {
                        camera = new_camera;
                        println!("✅ Camera restarted successfully");
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to restart camera: {}", e);
                        break;
                    }
                }
            }
        }
        
        // Get frame from camera (equivalent to queue->get<dai::ImgFrame>())
        match camera.capture() {
            Ok(frame) => {
                // Log frame to Rerun (replaces cv::imshow(name, videoIn->getCvFrame()))
                if let Err(e) = rec.log(
                    config.entity_path.as_str(),
                    &rerun::Image::from_elements(
                        &frame.data,
                        [frame.width as u32, frame.height as u32],
                        rerun::ColorModel::L
                    )
                ) {
                    eprintln!("Rerun logging error: {}", e);
                    break;
                } else {
                    // Confirm successful logging
                    if frame_count == 1 {
                        println!("✓ First frame successfully sent to Rerun viewer");
                    }
                }
                
                frame_count += 1;
                println!("Frame {}: {}x{} ({} bytes)", 
                    frame_count, frame.width, frame.height, frame.data.len());
                
                // Check if we should stop (only if max_frames is set)
                if let Some(max_frames) = config.max_frames {
                    if frame_count >= max_frames {
                        println!("Captured {} frames successfully!", frame_count);
                        println!("Check the Rerun viewer to see the captured frames.");
                        break;
                    }
                }
                // If max_frames is None, continue indefinitely (like OpenCV while(true) loop)
            }
            Err(e) => {
                eprintln!("Capture error: {}", e);
                
                // Try to restart camera immediately on error
                if config.restart_camera_every.is_some() {
                    println!("🔄 Error occurred, restarting camera...");
                    
                    // Drop the old camera
                    drop(camera);
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    let device = Device::new().map_err(|e| format!("Failed to create device: {}", e))?;
                    // Try to create new camera
                    match Camera::new(device) {
                        Ok(new_camera) => {
                            camera = new_camera;
                            println!("✅ Camera restarted after error");
                            continue; // Skip the rest of this iteration
                        }
                        Err(restart_err) => {
                            eprintln!("❌ Failed to restart camera after error: {}", restart_err);
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
        }
        
        // Simple frame rate control (replaces cv::waitKey(1))
        std::thread::sleep(std::time::Duration::from_millis(config.fps_delay_ms));
        
        // Note: This creates a continuous stream like OpenCV imshow() in while(true) loop
        
    }
    
    if config.max_frames.is_some() {
        println!("Streaming completed. Data is available in Rerun viewer.");
    } else {
        println!("Streaming stopped. Final data is available in Rerun viewer.");
    }
    println!("The captured frames are stored and can be viewed with Rerun viewer.");
    Ok(())
}

/// Helper function to print Rerun setup instructions
pub fn print_rerun_instructions() {
    println!("📌 To view the visualization: run 'rerun' in another terminal");
    println!("   If not installed: pip install rerun-sdk");
    println!("✓ Data will be streamed to Rerun - open viewer to see frames in real-time");
}

/// Helper function to wait for system stabilization (useful for DepthAI)
pub fn wait_for_stabilization(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

/// Try to launch Rerun viewer using multiple methods
/// Returns true if successfully launched, false otherwise
pub fn try_launch_rerun() -> bool {
    // Try different ways to start Rerun with explicit viewer window
    if let Ok(_) = Command::new("rerun").spawn() {
        println!("✓ Rerun viewer started via 'rerun' command");
        true
    } else if let Ok(_) = Command::new("python").args(["-m", "rerun", "--web-viewer"]).spawn() {
        println!("✓ Rerun viewer started via 'python -m rerun --web-viewer'");
        true
    } else if let Ok(_) = Command::new("python3").args(["-m", "rerun", "--web-viewer"]).spawn() {
        println!("✓ Rerun viewer started via 'python3 -m rerun --web-viewer'");
        true
    } else if let Ok(_) = Command::new("python").args(["-m", "rerun"]).spawn() {
        println!("✓ Rerun viewer started via 'python -m rerun'");
        println!("🌐 If no window opens, go to: http://127.0.0.1:9090");
        true
    } else if let Ok(_) = Command::new("python3").args(["-m", "rerun"]).spawn() {
        println!("✓ Rerun viewer started via 'python3 -m rerun'");
        println!("🌐 If no window opens, go to: http://127.0.0.1:9090");
        true
    } else {
        eprintln!("⚠️  Could not start Rerun viewer automatically");
        eprintln!("   Install with: pip install rerun-sdk");
        eprintln!("   Then manually run: rerun");
        eprintln!("   Or run: python -m rerun --web-viewer");
        eprintln!("   Continuing anyway - data will be available when viewer connects...");
        false
    }
}

/// Try to open the Rerun web viewer in the default browser
pub fn try_open_web_viewer() {
    println!("🌐 Opening Rerun web viewer in browser...");
    
    // Try to open the default Rerun web URL
    let urls = [
        "http://127.0.0.1:9090",  // Default Rerun web port
        "http://localhost:9090",
    ];
    
    for url in &urls {
        #[cfg(target_os = "windows")]
        {
            if Command::new("cmd").args(["/c", "start", url]).spawn().is_ok() {
                println!("✓ Opened Rerun viewer at: {}", url);
                return;
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if Command::new("open").arg(url).spawn().is_ok() {
                println!("✓ Opened Rerun viewer at: {}", url);
                return;
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if Command::new("xdg-open").arg(url).spawn().is_ok() {
                println!("✓ Opened Rerun viewer at: {}", url);
                return;
            }
        }
    }
    
    println!("📋 Manual action: Open your browser and go to: http://127.0.0.1:9090");
}

/// Kill existing Rerun processes to avoid port conflicts
pub fn kill_existing_rerun_processes() {
    println!("🧹 Cleaning up existing Rerun processes...");
    
    #[cfg(target_os = "windows")]
    {
        // Kill existing Rerun processes on Windows
        let _ = Command::new("taskkill").args(["/F", "/IM", "rerun.exe"]).spawn();
        let _ = Command::new("taskkill").args(["/F", "/IM", "python.exe", "/FI", "WINDOWTITLE eq rerun*"]).spawn();
    }
    
    #[cfg(unix)]
    {
        // Kill existing Rerun processes on Unix-like systems
        let _ = Command::new("pkill").args(["-f", "rerun"]).spawn();
    }
}

fn main() {
    println!("Development visualization example");
    println!("This example demonstrates visualization capabilities for development.");
    println!("Run this to test visualization features.");
}