use crate::error::{DaiError, DaiResult};
use crate::pipeline::PipelineNode;
use daic_sys::root::dai::node::Camera as DaiCamera;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraBoardSocket {
    Rgb,
    Left, 
    Right,
}

impl CameraBoardSocket {
    pub fn as_u32(&self) -> u32 {
        match self {
            CameraBoardSocket::Rgb => 0,
            CameraBoardSocket::Left => 1,
            CameraBoardSocket::Right => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CameraResolution {
    #[default]
    The1080P,
    The4K,
    The720P,
    The800P,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorOrder {
    BGR,
    RGB,
}

#[derive(Debug, Clone, Default)]
pub struct CameraConfig {
    pub board_socket: Option<CameraBoardSocket>,
    pub resolution: CameraResolution,
    pub fps: f32,
    pub preview_size: Option<(u32, u32)>,
    pub color_order: Option<ColorOrder>,
}

/// Camera node wrapper
pub struct Camera {
    pub id: String,
    pub config: CameraConfig,
    pub(crate) inner: Option<DaiCamera>,
}

impl Camera {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            config: CameraConfig::default(),
            inner: None,
        }
    }

    pub fn with_config(id: impl Into<String>, config: CameraConfig) -> Self {
        Self {
            id: id.into(),
            config,
            inner: None,
        }
    }

    pub fn set_board_socket(&mut self, socket: CameraBoardSocket) -> &mut Self {
        self.config.board_socket = Some(socket);
        self
    }

    pub fn set_resolution(&mut self, resolution: CameraResolution) -> &mut Self {
        self.config.resolution = resolution;
        self
    }

    pub fn set_fps(&mut self, fps: f32) -> &mut Self {
        self.config.fps = fps;
        self
    }
}

impl PipelineNode for Camera {
    fn id(&self) -> String {
        self.id.clone()
    }
    
    fn node_type(&self) -> String {
        "Camera".to_string()
    }
    
    fn configure(&mut self) -> DaiResult<()> {
        if self.inner.is_some() {
            return Err(DaiError::AlreadyInitialized);
        }

        // Créer le nœud Camera en utilisant les bindings générés
        unsafe {
            let camera = std::mem::zeroed::<DaiCamera>();
            self.inner = Some(camera);
        }

        Ok(())
    }
    
    fn inputs(&self) -> Vec<String> {
        vec!["inputControl".to_string()]
    }
    
    fn outputs(&self) -> Vec<String> {
        vec!["video".to_string(), "still".to_string(), "isp".to_string()]
    }
}
