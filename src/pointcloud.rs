use std::time::Duration;

use autocxx::c_int;
use daic_sys::{daic, DaiPointCloud};

use crate::camera::OutputQueue;
use crate::error::{clear_error_flag, take_error_if_any, Result};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Point3fRGBA {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct PointCloudData {
    handle: DaiPointCloud,
}

impl Drop for PointCloudData {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { daic::dai_pointcloud_release(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl PointCloudData {
    pub(crate) fn from_handle(handle: DaiPointCloud) -> Self {
        Self { handle }
    }

    pub fn width(&self) -> u32 {
        let raw: ::std::os::raw::c_int = unsafe { daic::dai_pointcloud_get_width(self.handle) }.into();
        raw.max(0) as u32
    }

    pub fn height(&self) -> u32 {
        let raw: ::std::os::raw::c_int = unsafe { daic::dai_pointcloud_get_height(self.handle) }.into();
        raw.max(0) as u32
    }

    pub fn points(&self) -> &[Point3fRGBA] {
        let len: usize = unsafe { daic::dai_pointcloud_get_points_rgba_len(self.handle) }.into();
        if len == 0 {
            return &[];
        }
        let ptr = unsafe { daic::dai_pointcloud_get_points_rgba(self.handle) };
        if ptr.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(ptr as *const Point3fRGBA, len) }
    }
}

impl OutputQueue {
    pub fn blocking_next_pointcloud(&self, timeout: Option<Duration>) -> Result<Option<PointCloudData>> {
        clear_error_flag();
        let timeout_ms = timeout.map(|d| d.as_millis() as i32).unwrap_or(-1);
        let pcl = unsafe { daic::dai_queue_get_pointcloud(self.handle(), c_int(timeout_ms)) };
        if pcl.is_null() {
            if let Some(err) = take_error_if_any("failed to pull pointcloud") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(PointCloudData::from_handle(pcl)))
        }
    }

    pub fn try_next_pointcloud(&self) -> Result<Option<PointCloudData>> {
        clear_error_flag();
        let pcl = unsafe { daic::dai_queue_try_get_pointcloud(self.handle()) };
        if pcl.is_null() {
            if let Some(err) = take_error_if_any("failed to poll pointcloud") {
                Err(err)
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(PointCloudData::from_handle(pcl)))
        }
    }
}

/// Helper for constructing rerun-compatible RGBA colors.
pub fn rgba32_from_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}
