//! Output nodes for data streaming
//!
//! Provides safe wrapper around DepthAI output nodes.

use crate::pipeline::PipelineNode;
use crate::error::{DaiError, DaiResult};
use daic_sys::root::dai::Node_Output as DaiXLinkOut;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct XLinkOutConfig {
    pub stream_name: String,
    pub metadata_only: bool,
}

impl Default for XLinkOutConfig {
    fn default() -> Self {
        Self {
            stream_name: "output".to_string(),
            metadata_only: false,
        }
    }
}

/// XLink output node for streaming data from device to host
pub struct XLinkOut {
    id: String,
    config: XLinkOutConfig,
    inner: Option<Box<DaiXLinkOut>>,
    inputs: HashMap<String, String>,
}

impl XLinkOut {
    /// Create a new XLink output node
    pub fn new(id: impl Into<String>, stream_name: impl Into<String>) -> Self {
        let config = XLinkOutConfig {
            stream_name: stream_name.into(),
            ..Default::default()
        };
        Self::with_config(id, config)
    }
    
    /// Create a new XLink output node with custom configuration
    pub fn with_config(id: impl Into<String>, config: XLinkOutConfig) -> Self {
        Self {
            id: id.into(),
            config,
            inner: None,
            inputs: HashMap::new(),
        }
    }
    
    /// Set the stream name
    pub fn set_stream_name(&mut self, name: impl Into<String>) {
        self.config.stream_name = name.into();
        
        if let Some(ref mut xlink) = self.inner {
            unsafe {
                // TODO: Implement stream name setting
                // dai::node::XLinkOut_setStreamName(xlink.as_mut(), self.config.stream_name.as_ptr());
            }
        }
    }
    
    
    /// Set metadata only mode
    pub fn set_metadata_only(&mut self, metadata_only: bool) {
        self.config.metadata_only = metadata_only;
        
        if let Some(ref mut xlink) = self.inner {
            unsafe {
                // TODO: Implement metadata only setting
                // dai::node::XLinkOut_setMetadataOnly(xlink.as_mut(), metadata_only);
            }
        }
    }
    
    /// Request input stream
    pub fn request_input(&mut self, stream_name: impl Into<String>) -> DaiResult<String> {
        let stream_name = stream_name.into();
        
        if let Some(ref mut _xlink) = self.inner {
            unsafe {
                // TODO: Implement input request
                // dai::node::XLinkOut_requestInput(xlink.as_mut());
            }
        }
        
        self.inputs.insert("input".to_string(), stream_name.clone());
        Ok(stream_name)
    }
    
    /// Get output configuration
    pub fn config(&self) -> &XLinkOutConfig {
        &self.config
    }
    
    /// Get the stream name
    pub fn stream_name(&self) -> &str {
        &self.config.stream_name
    }
    
    /// Get the underlying C++ XLink output node (for advanced use)
    pub fn as_raw(&self) -> Option<&DaiXLinkOut> {
        self.inner.as_ref().map(|x| x.as_ref())
    }
}

impl PipelineNode for XLinkOut {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn node_type(&self) -> String {
        "XLinkOut".to_string()
    }
    
    fn configure(&mut self) -> DaiResult<()> {
        // Validate configuration
        if self.config.stream_name.is_empty() {
            return Err(DaiError::InvalidConfiguration(
                "XLinkOut stream name is required".to_string()
            ));
        }
        
        // Create the underlying C++ XLink output node
        let xlink = unsafe {
            let mut xlink = Box::new(std::mem::zeroed::<DaiXLinkOut>());
            // TODO: Properly initialize XLink output node
            // dai::node::XLinkOut_XLinkOut(xlink.as_mut());
            xlink
        };
        
        // Configure the XLink output node based on our config
        // TODO: Apply configuration to the C++ xlink object
        
        self.inner = Some(xlink);
        Ok(())
    }
    
    fn inputs(&self) -> Vec<String> {
        self.inputs.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xlink_out_creation() {
        let xlink = XLinkOut::new("test_xlink", "video_stream");
        assert_eq!(xlink.id(), "test_xlink");
        assert_eq!(xlink.node_type(), "XLinkOut");
        assert_eq!(xlink.stream_name(), "video_stream");
    }
    
    #[test]
    fn test_xlink_out_configuration() {
        let config = XLinkOutConfig {
            stream_name: "custom_stream".to_string(),
            metadata_only: true,
        };
        
        let xlink = XLinkOut::with_config("test_xlink", config);
        assert_eq!(xlink.config().stream_name, "custom_stream");
        assert_eq!(xlink.config().metadata_only, true);
    }
    
    #[test]
    fn test_empty_stream_name_validation() {
        let config = XLinkOutConfig {
            stream_name: String::new(),
            ..Default::default()
        };
        
        let mut xlink = XLinkOut::with_config("test_xlink", config);
        let result = xlink.configure();
        assert!(result.is_err());
    }
}
