use daic_rs::bindings;
use std::ffi::CString;

fn main() {
    // Example usage of the bindings
    let device_name = "TestDevice";
    let handle = unsafe {
        // Assuming device_create is a function in the bindings that returns a DeviceHandle
        bindings::device_create(CString::new(device_name).unwrap().as_ptr())
    };
    
    // Check if the handle is not null
    assert!(!handle.is_null(), "Device handle should not be null");
    
    // Clean up by destroying the device
    unsafe {
        bindings::device_destroy(handle);
    }
}