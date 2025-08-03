//! Depth estimation node implementation
//!
//! Provides safe wrapper around DepthAI depth nodes for stereo vision.

use crate::pipeline::PipelineNode;
use crate::error::{DaiError, DaiResult};
use daic_sys::root::dai;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DepthPreset {
    HighAccuracy,
    HighDensity, 
    MediumAccuracy,
    MediumDensity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MedianFilter {
    Kernel3x3,
    Kernel5x5,
    Kernel7x7,
}

impl MedianFilter {
    pub fn as_type(&self) -> u32 {
        match self {
            MedianFilter::Kernel3x3 => 0,
            MedianFilter::Kernel5x5 => 1, 
            MedianFilter::Kernel7x7 => 2,
        }
    }
}

/// Configuration for Depth node
#[derive(Debug, Clone, Default)]
pub struct DepthConfig {
    pub preset: Option<DepthPreset>,
    pub median_filter: Option<MedianFilter>,
    pub left_right_check: bool,
    pub extended_disparity: bool,
    pub subpixel: bool,
}

/// Depth node for stereo depth estimation
pub struct Depth {
    id: String,
    config: DepthConfig,
    inner: Option<Box<dai::node::StereoDepth>>,
    inputs: HashMap<String, String>,
    outputs: HashMap<String, String>,
}

impl Depth {
    /// Create a new depth node
    pub fn new(id: impl Into<String>) -> Self {
        Self::with_config(id, DepthConfig::default())
    }
    
    /// Create a new depth node with custom configuration
    pub fn with_config(id: impl Into<String>, config: DepthConfig) -> Self {
        Self {
            id: id.into(),
            config,
            inner: None,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
    
    /// Set depth preset
    pub fn set_preset(&mut self, preset: DepthPreset) {
        self.config.preset = Some(preset);
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement preset setting
                // dai::node::StereoDepth_setPreset(depth.as_mut(), preset);
            }
        }
    }
    
    /// Enable/disable left-right check
    pub fn set_left_right_check(&mut self, enable: bool) {
        self.config.left_right_check = enable;
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement LR check setting
                // dai::node::StereoDepth_setLeftRightCheck(depth.as_mut(), enable);
            }
        }
    }
    
    /// Enable/disable subpixel mode
    pub fn set_subpixel(&mut self, enable: bool) {
        self.config.subpixel = enable;
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement subpixel setting
                // dai::node::StereoDepth_setSubpixel(depth.as_mut(), enable);
            }
        }
    }
    
    /// Enable/disable extended disparity
    pub fn set_extended_disparity(&mut self, enable: bool) {
        self.config.extended_disparity = enable;
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement extended disparity setting
                // dai::node::StereoDepth_setExtendedDisparity(depth.as_mut(), enable);
            }
        }
    }
    
    /// Set median filter
    pub fn set_median_filter(&mut self, filter: MedianFilter) {
        self.config.median_filter = Some(filter);
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement median filter setting
                // dai::node::StereoDepth_setMedianFilter(depth.as_mut(), filter);
            }
        }
    }
    
    /// Request left input
    pub fn request_left_input(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement left input request
                // dai::node::StereoDepth_requestLeftInput(depth.as_mut());
            }
        }
        
        self.inputs.insert("left".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Request right input
    pub fn request_right_input(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement right input request
                // dai::node::StereoDepth_requestRightInput(depth.as_mut());
            }
        }
        
        self.inputs.insert("right".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Request depth output
    pub fn request_depth_output(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement depth output request
                // dai::node::StereoDepth_requestDepthOutput(depth.as_mut());
            }
        }
        
        self.outputs.insert("depth".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Request disparity output
    pub fn request_disparity_output(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut depth) = self.inner {
            unsafe {
                // TODO: Implement disparity output request
                // dai::node::StereoDepth_requestDisparityOutput(depth.as_mut());
            }
        }
        
        self.outputs.insert("disparity".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Request rectified left output
    pub fn request_rectified_left_output(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        self.outputs.insert("rectified_left".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Request rectified right output
    pub fn request_rectified_right_output(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        self.outputs.insert("rectified_right".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Get depth configuration
    pub fn config(&self) -> &DepthConfig {
        &self.config
    }
    
    /// Get the underlying C++ stereo depth node (for advanced use)
    pub fn as_raw(&self) -> Option<&dai::node::StereoDepth> {
        self.inner.as_ref().map(|d| d.as_ref())
    }
}

impl PipelineNode for Depth {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn node_type(&self) -> String {
        "StereoDepth".to_string()
    }
    
    fn configure(&mut self) -> DaiResult<()> {
        // Create the underlying C++ stereo depth node
        let depth = unsafe {
            let mut depth = Box::new(std::mem::zeroed::<dai::node::StereoDepth>());
            // TODO: Properly initialize stereo depth node
            // dai::node::StereoDepth_StereoDepth(depth.as_mut());
            depth
        };
        
        // Configure the depth node based on our config
        // TODO: Apply configuration to the C++ depth object
        
        self.inner = Some(depth);
        Ok(())
    }
    
    fn inputs(&self) -> Vec<String> {
        self.inputs.values().cloned().collect()
    }
    
    fn outputs(&self) -> Vec<String> {
        self.outputs.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_creation() {
        let depth = Depth::new("test_depth");
        assert_eq!(depth.id(), "test_depth");
        assert_eq!(depth.node_type(), "StereoDepth");
    }
    
    #[test]
    fn test_depth_configuration() {
        let config = DepthConfig {
            preset: Some(DepthPreset::HighAccuracy),
            left_right_check: false,
            subpixel: true,
            extended_disparity: true,
            median_filter: Some(MedianFilter::Kernel5x5),
        };
        
        let depth = Depth::with_config("test_depth", config);
        assert_eq!(depth.config().left_right_check, false);
        assert_eq!(depth.config().subpixel, true);
        assert_eq!(depth.config().extended_disparity, true);
    }
}
