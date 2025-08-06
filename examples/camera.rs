use daic_rs::device::{Device, Pipeline};
use daic_rs::camera::MessageQueue;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Camera Startup with DepthAI ===");

    // Create device using safe API
    let device = Device::new()?;
    println!("Device created successfully");
    println!("Device connected: {}", device.is_connected());

    // Create pipeline with device (following C++ pattern)
    let pipeline = Pipeline::new(&device)?;
    println!("Pipeline created successfully");

    // Map to store output queues (equivalent to the C++ std::map)
    let mut output_queues: HashMap<String, MessageQueue> = HashMap::new();

    // Get connected cameras
    let sockets = device.get_connected_cameras()?;
    println!("Found {} connected cameras", sockets.len());

    for socket in sockets {
        println!("Configuring camera for socket: {:?}", socket);
        
        // Create camera node
        let camera = pipeline.create_camera_node()?;
        
        // Get full resolution output using the correct method
        let output = camera.request_full_resolution_output()?;
        
        // Create output queue (this will fail until implemented)
        match output.create_output_queue() {
            Ok(queue) => {
                // Store in our map using socket name as key
                let socket_name = socket.to_string()?;
                output_queues.insert(socket_name.clone(), queue);
                println!("Camera configured for socket: {}", socket_name);
            }
            Err(e) => {
                println!("Warning: Could not create output queue: {}", e);
                // Continue without this camera for now
            }
        }
    }

    // Start the pipeline
    pipeline.start(&device)?;
    println!("Pipeline started successfully");
    println!("Pipeline running: {}", pipeline.is_running());

    // Main loop to get frames (simplified version)
    let mut frame_count = 0;
    let max_frames = 10; // Limit for demo purposes
    
    println!("Starting frame capture loop...");
    while frame_count < max_frames {
        for (name, queue) in &output_queues {
            if let Some(frame_ptr) = queue.get() {
                println!("Got frame from {}: {:p}", name, frame_ptr);
                frame_count += 1;
            }
        }
        
        // Small delay to avoid busy waiting
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Captured {} frames, stopping...", frame_count);

    // The pipeline will automatically stop when dropped
    Ok(())
}