pub mod device_node;
pub mod node;

use autocxx::c_int;
use daic_sys::{daic, DaiPipeline};
pub use device_node::{CreateInPipeline, CreateInPipelineWith, DeviceNode, DeviceNodeWithParams};
pub use node::{Node, NodeKind};

use std::sync::Arc;

use crate::{
    camera::{CameraBoardSocket, CameraNode},
    device::Device,
    error::{clear_error_flag, last_error, Result},
};

pub(crate) struct PipelineInner {
    handle: DaiPipeline,
}

impl Drop for PipelineInner {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_pipeline_delete(self.handle) };
        }
    }
}

#[derive(Clone)]
pub struct Pipeline {
    inner: Arc<PipelineInner>,
}

impl Pipeline {
    pub fn new() -> Result<Self> {
        clear_error_flag();
        let handle = daic::dai_pipeline_new();
        if handle.is_null() {
            Err(last_error("failed to create pipeline"))
        } else {
            Ok(Self {
                inner: Arc::new(PipelineInner { handle }),
            })
        }
    }

    /// Create a pipeline that is explicitly bound to an existing device connection.
    ///
    /// This matches the DepthAI C++ pattern:
    /// `auto device = std::make_shared<dai::Device>(); dai::Pipeline pipeline(device);`
    pub fn with_device(device: &Device) -> Result<Self> {
        clear_error_flag();
        let handle = unsafe { daic::dai_pipeline_new_with_device(device.handle()) };
        if handle.is_null() {
            Err(last_error("failed to create pipeline with device"))
        } else {
            Ok(Self {
                inner: Arc::new(PipelineInner { handle }),
            })
        }
    }

    /// Get the pipeline's default device handle (shared).
    ///
    /// Use this to avoid accidentally opening a second device connection when the pipeline
    /// was created with an implicit/default device.
    pub fn default_device(&self) -> Result<Device> {
        clear_error_flag();
        let handle = unsafe { daic::dai_pipeline_get_default_device(self.inner.handle) };
        if handle.is_null() {
            Err(last_error("failed to get pipeline default device"))
        } else {
            Ok(Device::from_handle(handle))
        }
    }

    /// Generic method to create device nodes of any type implementing DeviceNode
    /// 
    /// # Example
    /// ```ignore
    /// let pipeline = Pipeline::new()?;
    /// let camera = pipeline.create::<CameraNode>()?;
    /// let stereo = pipeline.create::<StereoDepthNode>()?;
    /// ```
    pub fn create<T: CreateInPipeline>(&self) -> Result<T> {
        T::create(self)
    }

    /// Generic method to create device nodes that require parameters
    /// 
    /// # Example
    /// ```ignore
    /// let pipeline = Pipeline::new()?;
    /// let camera = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    /// ```
    pub fn create_with<T: CreateInPipelineWith<P>, P>(&self, params: P) -> Result<T> {
        T::create_with(self, params)
    }

    pub fn create_camera(&self, socket: CameraBoardSocket) -> Result<CameraNode> {
        clear_error_flag();
        let handle =
            unsafe { daic::dai_pipeline_create_camera(self.inner.handle, c_int(socket.as_raw())) };
        if handle.is_null() {
            Err(last_error("failed to create camera node"))
        } else {
            Ok(CameraNode::from_handle(self.inner_arc(), handle))
        }
    }

    pub fn create_node(&self, kind: NodeKind) -> Result<Node> {
        node::create_node(self.inner_arc(), kind)
    }

    pub fn start_with_device(&self, device: &Device) -> Result<()> {
        clear_error_flag();
        let started = unsafe { daic::dai_pipeline_start(self.inner.handle, device.handle()) };
        if started {
            Ok(())
        } else {
            Err(last_error("failed to start pipeline"))
        }
    }

    /// Start the pipeline using its internally-held default device.
    pub fn start_default(&self) -> Result<()> {
        clear_error_flag();
        let started = unsafe { daic::dai_pipeline_start_default(self.inner.handle) };
        if started {
            Ok(())
        } else {
            Err(last_error("failed to start pipeline"))
        }
    }

    pub(crate) fn handle(&self) -> DaiPipeline {
        self.inner.handle
    }

    pub(crate) fn inner_arc(&self) -> Arc<PipelineInner> {
        Arc::clone(&self.inner)
    }
}

unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}
