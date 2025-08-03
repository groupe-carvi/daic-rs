//! Neural network node implementation
//!
//! Provides safe wrapper around DepthAI neural network nodes.

use crate::pipeline::PipelineNode;
use crate::error::{DaiError, DaiResult};
use crate::bindings::root::dai;
use std::collections::HashMap;
use std::path::Path;

/// Neural network node for running AI models
pub struct NeuralNetwork {
    id: String,
    config: NeuralNetworkConfig,
    inner: Option<Box<dai::node::NeuralNetwork>>,
    inputs: HashMap<String, String>,
    outputs: HashMap<String, String>,
}

/// Configuration for NeuralNetwork node
#[derive(Debug, Clone)]
pub struct NeuralNetworkConfig {
    pub blob_path: String,
    pub num_threads: Option<u32>,
    pub num_nce_per_thread: Option<u32>,
    pub input_size: Option<(u32, u32)>,
    pub confidence_threshold: Option<f32>,
}

impl Default for NeuralNetworkConfig {
    fn default() -> Self {
        Self {
            blob_path: String::new(),
            num_threads: None,
            num_nce_per_thread: None,
            input_size: None,
            confidence_threshold: None,
        }
    }
}

impl NeuralNetwork {
    /// Create a new neural network node
    pub fn new(id: impl Into<String>, blob_path: impl Into<String>) -> Self {
        let config = NeuralNetworkConfig {
            blob_path: blob_path.into(),
            ..Default::default()
        };
        Self::with_config(id, config)
    }
    
    /// Create a new neural network node with custom configuration
    pub fn with_config(id: impl Into<String>, config: NeuralNetworkConfig) -> Self {
        Self {
            id: id.into(),
            config,
            inner: None,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
    
    /// Set the blob path
    pub fn set_blob_path(&mut self, path: impl Into<String>) -> DaiResult<()> {
        let path = path.into();
        
        // Validate that the blob file exists
        if !Path::new(&path).exists() {
            return Err(DaiError::FileNotFound(format!("Blob file not found: {}", path)));
        }
        
        self.config.blob_path = path;
        
        // If the inner node exists, update it
        if let Some(ref mut nn) = self.inner {
            unsafe {
                // TODO: Implement blob path setting
                // dai::node::NeuralNetwork_setBlobPath(nn.as_mut(), self.config.blob_path.as_ptr());
            }
        }
        
        Ok(())
    }
    
    /// Set number of threads
    pub fn set_num_threads(&mut self, num_threads: u32) {
        self.config.num_threads = Some(num_threads);
        
        if let Some(ref mut nn) = self.inner {
            unsafe {
                // TODO: Implement thread count setting
                // dai::node::NeuralNetwork_setNumThreads(nn.as_mut(), num_threads);
            }
        }
    }
    
    /// Set number of NCE per thread
    pub fn set_num_nce_per_thread(&mut self, num_nce: u32) {
        self.config.num_nce_per_thread = Some(num_nce);
        
        if let Some(ref mut nn) = self.inner {
            unsafe {
                // TODO: Implement NCE setting
                // dai::node::NeuralNetwork_setNumNCEPerThread(nn.as_mut(), num_nce);
            }
        }
    }
    
    /// Set input size for preprocessing
    pub fn set_input_size(&mut self, width: u32, height: u32) {
        self.config.input_size = Some((width, height));
    }
    
    /// Set confidence threshold
    pub fn set_confidence_threshold(&mut self, threshold: f32) {
        self.config.confidence_threshold = Some(threshold);
    }
    
    /// Request input stream
    pub fn request_input(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut nn) = self.inner {
            unsafe {
                // TODO: Implement input request
                // dai::node::NeuralNetwork_requestInput(nn.as_mut());
            }
        }
        
        self.inputs.insert("input".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Request output stream
    pub fn request_output(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut nn) = self.inner {
            unsafe {
                // TODO: Implement output request
                // dai::node::NeuralNetwork_requestOutput(nn.as_mut());
            }
        }
        
        self.outputs.insert("out".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Request passthrough output (original input)
    pub fn request_passthrough(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut nn) = self.inner {
            unsafe {
                // TODO: Implement passthrough request
                // dai::node::NeuralNetwork_requestPassthrough(nn.as_mut());
            }
        }
        
        self.outputs.insert("passthrough".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Get neural network configuration
    pub fn config(&self) -> &NeuralNetworkConfig {
        &self.config
    }
    
    /// Get the underlying C++ neural network node (for advanced use)
    pub fn as_raw(&self) -> Option<&dai::node::NeuralNetwork> {
        self.inner.as_ref().map(|nn| nn.as_ref())
    }
}

impl PipelineNode for NeuralNetwork {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn node_type(&self) -> String {
        "NeuralNetwork".to_string()
    }
    
    fn configure(&mut self) -> DaiResult<()> {
        // Validate configuration
        if self.config.blob_path.is_empty() {
            return Err(DaiError::InvalidConfiguration(
                "Neural network blob path is required".to_string()
            ));
        }
        
        if !Path::new(&self.config.blob_path).exists() {
            return Err(DaiError::FileNotFound(
                format!("Blob file not found: {}", self.config.blob_path)
            ));
        }
        
        // Create the underlying C++ neural network node
        let nn = unsafe {
            let mut nn = Box::new(std::mem::zeroed::<dai::node::NeuralNetwork>());
            // TODO: Properly initialize neural network node
            // dai::node::NeuralNetwork_NeuralNetwork(nn.as_mut());
            nn
        };
        
        // Configure the neural network based on our config
        // TODO: Apply configuration to the C++ nn object
        
        self.inner = Some(nn);
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
    use std::fs;

    #[test]
    fn test_neural_network_creation() {
        let nn = NeuralNetwork::new("test_nn", "test.blob");
        assert_eq!(nn.id(), "test_nn");
        assert_eq!(nn.node_type(), "NeuralNetwork");
        assert_eq!(nn.config().blob_path, "test.blob");
    }
    
    #[test]
    fn test_neural_network_configuration() {
        let config = NeuralNetworkConfig {
            blob_path: "test.blob".to_string(),
            num_threads: Some(4),
            num_nce_per_thread: Some(2),
            input_size: Some((416, 416)),
            confidence_threshold: Some(0.5),
        };
        
        let nn = NeuralNetwork::with_config("test_nn", config);
        assert_eq!(nn.config().num_threads, Some(4));
        assert_eq!(nn.config().input_size, Some((416, 416)));
        assert_eq!(nn.config().confidence_threshold, Some(0.5));
    }
    
    #[test]
    fn test_blob_path_validation() {
        let mut nn = NeuralNetwork::new("test_nn", "nonexistent.blob");
        
        // This should fail because the file doesn't exist
        let result = nn.set_blob_path("nonexistent.blob");
        assert!(result.is_err());
        
        // Create a temporary blob file for testing
        let temp_path = "temp_test.blob";
        fs::write(temp_path, "test blob content").unwrap();
        
        // This should succeed
        let result = nn.set_blob_path(temp_path);
        assert!(result.is_ok());
        
        // Clean up
        fs::remove_file(temp_path).unwrap();
    }
}
