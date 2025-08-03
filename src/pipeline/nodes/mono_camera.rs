//! Mono camera node implementation
//!
//! Provides safe wrapper around DepthAI mono camera nodes for stereo vision.

use crate::pipeline::PipelineNode;
use crate::error::{DaiResult};
use daic_sys::root::dai;
use std::collections::HashMap;
use crate::common::CameraBoardSocket;

#[derive(Debug, Clone, Copy, PartialEq, Default)] 
pub enum MonoCameraResolution {
    #[default]
    The400P,
    The480P,
    The720P,
    The800P,
    The1200P,
}

/// Configuration for MonoCamera node
#[derive(Debug, Clone)]
pub struct MonoCameraConfig {
    pub board_socket: Option<CameraBoardSocket>,
    pub resolution: MonoCameraResolution,
    pub fps: f32,
}

/// Mono camera node for capturing grayscale images
pub struct MonoCamera {
    id: String,
    config: MonoCameraConfig,
    inner: Option<Box<dai::node::MonoCamera>>,
    outputs: HashMap<String, String>,
}

impl Default for MonoCameraConfig {
    fn default() -> Self {
        Self {
            board_socket: Some(CameraBoardSocket::CamA),
            resolution: MonoCameraResolution::The720P,
            fps: 30.0,
        }
    }
}

impl MonoCamera {
    /// Create a new mono camera node
    pub fn new(id: impl Into<String>) -> Self {
        Self::with_config(id, MonoCameraConfig::default())
    }
    
    /// Create a new mono camera node with custom configuration
    pub fn with_config(id: impl Into<String>, config: MonoCameraConfig) -> Self {
        Self {
            id: id.into(),
            config,
            inner: None,
            outputs: HashMap::new(),
        }
    }
    
    /// Request output stream
    pub fn request_output(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut camera) = self.inner {
            unsafe {
                // TODO: Implement actual output request
                // dai::node::MonoCamera_requestOutput(camera.as_mut());
            }
        }
        
        self.outputs.insert("out".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Set camera board socket
    pub fn set_board_socket(&mut self, socket: CameraBoardSocket) {
        self.config.board_socket = Some(socket);
    }
    
    /// Set camera resolution
    pub fn set_resolution(&mut self, resolution: MonoCameraResolution) {
        self.config.resolution = resolution;
    }
    
    /// Set camera FPS
    pub fn set_fps(&mut self, fps: f32) {
        self.config.fps = fps;
    }
    
    /// Get camera configuration
    pub fn config(&self) -> &MonoCameraConfig {
        &self.config
    }
    
    /// Get the underlying C++ mono camera node (for advanced use)
    pub fn as_raw(&self) -> Option<&dai::node::MonoCamera> {
        self.inner.as_ref().map(|c| c.as_ref())
    }
}

impl PipelineNode for MonoCamera {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn node_type(&self) -> String {
        "MonoCamera".to_string()
    }
    
    fn configure(&mut self) -> DaiResult<()> {
        // Create the underlying C++ mono camera node
        let camera = unsafe {
            let mut camera = Box::new(std::mem::zeroed::<dai::node::MonoCamera>());
            // TODO: Properly initialize mono camera node
            // dai::node::MonoCamera_MonoCamera(camera.as_mut());
            camera
        };
        
        // Configure the camera based on our config
        // TODO: Apply configuration to the C++ camera object
        
        self.inner = Some(camera);
        Ok(())
    }
    
    fn outputs(&self) -> Vec<String> {
        self.outputs.values().cloned().collect()
    }
}

impl MonoCameraResolution {
    /// Get the width and height for this resolution
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            MonoCameraResolution::The400P => (640, 400),
            MonoCameraResolution::The480P => (640, 480),
            MonoCameraResolution::The720P => (1280, 720),
            MonoCameraResolution::The800P => (1280, 800),
            MonoCameraResolution::The1200P => (1920, 1200),
        }
    }
    
    /// Get the pixel count for this resolution
    pub fn pixel_count(&self) -> u32 {
        let (w, h) = self.dimensions();
        w * h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mono_camera_creation() {
        let camera = MonoCamera::new("test_mono_camera");
        assert_eq!(camera.id(), "test_mono_camera");
        assert_eq!(camera.node_type(), "MonoCamera");
    }
    
    #[test]
    fn test_mono_camera_configuration() {
        let config = MonoCameraConfig {
            board_socket: Some(CameraBoardSocket::Auto),
            resolution: MonoCameraResolution::The800P,
            fps: 60.0,
        };
        
        let camera = MonoCamera::with_config("test_mono_camera", config);
        assert_eq!(camera.config().fps, 60.0);
        assert_eq!(camera.config().resolution.dimensions(), (1280, 800));
    }
}
