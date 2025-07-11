/// This file contains hardware-dependent tests for the daic_sys library.
/// These tests are only compiled and run when the "hdep-tests" feature is enabled.
/// 
/// For the moment there tests are using the daic_sys bindings directly.
/// In the future, they may be moved to a separate crate or module.
#[cfg(feature = "hdep-tests")]
mod hdep_binding_tests {
use daic_sys::bindings::{device_create, device_destroy, *};

    #[test]
    fn hdep_test_create_device() {
        let handle = unsafe {
            // Attempt to create a device and get its handle
            device_create()
        };
        // Check if the handle is not null
        assert!(!handle.is_null(), "Device handle should not be null");
        // Clean up by destroying the device
        unsafe {
            device_destroy(handle);
        }
    }
    #[test]
    fn hdep_test_pipeline_create() {
        let device_handle = unsafe {
            // Create a device first
            device_create()
        };
        // Check if the device handle is not null
        assert!(!device_handle.is_null(), "Device handle should not be null");

        // Create a pipeline using the device handle
        let pipeline_handle = unsafe {
            // Attempt to create a pipeline and get its handle
            pipeline_create(device_handle)
        };
        // Check if the handle is not null
        assert!(!pipeline_handle.is_null(), "Pipeline handle should not be null");
        // Clean up by destroying the pipeline
        unsafe {
            // Destroy the pipeline
            pipeline_destroy(pipeline_handle);
            device_destroy(device_handle);
        }
    }
}