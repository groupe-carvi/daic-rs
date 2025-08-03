//! Camera-specific C bindings for DepthAI
//! 
//! This module provides the C API bindings for camera functionality

use std::os::raw::{c_char, c_int, c_float};

/// Opaque handle to a DepthAI camera instance
pub type CameraHandle = *mut std::ffi::c_void;

/// Camera board socket enumeration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraBoardSocket {
    CAM_A = 0,
    CAM_B = 1,
    CAM_C = 2,
    CAM_D = 3,
}

/// Camera resolution enumeration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraResolution {
    THE_480_P = 0,
    THE_720_P = 1,
    THE_800_P = 2,
    THE_1080_P = 3,
    THE_1200_P = 4,
    THE_12_MP = 5,
    THE_13_MP = 6,
}

extern "C" {
    /// Create a new camera instance
    pub fn camera_create() -> CameraHandle;
    
    /// Destroy a camera instance
    pub fn camera_destroy(camera: CameraHandle);
    
    /// Set the camera board socket
    pub fn camera_set_board_socket(camera: CameraHandle, socket: CameraBoardSocket) -> bool;
    
    /// Set the camera resolution
    pub fn camera_set_resolution(camera: CameraHandle, resolution: CameraResolution) -> bool;
    
    /// Set the camera FPS
    pub fn camera_set_fps(camera: CameraHandle, fps: c_float) -> bool;
    
    /// Request preview output from camera
    pub fn camera_request_preview(camera: CameraHandle) -> bool;
    
    /// Request video output from camera  
    pub fn camera_request_video(camera: CameraHandle) -> bool;
    
    /// Request still output from camera
    pub fn camera_request_still(camera: CameraHandle) -> bool;
}
