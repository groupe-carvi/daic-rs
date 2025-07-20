pub mod device;

use daic_sys::bindings::root::dai as dai;


pub struct Camera {
    pub(crate) inner: dai::node::Camera,
}

pub struct CameraBoardSocket {
    pub(crate) inner: dai::CameraBoardSocket::Type,
}

pub enum ReconnectionStatus {
Reconnected, 
Reconnecting, 
ReconnectFailed
}





