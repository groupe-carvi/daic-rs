pub mod device;
pub mod frame;
pub mod camera;
pub mod pipeline;
pub mod camera_info;
pub mod calibration;
pub mod device_info;
pub mod misc;

pub use daic_sys::bindings::root::dai as dai;
pub use daic_sys::bindings as bindings;

pub struct CameraBoardSocket {
    pub(crate) inner: dai::CameraBoardSocket::Type,
}

pub enum ReconnectionStatus {
    Reconnected, 
    Reconnecting, 
    ReconnectFailed
}





