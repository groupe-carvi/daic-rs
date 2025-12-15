use std::ffi::CString;
use std::sync::Arc;

use autocxx::c_int;
use daic_sys::{daic, DaiNode};

use crate::error::{clear_error_flag, last_error, Result};

use super::PipelineInner;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Camera = 1,
    StereoDepth = 2,
    ImageAlign = 3,
    Rgbd = 4,
}

#[derive(Clone)]
pub struct Node {
    pub(crate) pipeline: Arc<PipelineInner>,
    pub(crate) handle: DaiNode,
}

impl Node {
    pub(crate) fn from_handle(pipeline: Arc<PipelineInner>, handle: DaiNode) -> Self {
        Self { pipeline, handle }
    }

    pub fn handle(&self) -> DaiNode {
        self.handle
    }

    pub fn link(
        &self,
        out_group: Option<&str>,
        out_name: Option<&str>,
        to: &Node,
        in_group: Option<&str>,
        in_name: Option<&str>,
    ) -> Result<()> {
        clear_error_flag();

        let out_name_c = out_name
            .map(|s| CString::new(s).map_err(|_| last_error("invalid out_name")))
            .transpose()?;
        let in_name_c = in_name
            .map(|s| CString::new(s).map_err(|_| last_error("invalid in_name")))
            .transpose()?;

        let out_group_c = out_group
            .map(|s| CString::new(s).map_err(|_| last_error("invalid out_group")))
            .transpose()?;
        let in_group_c = in_group
            .map(|s| CString::new(s).map_err(|_| last_error("invalid in_group")))
            .transpose()?;

        let ok = unsafe {
            daic::dai_node_link(
                self.handle,
                out_group_c
                    .as_ref()
                    .map(|s| s.as_ptr())
                    .unwrap_or(std::ptr::null()),
                out_name_c
                    .as_ref()
                    .map(|s| s.as_ptr())
                    .unwrap_or(std::ptr::null()),
                to.handle,
                in_group_c
                    .as_ref()
                    .map(|s| s.as_ptr())
                    .unwrap_or(std::ptr::null()),
                in_name_c
                    .as_ref()
                    .map(|s| s.as_ptr())
                    .unwrap_or(std::ptr::null()),
            )
        };

        if ok {
            Ok(())
        } else {
            Err(last_error("failed to link nodes"))
        }
    }
}

pub(crate) fn create_node(pipeline: Arc<PipelineInner>, kind: NodeKind) -> Result<Node> {
    clear_error_flag();
    let handle = unsafe {
        daic::dai_pipeline_create_node(pipeline.handle, c_int(kind as i32))
    };
    if handle.is_null() {
        Err(last_error("failed to create node"))
    } else {
        Ok(Node::from_handle(pipeline, handle))
    }
}
