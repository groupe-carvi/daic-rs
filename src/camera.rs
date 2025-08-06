use daic_sys::daic;
// use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CameraBoardSocket(pub i32);

/// Safe Rust wrapper for DepthAI Camera Node
pub struct CameraNode {
    handle: daic::DaiCameraNode,
}

impl CameraNode {
    pub(crate) fn new(handle: daic::DaiCameraNode) -> Self {
        Self { handle }
    }

// Adapt according to the actual C++ API if build is needed
// pub fn build(&self, socket: CameraBoardSocket) -> Result<(), String> { ... }

    pub fn request_output(&self, width: i32, height: i32, type_: i32, resize_mode: i32, fps: f32, enable_undistortion: i32) -> Result<Output, String> {
        let handle = unsafe {
            daic::dai_camera_request_output(
                self.handle,
                width,
                height,
                type_,
                resize_mode,
                fps,
                enable_undistortion,
            )
        };
        if handle.is_null() {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "Failed to get camera output".to_string()
                }
            };
            Err(error)
        } else {
            Ok(Output::new(handle as *mut std::ffi::c_void))
        }
    }


    pub fn request_full_resolution_output(&self) -> Result<Output, String> {
        let handle = unsafe { daic::dai_camera_request_full_resolution_output(self.handle) };
        if handle.is_null() {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "Failed to get camera output".to_string()
                }
            };
            Err(error)
        } else {
            Ok(Output::new(handle as *mut std::ffi::c_void))
        }
    }
}

/// Safe Rust wrapper for DepthAI Output
pub struct Output {
    handle: *mut std::ffi::c_void,
}

impl Output {
    pub(crate) fn new(handle: *mut std::ffi::c_void) -> Self {
        Self { handle }
    }

    pub fn create_output_queue(&self) -> Result<MessageQueue, String> {
        // There is no dai_output_create_queue, use DaiDataQueue if needed
        // Placeholder: returns an error
        Err("Not implemented: output queue creation".to_string())
    }
}

/// Safe Rust wrapper for DepthAI Message Queue
pub struct MessageQueue {
    handle: *mut daic::DaiDataQueue,
}

impl MessageQueue {
    pub(crate) fn new(handle: *mut daic::DaiDataQueue) -> Self {
        Self { handle }
    }

    pub fn get(&self) -> Option<*mut std::ffi::c_void> {
        // There is no dai_message_queue_get, use dai_frame_get_data if needed
        None
    }
}

impl Drop for MessageQueue {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_queue_delete(self.handle as *mut ::std::os::raw::c_void) };
        }
    }
}

unsafe impl Send for MessageQueue {}
unsafe impl Sync for MessageQueue {}

impl CameraBoardSocket {
    pub fn to_string(&self) -> Result<String, String> {
        let c_str = unsafe { daic::dai_camera_socket_name(self.0) };
        if c_str.is_null() {
            let error = unsafe {
                let error_ptr = daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
                } else {
                    "Failed to convert socket to string".to_string()
                }
            };
            Err(error)
        } else {
            let result = unsafe {
                let rust_str = std::ffi::CStr::from_ptr(c_str).to_string_lossy().into_owned();
                // Free the C string properly
                daic::dai_free_cstring(c_str as *mut ::std::os::raw::c_char);
                rust_str
            };
            Ok(result)
        }
    }
}
