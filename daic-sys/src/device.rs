//! Safe Rust API for DepthAI Device

use crate::generated::bindings;
use crate::frame::Frame;
use std::ptr;

pub struct Device {
    inner: *mut bindings::root::dai::DeviceInfo,
}

impl Device {
    /// Open a DepthAI device
    pub fn open() -> Result<Self, &'static str> {
        // Stub: allocate DeviceInfo
        let inner = unsafe { ptr::null_mut() };
        if inner.is_null() {
            Err("No device detected")
        } else {
            Ok(Device { inner })
        }
    }

    /// Capture a frame from the device
    pub fn capture_frame(&self) -> Result<Frame, &'static str> {
        // Stub: return a dummy frame
        Ok(Frame::dummy())
    }
}
