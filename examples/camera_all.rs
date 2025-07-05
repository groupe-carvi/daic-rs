use daic_rs::bindings::{self, device_create};
use std::ffi::CString;

fn main() {
    // Example usage of the bindings
    let device = unsafe{device_create()};
    
    // Check if the handle is not null
    assert!(!device.is_null(), "Device handle should not be null");
    
    // Clean up by destroying the device
    unsafe {
        bindings::device_destroy(device);
    }
}