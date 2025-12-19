use std::ffi::CString;
use std::sync::Arc;

use autocxx::c_uint;
use depthai_sys::{depthai, DaiOutput, DaiInput};

use crate::camera::OutputQueue;
use crate::error::{clear_error_flag, last_error, Result};
use crate::pipeline::{Node, PipelineInner};

#[derive(Clone)]
pub struct Output {
    pub(crate) pipeline: Arc<PipelineInner>,
    pub(crate) handle: DaiOutput,
}

#[derive(Clone)]
pub struct Input {
    pub(crate) pipeline: Arc<PipelineInner>,
    pub(crate) handle: DaiInput,
}

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
}

impl Input {
    pub(crate) fn from_handle(pipeline: Arc<PipelineInner>, handle: DaiInput) -> Self {
        Self { pipeline, handle }
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
