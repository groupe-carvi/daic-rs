#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
pub mod bindings {
    include!("../generated/bindings.rs");
}



#[cfg(test)]
mod tests {
    use super::bindings;

    #[test]
    fn test_bindings() {
        // Example test to ensure bindings are accessible
        let handle = unsafe {
            // Assuming device_create is a function in the bindings that returns a DeviceHandle
            bindings::device_create("Test Device".as_ptr() as *const i8)
        };
        // Check if the handle is not null
        assert!(!handle.is_null(), "Device handle should not be null");
        // Clean up by destroying the device
        unsafe {
            bindings::device_destroy(handle);
        }
    }
}
