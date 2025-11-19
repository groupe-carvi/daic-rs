# daic-sys Rewrite: From bindgen to autocxx/cxx

## Overview

This document describes the complete rewrite of `daic-sys` to use `autocxx`, `cxx`, and `autocxx_build` instead of `bindgen` for generating Rust bindings to the depthai-core C++ library.

## What Changed

### Dependencies (Cargo.toml)

**Before:**
- `bindgen = "0.72.0"` (build-dependency)

**After:**
- `autocxx = "0.27"` (dependency)
- `autocxx-build = "0.27"` (build-dependency)
- `cxx-build = "1.0.160"` (build-dependency)

### Build Script (build.rs)

**Key Changes:**
1. Removed `generate_bindings_if_needed()` function that used bindgen
2. Added `build_with_autocxx()` function that uses autocxx_build API
3. Removed generation of static bindings file (`generated/bindings.rs`)
4. Autocxx now generates bindings at compile-time using the `include_cpp!` macro

**Build Process:**
- Autocxx processes the C++ headers during the build
- Include paths are collected and passed to autocxx_build::Builder
- The builder generates Rust bindings on-the-fly

### Library Source (src/lib.rs)

**Before:**
```rust
mod bindings {
    include!("../generated/bindings.rs");
}
pub use bindings::root::daic as daic;
pub use bindings::root::dai as dai;
```

**After:**
```rust
use autocxx::prelude::*;

include_cpp! {
    #include "autocxx_wrapper.h"
    
    generate!("daic::get_build_version")
    generate!("dai::Device")
    generate!("dai::Pipeline")
    // ... more types and functions
    
    safety!(unsafe_ffi)
}

pub use ffi::*;
```

### Wrapper Headers

**Created:**
- `wrapper/autocxx_wrapper.h` - Simplified header for autocxx
  - Uses inline functions for simple wrappers
  - Directly exposes C++ types that autocxx can handle

**Existing:**
- `wrapper/wrapper.h` and `wrapper.cpp` - Still available for complex C wrappers if needed

### String Utilities (src/string_utils.rs)

**Simplified significantly** because autocxx provides native `cxx::CxxString` support:
- Removed complex opaque type wrappers
- Removed manual string conversion functions
- Now relies on autocxx's built-in string handling

## Advantages of autocxx over bindgen

### 1. Better C++ Support
- **autocxx**: Understands C++ semantics (namespaces, templates, overloading)
- **bindgen**: Primarily designed for C APIs, treats C++ as "C with namespaces"

### 2. Type Safety
- **autocxx**: Generates type-safe Rust wrappers that match C++ semantics
- **bindgen**: Generates raw FFI bindings that may not preserve C++ invariants

### 3. String Handling
- **autocxx**: Native `cxx::CxxString` support with automatic conversions
- **bindgen**: Requires manual wrapper code for `std::string`

### 4. Standard Library Support
- **autocxx**: Can handle `std::vector`, `std::unique_ptr`, etc. with cxx bridge
- **bindgen**: Treats them as opaque types requiring manual wrappers

### 5. Compile-Time Generation
- **autocxx**: Bindings generated during compilation based on actual usage
- **bindgen**: Pre-generates all bindings, even unused ones

## Migration Guide

### For Users of daic-sys

The public API should remain largely the same. Key differences:

1. **Type Imports:**
   ```rust
   // Before
   use daic_sys::dai::Device;
   
   // After  
   use daic_sys::ffi::dai::Device;
   // or
   use daic_sys::Device; // if re-exported
   ```

2. **String Handling:**
   ```rust
   // Before
   let s = CppString::new("hello")?;
   
   // After
   let s = cxx::CxxString::from("hello");
   ```

3. **Integer Types:**
   ```rust
   // autocxx uses c_int for int parameters
   let result = ffi::test::add(autocxx::c_int(2), autocxx::c_int(3));
   ```

### For Developers

To add new bindings:

1. Update `wrapper/autocxx_wrapper.h` with the C++ declarations
2. Add `generate!("namespace::function")` to the `include_cpp!` macro in `src/lib.rs`
3. Rebuild - autocxx will generate the bindings automatically

## Testing

A simple test suite (`src/simple_test.rs`) verifies that autocxx integration works:

```rust
#[test]
fn test_autocxx_simple() {
    let result = ffi::test::add(autocxx::c_int(2), autocxx::c_int(3));
    assert_eq!(result, autocxx::c_int(5));
}
```

Run with: `cargo test --lib simple_test`

## Current Limitations

1. **Full depthai-core build**: The complete build is currently blocked by infrastructure issues (vcpkg downloads) unrelated to the autocxx changes

2. **Complex Types**: Some complex C++ types may need manual bridge definitions using cxx

3. **Macros**: C++ macros are not supported - need to be wrapped in inline functions

## Future Work

1. Complete integration with full depthai-core library once infrastructure issues are resolved
2. Add more comprehensive tests
3. Document best practices for adding new bindings
4. Consider using cxx's bridge definitions for complex types

## References

- [autocxx documentation](https://google.github.io/autocxx/)
- [cxx documentation](https://cxx.rs/)
- [depthai-core repository](https://github.com/luxonis/depthai-core)
