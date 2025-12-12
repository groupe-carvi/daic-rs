use std::time::Duration;

use autocxx::{c_int, c_uint};
use daic_sys::{daic, DaiCameraNode, DaiOutput, DaiDataQueue, DaiImgFrame};

pub use crate::common::{CameraBoardSocket, ImageFrameType, ResizeMode};
use crate::error::{Result, clear_error_flag, last_error, take_error_if_any};

pub struct CameraNode {
    handle: DaiCameraNode,
}

pub struct CameraOutput {
    handle: DaiOutput,
}

pub struct OutputQueue {
    handle: DaiDataQueue,
}

pub struct ImageFrame {
    handle: DaiImgFrame,
}

#[derive(Debug, Clone)]
pub struct CameraOutputConfig {
    pub size: (u32, u32),
    pub frame_type: Option<ImageFrameType>,
    pub resize_mode: ResizeMode,
    pub fps: Option<f32>,
    pub enable_undistortion: Option<bool>,
}

impl Default for CameraOutputConfig {
    fn default() -> Self {
        Self {
            size: (640, 400),
            frame_type: None,
            resize_mode: ResizeMode::Crop,
            fps: None,
            enable_undistortion: None,
        }
    }
}

impl CameraOutputConfig {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }
}

impl CameraNode {
    pub(crate) fn from_handle(handle: DaiCameraNode) -> Self {
        Self { handle }
    }

    pub fn request_output(&self, config: CameraOutputConfig) -> Result<CameraOutput> {
        clear_error_flag();
        let fmt = config.frame_type.map(|t| t as i32).unwrap_or(-1);
        let resize = config.resize_mode as i32;
        let fps = config.fps.unwrap_or(-1.0);
        let undist = config
            .enable_undistortion
            .map(|v| if v { 1 } else { 0 })
            .unwrap_or(-1);
        let handle = unsafe {
            daic::dai_camera_request_output(
                self.handle,
                c_int(config.size.0 as i32),
                c_int(config.size.1 as i32),
                c_int(fmt),
                c_int(resize),
                fps,
                c_int(undist),
            )
        };
        if handle.is_null() {
            Err(last_error("failed to request camera output"))
        } else {
            Ok(CameraOutput {
                handle,
            })
        }
    }

    pub fn request_full_resolution_output(&self) -> Result<CameraOutput> {
        clear_error_flag();
        let handle = unsafe { daic::dai_camera_request_full_resolution_output(self.handle) };
        if handle.is_null() {
            Err(last_error("failed to request full resolution output"))
        } else {
            Ok(CameraOutput {
                handle,
            })
        }
    }
}

impl CameraOutput {
    pub fn create_queue(&self, max_size: u32, blocking: bool) -> Result<OutputQueue> {
        clear_error_flag();
        let handle =
            unsafe { daic::dai_output_create_queue(self.handle, c_uint(max_size), blocking) };
        if handle.is_null() {
            Err(last_error("failed to create output queue"))
        } else {
            Ok(OutputQueue { handle })
        }
    }
}

impl Drop for OutputQueue {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_queue_delete(self.handle) };
        }
    }
}

impl OutputQueue {
    pub fn blocking_next(&self, timeout: Option<Duration>) -> Result<Option<ImageFrame>> {
        clear_error_flag();
        let timeout_ms = timeout.map(|d| d.as_millis() as i32).unwrap_or(-1);
        let frame = unsafe { daic::dai_queue_get_frame(self.handle, c_int(timeout_ms)) };
        if frame.is_null() {
            if let Some(err) = take_error_if_any("failed to pull frame") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(ImageFrame { handle: frame }))
        }
    }

    pub fn try_next(&self) -> Result<Option<ImageFrame>> {
        clear_error_flag();
        let frame = unsafe { daic::dai_queue_try_get_frame(self.handle) };
        if frame.is_null() {
            if let Some(err) = take_error_if_any("failed to poll frame") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(ImageFrame { handle: frame }))
        }
    }
}

impl Drop for ImageFrame {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_frame_release(self.handle) };
        }
    }
}

impl ImageFrame {
    pub fn width(&self) -> u32 {
        let raw: ::std::os::raw::c_int = unsafe { daic::dai_frame_get_width(self.handle) }.into();
        raw as u32
    }

    pub fn height(&self) -> u32 {
        let raw: ::std::os::raw::c_int = unsafe { daic::dai_frame_get_height(self.handle) }.into();
        raw as u32
    }

    pub fn format(&self) -> Option<ImageFrameType> {
        let raw: ::std::os::raw::c_int = unsafe { daic::dai_frame_get_type(self.handle) }.into();
        ImageFrameType::from_raw(raw)
    }

    pub fn byte_len(&self) -> usize {
        let raw: usize = unsafe { daic::dai_frame_get_size(self.handle) }.into();
        raw
    }

    pub fn bytes(&self) -> Vec<u8> {
        let len = self.byte_len();
        if len == 0 {
            return Vec::new();
        }
        let data_ptr = unsafe { daic::dai_frame_get_data(self.handle) };
        if data_ptr.is_null() {
            return Vec::new();
        }
        unsafe { std::slice::from_raw_parts(data_ptr as *const u8, len).to_vec() }
    }

    pub fn describe(&self) -> String {
        let fmt = self
            .format()
            .map(|f| format!("{f:?}"))
            .unwrap_or_else(|| "unknown".into());
        format!("{}x{} {}", self.width(), self.height(), fmt)
    }
}
