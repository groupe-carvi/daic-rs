use std::ptr::null;

use daic_sys::bindings::root::dai as dai;
use crate::CameraBoardSocket;

pub struct Device {
    pub(crate) inner: dai::Device,
}
impl Device {
    pub fn new() -> Self {
        Self {
            inner: unsafe{dai::Device::new()},
        }
    }

    pub fn get_connected_cameras(&self) -> Result<Vec<CameraBoardSocket>, String> {
        unsafe {
            // Assuming as_mut_ptr() returns *mut DeviceBase
            let ptr: *mut dai::DeviceBase = &self.inner._base as *const dai::DeviceBase as *mut dai::DeviceBase;
            let camera_count: u8 = dai::DeviceBase_getConnectedCameras(ptr);
            if camera_count == 0 {
                Err("No cameras connected".to_string())
            } else {
                // You need to implement logic to actually retrieve camera objects.
                // Here we just create empty Camera structs for demonstration.
                let cameras: Vec<CameraBoardSocket> = (0..camera_count)
                    .map(|_| CameraBoardSocket { inner: dai::CameraBoardSocket::Type::default() })
                    .collect();
                Ok(cameras)
            }
        }
    }
    pub fn is_valid(&self) -> bool {
        return false;
    }
}