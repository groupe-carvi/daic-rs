use daic_sys::daic;
use crate::camera::{CameraBoardSocket, CameraNode};

/// Safe Rust wrapper for DepthAI Device
pub struct Device {
    handle: daic::DaiDevice,
}

impl Device {
    pub fn new() -> Result<Self, String> {
        let handle = unsafe { daic::dai_device_new() };
        if handle.is_null() {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "Failed to create device".to_string()
                }
            };
            Err(error)
        } else {
            Ok(Device { handle })
        }
    }

    pub fn is_connected(&self) -> bool {
        // Check if device is not closed (inverse logic)
        !unsafe { daic::dai_device_is_closed(self.handle) }
    }

    pub(crate) fn handle(&self) -> daic::DaiDevice {
        self.handle
    }

    pub fn get_connected_cameras(&self) -> Result<Vec<CameraBoardSocket>, String> {
        const MAX_SOCKETS: usize = 16; // Reasonable maximum for DepthAI devices
        let mut sockets = vec![0i32; MAX_SOCKETS];
        
        let count = unsafe { 
            daic::dai_device_get_connected_camera_sockets(self.handle, sockets.as_mut_ptr(), MAX_SOCKETS as i32) 
        };
        
        if count == 0 {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "No cameras found or error getting cameras".to_string()
                }
            };
            return Err(error);
        }
        
        sockets.truncate(count as usize);
        Ok(sockets.into_iter().map(CameraBoardSocket).collect())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_device_delete(self.handle) };
        }
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

/// Safe Rust wrapper for DepthAI Pipeline
pub struct Pipeline {
    handle: daic::DaiPipeline,
}

impl Pipeline {
    pub fn new(_device: &Device) -> Result<Self, String> {
        let handle = unsafe { daic::dai_pipeline_new() };
        if handle.is_null() {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "Failed to create pipeline".to_string()
                }
            };
            Err(error)
        } else {
            Ok(Pipeline { handle })
        }
    }

    pub fn start(&self, device: &Device) -> Result<(), String> {
        let success = unsafe { daic::dai_pipeline_start(self.handle, device.handle()) };
        if success {
            Ok(())
        } else {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "Failed to start pipeline".to_string()
                }
            };
            Err(error)
        }
    }

    pub fn stop(&self) {
        // Note: No dai_pipeline_stop function available in the bindings
        // This might need to be implemented in the C++ wrapper if needed
    }

    pub fn is_running(&self) -> bool {
        // Note: No dai_pipeline_is_running function available in the bindings
        // This might need to be implemented in the C++ wrapper if needed
        false
    }

    pub fn create_camera_node(&self) -> Result<CameraNode, String> {
        let handle = unsafe { daic::dai_pipeline_create_camera(self.handle) };
        if handle.is_null() {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "Failed to create camera node".to_string()
                }
            };
            Err(error)
        } else {
            Ok(CameraNode::new(handle))
        }
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_pipeline_delete(self.handle) };
        }
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}
