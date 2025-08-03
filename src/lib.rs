pub mod device;
pub mod frame;
pub mod camera;
pub mod pipeline;
pub mod camera_info;
pub mod calibration;
pub mod device_info;
pub mod misc;
pub mod error;

pub use daic_sys as bindings;

// Re-export main types for convenience
pub use crate::device::Device;
pub use crate::pipeline::{Pipeline, PipelineBuilder};
pub use crate::error::{DaiError, DaiResult};

pub enum ReconnectionStatus {
    Reconnected, 
    Reconnecting, 
    ReconnectFailed
}





