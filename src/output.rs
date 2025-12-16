use std::ffi::CString;
use std::sync::Arc;

use autocxx::c_uint;
use daic_sys::{daic, DaiOutput};

use crate::camera::OutputQueue;
use crate::error::{clear_error_flag, last_error, Result};
use crate::pipeline::{Node, PipelineInner};

#[derive(Clone)]
pub struct Output {
    pub(crate) pipeline: Arc<PipelineInner>,
    pub(crate) handle: DaiOutput,
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
            daic::dai_output_link(
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

    pub fn create_queue(&self, max_size: u32, blocking: bool) -> Result<OutputQueue> {
        clear_error_flag();
        let handle = unsafe { daic::dai_output_create_queue(self.handle, c_uint(max_size), blocking) };
        if handle.is_null() {
            Err(last_error("failed to create output queue"))
        } else {
            Ok(OutputQueue::from_handle(handle))
        }
    }
}

impl Node {
    pub fn output(&self, name: &str) -> Result<Output> {
        clear_error_flag();
        let name_c = CString::new(name).map_err(|_| last_error("invalid output name"))?;
        let handle = unsafe { daic::dai_node_get_output(self.handle(), std::ptr::null(), name_c.as_ptr()) };
        if handle.is_null() {
            Err(last_error("failed to get node output"))
        } else {
            Ok(Output::from_handle(Arc::clone(&self.pipeline), handle))
        }
    }
}
