//! # DepthAI Pipeline Module
//!
//! This module provides a comprehensive safe Rust API for DepthAI pipelines and nodes.
//! It wraps the native C++ DepthAI API with safe Rust abstractions while preserving 
//! the full functionality of the underlying library.
//!
//! ## Architecture
//!
//! The module is organized into several submodules:
//! - `core` - Core pipeline functionality (Pipeline, Device)
//! - `nodes` - Individual node types (Camera, Neural Network, etc.)
//! - `data` - Data flow types (ImgFrame, NNData, etc.)
//! - `builder` - Builder patterns for pipeline construction
//!
//! ## Example Usage
//!
//! ```rust
//! use daic_rs::pipeline::*;
//!
//! // Create a pipeline with camera and neural network
//! let mut pipeline = PipelineBuilder::new()
//!     .add_camera(CameraBoardSocket::CAM_A)
//!     .add_neural_network("path/to/model.blob")
//!     .build()?;
//!
//! // Connect to device and start
//! let device = Device::new()?;
//! pipeline.start(&device)?;
//! # Ok::<(), daic_rs::DaiError>(())
//! ```

pub mod nodes;
pub mod data;
pub mod builder;

// Re-export main types for convenience
pub use builder::PipelineBuilder;
pub use nodes::{
    Camera, CameraConfig, CameraResolution, MonoCameraResolution, ColorOrder, CameraBoardSocket,
    DepthPreset, ResizeMode, ResizeConfig, CropConfig,
    XLinkIn, XLinkOut,
};
pub use data::{ImgFrame, NNData, IMUData};

// Import DepthAI C bindings from generated bindings
use daic_sys::root::daic::{PipelineHandle, pipeline_create, pipeline_destroy, pipeline_start, pipeline_stop, pipeline_is_running};
use std::collections::HashMap;

use crate::{DaiError, DaiResult, Device};

/// A DepthAI pipeline that defines the computational graph
/// 
/// The Pipeline represents a complete computational workflow that can be executed
/// on a DepthAI device. It manages the lifecycle of the underlying C++ pipeline
pub struct Pipeline {
    handle: PipelineHandle,
    nodes: std::collections::HashMap<String, Box<dyn PipelineNode>>,
    is_running: bool,
    config: PipelineConfig,
}

/// Configuration for pipeline creation
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub create_implicit_device: bool,
    pub holistic_record_enabled: bool,
    pub holistic_replay_path: Option<String>,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            create_implicit_device: false,
            holistic_record_enabled: false,
            holistic_replay_path: None,
        }
    }
}

/// Trait for all pipeline nodes
pub trait PipelineNode {
    /// Get the node ID
    fn id(&self) -> String;
    
    /// Get the node type name
    fn node_type(&self) -> String;
    
    /// Configure the node (called during pipeline build)
    fn configure(&mut self) -> DaiResult<()>;
    
    /// Get input connections
    fn inputs(&self) -> Vec<String> { Vec::new() }
    
    /// Get output connections  
    fn outputs(&self) -> Vec<String> { Vec::new() }
}

impl Pipeline {
    /// Create a new pipeline with default configuration
    pub fn new() -> DaiResult<Self> {
        Self::with_config(PipelineConfig::default())
    }
    
