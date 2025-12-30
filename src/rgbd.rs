use std::time::Duration;

use autocxx::c_int;
use depthai_sys::{depthai, DaiRGBDData};

use crate::camera::{ImageFrame, OutputQueue};
use crate::error::{clear_error_flag, last_error, take_error_if_any, Result};

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

#[allow(non_snake_case)]
#[crate::native_node_wrapper(native = "dai::node::RGBD", inputs(inColor, inDepth), outputs(out))]
pub struct RgbdNode {
    node: crate::pipeline::Node,
}

impl RgbdNode {
    pub fn build(&self) -> Result<()> {
        clear_error_flag();
        let out = unsafe { depthai::dai_rgbd_build(self.node.handle()) };
        if out.is_null() {
            Err(last_error("failed to build RGBD node"))
        } else {
            Ok(())
        }
    }

    /// Build the RGBD node with explicit configuration.
    ///
    /// This maps to the native overload:
    /// `dai::node::RGBD::build(autocreate, mode, size, fps)`.
    ///
    /// Some devices/firmwares produce depth frames at sizes that don't match the requested
    /// RGB frames. Using this builder allows the host node to be configured with the
    /// expected size (and optionally FPS), preventing runtime "Color and depth frame sizes do not match" errors.
    pub fn build_ex(
        &self,
        autocreate: bool,
        mode: crate::stereo_depth::PresetMode,
        size: (i32, i32),
        fps: Option<f32>,
    ) -> Result<()> {
        clear_error_flag();
        let (w, h) = size;
        let fps_val = fps.unwrap_or(-1.0);
        let out = unsafe {
            depthai::dai_rgbd_build_ex(
                self.node.handle(),
                autocreate,
                c_int(mode as i32),
                c_int(w),
                c_int(h),
                fps_val,
            )
        };
        if out.is_null() {
            Err(last_error("failed to build RGBD node (build_ex)"))
        } else {
            Ok(())
        }
    }

    pub fn set_depth_unit(&self, unit: DepthUnit) {
        // setter cannot fail at the C ABI level (will record last_error on exception)
        clear_error_flag();
        unsafe { depthai::dai_rgbd_set_depth_unit(self.node.handle(), c_int(unit as i32)) };
    }
}

pub struct RgbdData {
    handle: DaiRGBDData,
}

impl Drop for RgbdData {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { depthai::dai_rgbd_release(self.handle) };
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
        let frame = unsafe { depthai::dai_rgbd_get_rgb_frame(self.handle) };
        if frame.is_null() {
            Err(last_error("failed to get RGB frame"))
        } else {
            Ok(ImageFrame::from_handle(frame))
        }
    }

    pub fn depth_frame(&self) -> Result<ImageFrame> {
        clear_error_flag();
        let frame = unsafe { depthai::dai_rgbd_get_depth_frame(self.handle) };
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
        let msg = unsafe { depthai::dai_queue_get_rgbd(self.handle(), c_int(timeout_ms)) };
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
        let msg = unsafe { depthai::dai_queue_try_get_rgbd(self.handle()) };
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
