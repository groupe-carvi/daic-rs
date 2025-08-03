use daic_sys::root::daic::{dai_string_to_cstring, dai_free_cstring};
use daic_sys::root::__BindgenOpaqueArray;
use std::ffi::CStr;

/// Helper function to convert __BindgenOpaqueArray (std::string) to Rust String
/// using the C wrapper functions
pub fn opaque_string_to_rust_string<const N: usize>(opaque_array: &__BindgenOpaqueArray<u64, N>) -> String {
    unsafe {
        // Try to interpret the opaque array as a C string pointer
        // This assumes the first element points to the string data (std::string layout)
        let str_ptr = opaque_array.0[0] as *const std::os::raw::c_char;
        
        // Check if the pointer is valid
        if str_ptr.is_null() {
            return String::new();
        }
        
        // Use the C wrapper function to create a copy
        let c_string_copy = dai_string_to_cstring(str_ptr);
        
        if c_string_copy.is_null() {
            return String::new();
        }
        
        // Convert to Rust String
        let rust_string = CStr::from_ptr(c_string_copy)
            .to_string_lossy()
            .to_string();
        
        // Free the C string copy
        dai_free_cstring(c_string_copy);
        
        rust_string
    }
}