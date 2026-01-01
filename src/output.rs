use std::ffi::CString;
use std::sync::Arc;

use autocxx::c_uint;
use depthai_sys::{depthai, DaiOutput, DaiInput};

use crate::camera::{ImageFrame, OutputQueue};
use crate::encoded_frame::EncodedFrameQueue;
use crate::error::{clear_error_flag, last_error, Result};
use crate::host_node::Buffer;
use crate::pipeline::{Node, PipelineInner};
use crate::queue::{InputQueue, MessageQueue};

#[derive(Clone)]
pub struct Output {
    pub(crate) pipeline: Arc<PipelineInner>,
    pub(crate) handle: DaiOutput,
}

unsafe impl Send for Output {}
unsafe impl Sync for Output {}

#[derive(Clone)]
pub struct Input {
    pub(crate) pipeline: Arc<PipelineInner>,
    pub(crate) handle: DaiInput,
}

unsafe impl Send for Input {}
unsafe impl Sync for Input {}

impl Output {
    pub(crate) fn from_handle(pipeline: Arc<PipelineInner>, handle: DaiOutput) -> Self {
        Self { pipeline, handle }
    }

    pub fn link_to(&self, to: &Node, in_name: Option<&str>) -> Result<()> {
        clear_error_flag();
        let in_name_c = in_name
            .map(|s| CString::new(s).map_err(|_| last_error("invalid in_name")))
            .transpose()?;

        let ok = unsafe {
            depthai::dai_output_link(
                self.handle,
                to.handle(),
                std::ptr::null(),
                in_name_c
                    .as_ref()
                    .map(|s| s.as_ptr())
                    .unwrap_or(std::ptr::null()),
            )
        };

        if ok {
            Ok(())
        } else {
            Err(last_error("failed to link output"))
        }
    }

    pub fn link(&self, input: &Input) -> Result<()> {
        clear_error_flag();
        let ok = unsafe { depthai::dai_output_link_input(self.handle, input.handle) };
        if ok {
            Ok(())
        } else {
            Err(last_error("failed to link output to input"))
        }
    }

    pub fn create_queue(&self, max_size: u32, blocking: bool) -> Result<OutputQueue> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_output_create_queue(self.handle, c_uint(max_size), blocking) };
        if handle.is_null() {
            Err(last_error("failed to create output queue"))
        } else {
            Ok(OutputQueue::from_handle(handle))
        }
    }

    /// Create a generic output queue which yields messages as `Datatype`.
    ///
    /// This maps closely to DepthAI-Core's `MessageQueue`/`DataOutputQueue` API.
    pub fn create_message_queue(&self, max_size: u32, blocking: bool) -> Result<MessageQueue> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_output_create_queue(self.handle, c_uint(max_size), blocking) };
        if handle.is_null() {
            Err(last_error("failed to create message queue"))
        } else {
            Ok(MessageQueue::from_handle(handle))
        }
    }

    /// Create an output queue that yields `EncodedFrame` messages.
    ///
    /// This is primarily used with `VideoEncoderNode::out()`.
    pub fn create_encoded_frame_queue(&self, max_size: u32, blocking: bool) -> Result<EncodedFrameQueue> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_output_create_queue(self.handle, c_uint(max_size), blocking) };
        if handle.is_null() {
            Err(last_error("failed to create encoded frame output queue"))
        } else {
            Ok(EncodedFrameQueue::from_handle(handle))
        }
    }

    pub fn send_buffer(&self, buffer: &Buffer) -> Result<()> {
        clear_error_flag();
        unsafe { depthai::dai_output_send_buffer(self.handle, buffer.handle()) };
        if let Some(err) = crate::error::take_error_if_any("failed to send buffer") {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn send_frame(&self, frame: &ImageFrame) -> Result<()> {
        clear_error_flag();
        unsafe { depthai::dai_output_send_img_frame(self.handle, frame.handle()) };
        if let Some(err) = crate::error::take_error_if_any("failed to send frame") {
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl Input {
    pub(crate) fn from_handle(pipeline: Arc<PipelineInner>, handle: DaiInput) -> Self {
        Self { pipeline, handle }
    }

    pub fn get_buffer(&self) -> Result<Buffer> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_input_get_buffer(self.handle) };
        if handle.is_null() {
            Err(last_error("failed to get buffer from input"))
        } else {
            Ok(Buffer::from_handle(handle))
        }
    }

    pub fn try_get_buffer(&self) -> Result<Option<Buffer>> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_input_try_get_buffer(self.handle) };
        if handle.is_null() {
            if let Some(err) = crate::error::take_error_if_any("failed to poll buffer from input") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(Buffer::from_handle(handle)))
        }
    }

    pub fn get_frame(&self) -> Result<ImageFrame> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_input_get_img_frame(self.handle) };
        if handle.is_null() {
            Err(last_error("failed to get frame from input"))
        } else {
            Ok(ImageFrame::from_handle(handle))
        }
    }

    pub fn try_get_frame(&self) -> Result<Option<ImageFrame>> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_input_try_get_img_frame(self.handle) };
        if handle.is_null() {
            if let Some(err) = crate::error::take_error_if_any("failed to poll frame from input") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(ImageFrame::from_handle(handle)))
        }
    }

    /// Create a host→device input queue (DepthAI-Core `InputQueue`).
    ///
    /// This is the canonical way to send messages into a pipeline input from the host.
    pub fn create_input_queue(&self, max_size: u32, blocking: bool) -> Result<InputQueue> {
        clear_error_flag();
        let handle = unsafe { depthai::dai_input_create_input_queue(self.handle, c_uint(max_size), blocking) };
        if handle.is_null() {
            Err(last_error("failed to create input queue"))
        } else {
            Ok(InputQueue::from_handle(handle))
        }
    }
}

impl Node {
    pub fn output(&self, name: &str) -> Result<Output> {
        clear_error_flag();
        let name_c = CString::new(name).map_err(|_| last_error("invalid output name"))?;
        let handle = unsafe { depthai::dai_node_get_output(self.handle(), std::ptr::null(), name_c.as_ptr()) };
        if handle.is_null() {
            Err(last_error("failed to get node output"))
        } else {
            Ok(Output::from_handle(Arc::clone(&self.pipeline), handle))
        }
    }

    pub fn input(&self, name: &str) -> Result<Input> {
        clear_error_flag();
        let name_c = CString::new(name).map_err(|_| last_error("invalid input name"))?;
        let handle = unsafe { depthai::dai_node_get_input(self.handle(), std::ptr::null(), name_c.as_ptr()) };
        if handle.is_null() {
            Err(last_error("failed to get node input"))
        } else {
            Ok(Input::from_handle(Arc::clone(&self.pipeline), handle))
        }
    }
}
