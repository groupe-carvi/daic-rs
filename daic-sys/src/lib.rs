#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[allow(unsafe_op_in_unsafe_fn)]
mod bindings {
    include!("../generated/bindings.rs");
}

// Re-export the generated bindings for easier access
pub use bindings::*;
