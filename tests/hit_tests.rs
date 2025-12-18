#[cfg(feature = "hit")]
/// This file contains hardware-dependent tests for the depthai-rs library.
/// These tests are only compiled and run when the "hdep-tests" feature is enabled.
///
/// These tests require actual DepthAI hardware to be connected and may take longer to run.
/// They are separated from unit tests to avoid slowing down the development cycle.

/// Tests for the high-level Rust API that require hardware
#[cfg(test)]
mod hardware_integration_tests {
    use depthai::{DaiError, Device, Pipeline, PipelineConfig};

    #[test]
    fn test_device_creation_with_hardware() {
        // This test requires actual DepthAI hardware to be connected
        let device =
            Device::new().expect("Failed to create device - ensure DepthAI hardware is connected");

        // Test that the device is properly initialized
        assert!(device.is_connected(), "Device should be connected");

        // Test device information retrieval
        let device_info = device.get_device_info();
        assert!(device_info.is_ok(), "Should be able to get device info");

        println!("Device created successfully with hardware");
    }

    #[test]
    fn test_pipeline_creation_with_hardware() {
        // This test requires actual hardware and may take several seconds
        let pipeline = Pipeline::new()
            .expect("Failed to create pipeline - ensure DepthAI hardware is connected");

        // Test that the pipeline is properly initialized
        assert!(
            !pipeline.is_running(),
            "Pipeline should not be running initially"
        );

        println!("Pipeline created successfully with hardware");
    }

    #[test]
    fn test_pipeline_with_config_and_hardware() {
        // Test pipeline creation with custom configuration
        let config = PipelineConfig {
            create_implicit_device: true,
            ..Default::default()
        };

        let pipeline = Pipeline::with_config(config)
            .expect("Failed to create pipeline with config - ensure DepthAI hardware is connected");

        // Test configuration retrieval
        let retrieved_config = pipeline.get_config();
        assert!(
            retrieved_config.create_implicit_device,
            "Config should have create_implicit_device set to true"
        );

        println!("Pipeline with config created successfully with hardware");
    }

    #[test]
    fn test_device_ffi_connection_with_hardware() {
        let device =
            Device::new().expect("Failed to create device - ensure DepthAI hardware is connected");

        // Test FFI connection (this requires actual hardware)
        let connected = device.is_connected_ffi();
        assert!(connected, "Device should be connected via FFI");

        println!("Device FFI connection test passed with hardware");
    }
}
