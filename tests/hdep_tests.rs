
#[cfg(feature = "hdep-tests")]
mod hdep_tests {
use daic_sys::bindings::*;

    #[test]
    fn test_create_device_bindings() {
        // Example test to ensure bindings are accessible
        let handle = unsafe {
            // Assuming device_create is a function in the bindings that returns a DeviceHandle
            device_create()
        };
        // Check if the handle is not null
        assert!(!handle.is_null(), "Device handle should not be null");
        // Clean up by destroying the device
        unsafe {
            device_destroy(handle);
        }
    }
}