    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;
    use crate::daic;

    /// Type alias for C++ std::string (opaque array)
    pub type CppStdString = crate::bindings::root::__BindgenOpaqueArray<u64, 4>;

    /// A wrapper around C++ std::string that provides safe Rust access
    pub struct CppString(*mut CppStdString);

    impl CppString {
        /// Create a new CppString from a Rust string
        pub fn new(s: &str) -> Result<Self, std::ffi::NulError> {
            let c_str = CString::new(s)?;
            let cpp_str = unsafe { daic::dai_create_std_string(c_str.as_ptr()) };
            Ok(CppString(cpp_str as *mut CppStdString))
        }

        /// Get the string content as a Rust string
        pub fn to_string(&self) -> String {
            unsafe {
                let c_str_ptr = daic::dai_std_string_c_str(self.0 as *const CppStdString);
                if c_str_ptr.is_null() {
                    return String::new();
                }
                CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
            }
        }

        /// Get the string content as a Rust str reference (unsafe due to lifetime)
        pub unsafe fn as_str(&self) -> &str {
            unsafe {
                let c_str_ptr = daic::dai_std_string_c_str(self.0 as *const CppStdString);
                if c_str_ptr.is_null() {
                    return "";
                }
                CStr::from_ptr(c_str_ptr).to_str().unwrap_or("")
            }
        }

        /// Get the length of the string
        pub fn len(&self) -> usize {
            unsafe { daic::dai_std_string_length(self.0 as *const CppStdString) }
        }

        /// Check if the string is empty
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Get a raw pointer to the C++ std::string (for FFI calls)
        pub fn as_ptr(&self) -> *const CppStdString {
            self.0
        }

        /// Get a mutable raw pointer to the C++ std::string (for FFI calls)
        pub fn as_mut_ptr(&mut self) -> *mut CppStdString {
            self.0
        }
    }

    impl Drop for CppString {
        fn drop(&mut self) {
            unsafe {
                daic::dai_std_string_destroy(self.0 as *const CppStdString);
            }
        }
    }

    impl From<&str> for CppString {
        fn from(s: &str) -> Self {
            Self::new(s).expect("Failed to create CppString from str")
        }
    }

    impl From<String> for CppString {
        fn from(s: String) -> Self {
            Self::new(&s).expect("Failed to create CppString from String")
        }
    }

    impl std::fmt::Display for CppString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }

    impl std::fmt::Debug for CppString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CppString(\"{}\")", self.to_string())
        }
    }

    /// Helper function to convert a C string to a Rust String
    pub unsafe fn c_str_to_string(c_str: *const c_char) -> String {
        if c_str.is_null() {
            return String::new();
        }
        unsafe {
            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }

    /// Helper function to convert a Rust string to a C string (caller must free)
    pub fn string_to_c_string(s: &str) -> Result<*mut c_char, std::ffi::NulError> {
        let c_str = CString::new(s)?;
        unsafe {
            Ok(daic::dai_string_to_cstring(c_str.as_ptr()))
        }
    }

    /// Helper function to free a C string
    pub unsafe fn free_c_string(c_str: *mut c_char) {
        unsafe {
            daic::dai_free_cstring(c_str);
        }
    }

    /// Convert opaque std::string to Rust String
    pub unsafe fn opaque_string_to_rust(opaque_str: &CppStdString) -> String {
        unsafe {
            let c_str_ptr = daic::dai_std_string_c_str(opaque_str);
            c_str_to_string(c_str_ptr)
        }
    }

    /// Create an opaque std::string from Rust string
    pub fn rust_string_to_opaque(s: &str) -> Result<*mut CppStdString, std::ffi::NulError> {
        let c_str = CString::new(s)?;
        unsafe {
            Ok(daic::dai_create_std_string(c_str.as_ptr()) as *mut CppStdString)
        }
    }