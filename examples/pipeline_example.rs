//! Example demonstrating the safe Pipeline API
//! 
//! This example shows how to create a device and pipeline,
//! start and stop the pipeline safely.

use daic_rs::{Device, Pipeline, DaiResult};
use std::thread;
use std::time::Duration;

fn main() -> DaiResult<()> {
    println!("DepthAI Pipeline API Example");
    println!("============================");

    // Create a DepthAI device
    println!("Creating device...");
    let device = Device::new()?;
    println!("✓ Device created successfully");

    // Check device connection
    match device.is_connected_ffi() {
        Ok(true) => println!("✓ Device is connected"),
        Ok(false) => println!("⚠ Device is not connected"),
        Err(e) => println!("⚠ Could not check device connection: {}", e),
    }

    // Create a pipeline
    println!("Creating pipeline...");
    let mut pipeline = Pipeline::new()?;
    println!("✓ Pipeline created successfully");

    // Check initial pipeline state
    match pipeline.is_running() {
        Ok(false) => println!("✓ Pipeline is initially stopped"),
        Ok(true) => println!("⚠ Pipeline is unexpectedly running"),
        Err(e) => println!("⚠ Could not check pipeline status: {}", e),
    }

    // Start the pipeline
    println!("Starting pipeline...");
    match pipeline.start(&device) {
        Ok(()) => {
            println!("✓ Pipeline started successfully");
            
            // Check if pipeline is running
            match pipeline.is_running() {
                Ok(true) => println!("✓ Pipeline is running"),
                Ok(false) => println!("⚠ Pipeline should be running but reports stopped"),
                Err(e) => println!("⚠ Could not check pipeline status: {}", e),
            }

            // Run for a short time
            println!("Running pipeline for 2 seconds...");
            thread::sleep(Duration::from_secs(2));

            // Stop the pipeline
            println!("Stopping pipeline...");
            match pipeline.stop() {
                Ok(()) => println!("✓ Pipeline stopped successfully"),
                Err(e) => println!("⚠ Error stopping pipeline: {}", e),
            }
        }
        Err(e) => {
            println!("⚠ Failed to start pipeline: {}", e);
            println!("  This is expected if no actual DepthAI device is connected");
        }
    }

    println!("\nPipeline example completed!");

    // Demonstrate builder pattern
    println!("\nTesting PipelineBuilder...");
    use daic_rs::PipelineBuilder;
    
    let _pipeline2 = PipelineBuilder::new()
        .build()?;
    println!("✓ PipelineBuilder works correctly");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_no_panic() {
        // This test ensures the example doesn't panic even if hardware is not available
        let result = main();
        // We allow errors here since hardware may not be available in test environment
        match result {
            Ok(()) => println!("Example completed successfully"),
            Err(e) => println!("Example completed with expected error: {}", e),
        }
    }
}
