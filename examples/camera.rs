use daic_rs::common::CameraBoardSocket;
use daic_rs::device::Device;
use daic_rs::pipeline::Pipeline;
use daic_rs::Result;

fn main() -> Result<()> {
    println!("=== Camera Startup with DepthAI ===");

    // Create device using safe API
    let device = Device::new()?;
    println!("Device created successfully");
    println!("Device connected: {}", device.is_connected());

    // Create pipeline
    let pipeline = Pipeline::new()?;
    println!("Pipeline created successfully");

    // Get connected cameras
    let sockets = device.connected_cameras()?;
    println!("Found {} connected cameras", sockets.len());

    for &socket in &sockets {
        println!("Configuring camera for socket: {:?}", socket);

        // Create camera node
        let _camera = pipeline.create_camera(socket)?;
        println!("Camera node created for socket: {}", socket);
    }

    // Always safe to create at least one node to keep pipeline non-empty.
    // If no cameras were detected, create an Auto camera node for demonstration.
    if sockets.is_empty() {
        let _camera = pipeline.create_camera(CameraBoardSocket::Auto)?;
        println!("No cameras detected; created Camera(Auto) node");
    }

    // Start the pipeline
    pipeline.start_with_device(&device)?;
    println!("Pipeline started successfully");

    println!("Pipeline started; exiting.");

    // The pipeline will automatically stop when dropped
    Ok(())
}
