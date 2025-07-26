#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#[allow(unsafe_op_in_unsafe_fn)]
pub mod bindings {
    include!("../generated/bindings.rs");
}

pub mod camera;
pub mod device;
pub mod frame;
pub mod pipeline;
pub mod camera_info;
pub mod calibration;
