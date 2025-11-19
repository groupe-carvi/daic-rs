// Simple test to verify autocxx works
use autocxx::prelude::*;

include_cpp! {
    #include "simple_test.h"

    generate!("test::add")
    generate!("test::get_version")

    safety!(unsafe_ffi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocxx_simple() {
        let result = ffi::test::add(autocxx::c_int(2), autocxx::c_int(3));
        assert_eq!(result, autocxx::c_int(5));
    }

    #[test]
    fn test_autocxx_string() {
        let version = ffi::test::get_version();
        let version_str = unsafe { std::ffi::CStr::from_ptr(version) }
            .to_str()
            .unwrap();
        assert_eq!(version_str, "0.1.0-test");
    }
}
