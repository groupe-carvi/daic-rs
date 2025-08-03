use crate::error::{DaiError, DaiResult};
use crate::pipeline::PipelineNode;
use daic_sys::root::dai::Node_Input as DaiXLinkIn;

#[derive(Debug, Clone)]
pub struct XLinkInConfig {
    pub stream_name: String,
    pub max_data_size: Option<usize>,
    pub num_frames: Option<u8>,
}

impl Default for XLinkInConfig {
    fn default() -> Self {
        Self {
            stream_name: "input".to_string(),
            max_data_size: None,
            num_frames: Some(8),
        }
    }
}

/// XLink input node for streaming data from host to device
pub struct XLinkIn {
    pub id: String,
    pub config: XLinkInConfig,
    pub(crate) inner: Option<DaiXLinkIn>,
}

impl XLinkIn {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            config: XLinkInConfig::default(),
            inner: None,
        }
    }

    pub fn with_config(id: impl Into<String>, config: XLinkInConfig) -> Self {
        Self {
            id: id.into(),
            config,
            inner: None,
        }
    }

    pub fn set_stream_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.config.stream_name = name.into();
        self
    }

    pub fn set_max_data_size(&mut self, size: usize) -> &mut Self {
        self.config.max_data_size = Some(size);
        self
    }

    pub fn set_num_frames(&mut self, frames: u8) -> &mut Self {
        self.config.num_frames = Some(frames);
        self
    }
}

impl PipelineNode for XLinkIn {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn node_type(&self) -> String {
        "XLinkIn".to_string()
    }
    
    fn configure(&mut self) -> DaiResult<()> {
        if self.inner.is_some() {
            return Err(DaiError::AlreadyInitialized);
        }

        // Créer le nœud XLinkIn en utilisant les bindings générés
        unsafe {
            let xlink = std::mem::zeroed::<DaiXLinkIn>();
            self.inner = Some(xlink);
        }

        Ok(())
    }
    
    fn inputs(&self) -> Vec<String> {
        vec![]
    }
    
    fn outputs(&self) -> Vec<String> {
        vec!["out".to_string()]
    }
}
