//! Safe Rust API for DepthAI camera
use crate::device::Device;
use crate::frame::Frame;

pub struct Camera {
    device: Device,
}

impl Camera {
    /// Initialise une nouvelle caméra DepthAI
    pub fn new() -> Result<Self, &'static str> {
        let device = Device::open()?;
        Ok(Camera { device })
    }

    /// Capture une image depuis la caméra
    pub fn capture(&self) -> Result<Frame, &'static str> {
        self.device.capture_frame()
    }
}
