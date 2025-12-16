use std::time::Duration;

use autocxx::c_int;
use daic_sys::{daic, DaiRGBDData};

use crate::camera::{ImageFrame, OutputQueue};
use crate::error::{clear_error_flag, last_error, take_error_if_any, Result};
use crate::pipeline::{DeviceNode, NodeKind, Pipeline};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthUnit {
    Meter = 0,
    Centimeter = 1,
    Millimeter = 2,
    Inch = 3,
    Foot = 4,
    Custom = 5,
}

pub struct RgbdNode {
    node: crate::pipeline::Node,
}

impl RgbdNode {
    pub fn as_node(&self) -> &crate::pipeline::Node {
        &self.node
    }

    pub fn build(&self) -> Result<()> {
        clear_error_flag();
        let out = unsafe { daic::dai_rgbd_build(self.node.handle()) };
        if out.is_null() {
            Err(last_error("failed to build RGBD node"))
        } else {
            Ok(())
        }
    }

    pub fn set_depth_unit(&self, unit: DepthUnit) {
        // setter cannot fail at the C ABI level (will record last_error on exception)
        clear_error_flag();
        unsafe { daic::dai_rgbd_set_depth_unit(self.node.handle(), c_int(unit as i32)) };
    }
}

unsafe impl DeviceNode for RgbdNode {
    fn create_in_pipeline(pipeline: &Pipeline) -> Result<Self> {
        let node = pipeline.create_node(NodeKind::Rgbd)?;
        Ok(Self { node })
    }
}

pub struct RgbdData {
    handle: DaiRGBDData,
}

impl Drop for RgbdData {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_rgbd_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl RgbdData {
    pub(crate) fn from_handle(handle: DaiRGBDData) -> Self {
        Self { handle }
    }

    pub fn rgb_frame(&self) -> Result<ImageFrame> {
        clear_error_flag();
        let frame = unsafe { daic::dai_rgbd_get_rgb_frame(self.handle) };
        if frame.is_null() {
            Err(last_error("failed to get RGB frame"))
        } else {
            Ok(ImageFrame::from_handle(frame))
        }
    }

    pub fn depth_frame(&self) -> Result<ImageFrame> {
        clear_error_flag();
        let frame = unsafe { daic::dai_rgbd_get_depth_frame(self.handle) };
        if frame.is_null() {
            Err(last_error("failed to get depth frame"))
        } else {
            Ok(ImageFrame::from_handle(frame))
        }
    }
}

impl OutputQueue {
    pub fn blocking_next_rgbd(&self, timeout: Option<Duration>) -> Result<Option<RgbdData>> {
        clear_error_flag();
        let timeout_ms = timeout.map(|d| d.as_millis() as i32).unwrap_or(-1);
        let msg = unsafe { daic::dai_queue_get_rgbd(self.handle(), c_int(timeout_ms)) };
        if msg.is_null() {
            if let Some(err) = take_error_if_any("failed to pull rgbd") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(RgbdData::from_handle(msg)))
        }
    }

    pub fn try_next_rgbd(&self) -> Result<Option<RgbdData>> {
        clear_error_flag();
        let msg = unsafe { daic::dai_queue_try_get_rgbd(self.handle()) };
        if msg.is_null() {
            if let Some(err) = take_error_if_any("failed to poll rgbd") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(RgbdData::from_handle(msg)))
        }
    }
}
