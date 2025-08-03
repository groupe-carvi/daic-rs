//! Core C bindings for DepthAI
//! 
//! This module provides the primary C API bindings for DepthAI functionality

use std::os::raw::c_char;

/// Opaque handle to a DepthAI device instance
pub type DeviceHandle = *mut std::ffi::c_void;

/// Opaque handle to a DepthAI pipeline instance
pub type PipelineHandle = *mut std::ffi::c_void;

/// Opaque handle to a DepthAI camera instance
pub type CameraHandle = *mut std::ffi::c_void;

/// Camera board socket enumeration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum CameraBoardSocket {
    CAM_A = 0,
    CAM_B = 1,
    CAM_C = 2,
    CAM_D = 3,
}

/// Camera resolution enumeration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum CameraResolution {
    THE_480_P = 0,
    THE_720_P = 1,
    THE_800_P = 2,
    THE_1080_P = 3,
    THE_1200_P = 4,
    THE_12_MP = 5,
    THE_13_MP = 6,
}

#[link(name = "depthai-core")]
unsafe extern "C" {
    // Device management functions
    /// Create a new device instance
    pub fn device_create() -> DeviceHandle;
    
    /// Destroy a device instance
    pub fn device_destroy(device: DeviceHandle);
    
    /// Check if device is connected
    pub fn device_is_connected(device: DeviceHandle) -> bool;
    
    // Pipeline management functions
    /// Create a new pipeline instance
    pub fn pipeline_create() -> PipelineHandle;
    
    /// Destroy a pipeline instance
    pub fn pipeline_destroy(pipeline: PipelineHandle);
    
    /// Start pipeline execution on device
    pub fn pipeline_start(pipeline: PipelineHandle, device: DeviceHandle) -> bool;
    
    /// Stop pipeline execution
    pub fn pipeline_stop(pipeline: PipelineHandle) -> bool;
    
    /// Check if pipeline is running
    pub fn pipeline_is_running(pipeline: PipelineHandle) -> bool;
    
    // Camera management functions
    /// Create a new camera instance
    pub fn camera_create() -> CameraHandle;
    
    /// Destroy a camera instance
    pub fn camera_destroy(camera: CameraHandle);
    
    /// Set the camera board socket
    pub fn camera_set_board_socket(camera: CameraHandle, socket: CameraBoardSocket) -> bool;
    
    /// Set the camera resolution
    pub fn camera_set_resolution(camera: CameraHandle, resolution: CameraResolution) -> bool;
    
    /// Set the camera FPS
    pub fn camera_set_fps(camera: CameraHandle, fps: f32) -> bool;
    
    /// Request preview output from camera
    pub fn camera_request_preview(camera: CameraHandle) -> bool;
    
    /// Request video output from camera  
    pub fn camera_request_video(camera: CameraHandle) -> bool;
    
    /// Request still output from camera
    pub fn camera_request_still(camera: CameraHandle) -> bool;
    
    // Error handling functions
    /// Get the last error message
    pub fn dai_get_last_error() -> *const c_char;
    
    /// Clear the last error
    pub fn dai_clear_last_error();
}
