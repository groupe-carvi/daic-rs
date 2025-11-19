// String utilities for working with C++ std::string via cxx
// With autocxx, we get native cxx::CxxString support

use std::ffi::{CStr, CString};

/// Helper function to convert a C string to a Rust String
pub unsafe fn c_str_to_string(c_str: *const std::os::raw::c_char) -> String {
    if c_str.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() }
}

/// Convert a Rust &str to a C-compatible string
pub fn str_to_cstring(s: &str) -> Result<CString, std::ffi::NulError> {
    CString::new(s)
}

// Note: With autocxx, we can directly use cxx::CxxString which provides:
// - .to_string_lossy() to convert to Rust String
// - .as_bytes() to get the underlying bytes
// - Direct integration with C++ std::string
//
// This is much cleaner than the previous opaque type approach.
