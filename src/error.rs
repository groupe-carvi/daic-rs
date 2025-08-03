//! Error handling for DepthAI Rust bindings

use std::fmt;

/// Result type for DepthAI operations
pub type DaiResult<T> = Result<T, DaiError>;

/// Errors that can occur when working with DepthAI
#[derive(Debug, Clone)]
pub enum DaiError {
    /// Pipeline creation failed
    PipelineCreationFailed(String),
    /// Device creation failed
    DeviceCreationFailed(String),
    /// Pipeline start failed
    PipelineStartFailed(String),
    /// Pipeline stop failed
    PipelineStopFailed(String),
    /// Invalid handle provided
    InvalidHandle(String),
    /// Device connection error
    DeviceConnectionError(String),
    /// Node already initialized
    AlreadyInitialized,
    /// FFI error occurred
    FfiError(String),
    /// File not found error
    FileNotFound(String),
    /// Invalid configuration error
    InvalidConfiguration(String),
    /// Generic DepthAI error
    Other(String),
}

impl DaiError {
    /// Create a new FFI error from the last C++ error
    pub fn from_ffi() -> Self {
        unsafe {
            let error_ptr = daic_sys::root::daic::dai_get_last_error();
            if error_ptr.is_null() {
                DaiError::FfiError("Unknown FFI error".to_string())
            } else {
                let error_cstr = std::ffi::CStr::from_ptr(error_ptr);
                let error_msg = error_cstr.to_string_lossy().into_owned();
                DaiError::FfiError(error_msg)
            }
        }
    }
}

impl fmt::Display for DaiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaiError::PipelineCreationFailed(msg) => write!(f, "Pipeline creation failed: {}", msg),
            DaiError::DeviceCreationFailed(msg) => write!(f, "Device creation failed: {}", msg),
            DaiError::PipelineStartFailed(msg) => write!(f, "Pipeline start failed: {}", msg),
            DaiError::PipelineStopFailed(msg) => write!(f, "Pipeline stop failed: {}", msg),
            DaiError::InvalidHandle(msg) => write!(f, "Invalid handle: {}", msg),
            DaiError::DeviceConnectionError(msg) => write!(f, "Device connection error: {}", msg),
            DaiError::AlreadyInitialized => write!(f, "Node already initialized"),
            DaiError::FfiError(msg) => write!(f, "FFI error: {}", msg),
            DaiError::FileNotFound(msg) => write!(f, "File not found: {}", msg),
            DaiError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
            DaiError::Other(msg) => write!(f, "DepthAI error: {}", msg),
        }
    }
}

impl std::error::Error for DaiError {}

impl From<std::ffi::NulError> for DaiError {
    fn from(err: std::ffi::NulError) -> Self {
        DaiError::FfiError(format!("Null character in string: {}", err))
    }
}