    /// Create a new pipeline with custom configuration
    pub fn with_config(config: PipelineConfig) -> DaiResult<Self> {
        // Create DepthAI pipeline handle using real C bindings
        let handle = unsafe { pipeline_create() };
        if handle.is_null() {
            let error_msg = unsafe {
                let error_ptr = daic_sys::root::daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr)
                        .to_string_lossy()
                        .into_owned()
                } else {
                    "Failed to create pipeline".to_string()
                }
            };
            return Err(DaiError::PipelineStartFailed(error_msg));
        }
        
        Ok(Pipeline {
            handle,
            nodes: HashMap::new(),
            is_running: false,
            config,
        })
    }
    
    /// Add a node to the pipeline
    pub fn add_node<T: PipelineNode + 'static>(&mut self, mut node: T) -> DaiResult<String> {
        node.configure()?;
        let id = node.id();
        self.nodes.insert(id.clone(), Box::new(node));
        Ok(id)
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: &str) -> Option<&dyn PipelineNode> {
        self.nodes.get(id).map(|n| n.as_ref())
    }
    
    /// Start the pipeline on the specified device
    pub fn start(&mut self, device: &Device) -> DaiResult<()> {
        if self.is_running {
            return Err(DaiError::PipelineStartFailed(
                "Pipeline is already running".to_string(),
            ));
        }
        
        // Get device handle from device
        let device_handle = device.get_handle();
        
        // Start pipeline execution using real DepthAI C bindings
        let success = unsafe { pipeline_start(self.handle, device_handle) };
        if !success {
            let error_msg = unsafe {
                let error_ptr = daic_sys::root::daic::dai_get_last_error();
                if !error_ptr.is_null() {
                    std::ffi::CStr::from_ptr(error_ptr)
                        .to_string_lossy()
                        .into_owned()
                } else {
                    "Failed to start pipeline".to_string()
                }
            };
            return Err(DaiError::PipelineStartFailed(error_msg));
        }
        
        self.is_running = true;
        Ok(())
    }
    
    /// Stop the pipeline
    pub fn stop(&mut self) -> DaiResult<()> {
        if !self.is_running {
            return Ok(());
        }
        
        // Stop pipeline execution using real DepthAI C bindings
        unsafe { pipeline_stop(self.handle) };
        
        self.is_running = false;
        Ok(())
    }
    
    /// Check if the pipeline is running
    pub fn is_running(&self) -> bool {
        // Use real DepthAI C binding to check status
        unsafe { pipeline_is_running(self.handle) }
    }
    
    /// Get the schema for this pipeline
    pub fn get_schema(&self) -> DaiResult<String> {
        // TODO: Implement schema generation using C++ bindings
        Ok("{}".to_string())
    }
    
    /// Enable holistic recording
    pub fn enable_holistic_record(&mut self, config: &HolisticRecordConfig) -> DaiResult<()> {
        // TODO: Implement holistic recording using C++ bindings
        Ok(())
    }
    
    /// Enable holistic replay
    pub fn enable_holistic_replay(&mut self, path: &str) -> DaiResult<()> {
        // TODO: Implement holistic replay using C++ bindings
        Ok(())
    }
    
    /// Get the underlying C pipeline handle (for advanced use)
    pub fn get_handle(&self) -> PipelineHandle {
        self.handle
    }
    
    /// Get the pipeline configuration
    pub fn get_config(&self) -> &PipelineConfig {
        &self.config
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        if self.is_running {
            let _ = self.stop();
        }
        
        // Destroy the DepthAI pipeline handle
        if !self.handle.is_null() {
            unsafe { 
                pipeline_destroy(self.handle);
            }
        }
    }
}

// Pipeline is Send but not Sync due to mutable state
unsafe impl Send for Pipeline {}


/// Configuration for holistic recording
#[derive(Debug, Clone)]
pub struct HolisticRecordConfig {
    pub output_path: String,
    pub encode_video: bool,
    pub encode_audio: bool,
}

impl Default for HolisticRecordConfig {
    fn default() -> Self {
        Self {
            output_path: "./recording".to_string(),
            encode_video: true,
            encode_audio: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let result = Pipeline::new();
        match result {
            Ok(_) => println!("Pipeline created successfully"),
            Err(e) => println!("Expected creation error: {}", e),
        }
    }
    
    #[test]
    fn test_pipeline_with_config() {
        let config = PipelineConfig {
            create_implicit_device: true,
            ..Default::default()
        };
        
        let result = Pipeline::with_config(config);
        match result {
            Ok(_) => println!("Pipeline with config created successfully"),
            Err(e) => println!("Expected config error: {}", e),
        }
    }
}

