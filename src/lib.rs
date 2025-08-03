pub mod device;
pub mod frame;
pub mod camera;
pub mod pipeline;
pub mod camera_info;
pub mod calibration;
pub mod device_info;
pub mod misc;
pub mod error;
pub mod common;
pub mod xlink;

pub use daic_sys as bindings;

// Re-export main types for convenience
pub use crate::device::{Device, Platform};
pub use crate::pipeline::{Pipeline, PipelineBuilder};
pub use crate::error::{DaiError, DaiResult};
pub use crate::xlink::{XLinkPlatform, XLinkDeviceState, XLinkProtocol, XLinkError, DeviceDesc};
pub use crate::common::{
    Point2f, Point3f, Size2f, Rect,
    CameraBoardSocket, CameraSensorType, CameraImageOrientation, CameraModel,
    CameraFeatures, CameraSensorConfig, CameraInfo, UsbSpeed, Version, DeviceInfo
};

pub enum ReconnectionStatus {
    Reconnected, 
    Reconnecting, 
    ReconnectFailed
}





