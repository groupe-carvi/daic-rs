pub use daic_sys as bindings;

pub mod camera;
pub mod common;
pub mod device;
pub mod error;
pub mod pipeline;

pub use error::{DaicError, Result};
pub use pipeline::{CreateInPipeline, CreateInPipelineWith, DeviceNode, DeviceNodeWithParams};

pub use device::Device;
pub use pipeline::Pipeline;
