//! Safe Rust API for DepthAI Device

use crate::bindings::root::dai;
use crate::frame::Frame;
use crate::error::{DaiError, DaiResult};
use daic_sys::root::daic::{device_create, device_destroy, device_is_connected, DeviceHandle, dai_get_last_error};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Device platform information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    /// RVC2 platform
    Rvc2,
    /// RVC3 platform  
    Rvc3,
    /// RVC4 platform
    Rvc4,
    /// Unknown platform
    Unknown(u32),
}

pub struct Device {
    inner: Arc<Mutex<DeviceInner>>,
    last_capture: Arc<Mutex<Option<Instant>>>,
    ffi_handle: DeviceHandle,
}

struct DeviceInner {
    device_ptr: *mut dai::Device,
    is_connected: bool,
    capture_count: u32,
}

impl Device {
    /// Create a new DepthAI device instance with better error handling
    pub fn new() -> DaiResult<Self> {
        // Create FFI handle for new pipeline API
        let ffi_handle = unsafe { device_create() };
        if ffi_handle.is_null() {
            return Err(DaiError::from_ffi());
        }

        // Use null pointer for legacy compatibility
        let device_ptr = std::ptr::null_mut();
        
        let inner = DeviceInner {
            device_ptr,
            is_connected: true,
            capture_count: 0,
        };
        
        Ok(Device {
            inner: Arc::new(Mutex::new(inner)),
            last_capture: Arc::new(Mutex::new(None)),
            ffi_handle,
        })
    }

    /// Create a new DepthAI device instance (legacy method for compatibility)
    pub fn new_legacy() -> Result<Self, &'static str> {
        match Self::new() {
            Ok(device) => Ok(device),
            Err(_) => Err("Failed to create device")
        }
    }

    /// Capture a frame from the device
    pub fn capture_frame(&self) -> Result<Frame, &'static str> {
        // Rate limiting: minimum 10ms between captures for stability
        {
            let mut last_capture = self.last_capture.lock().unwrap();
            if let Some(last_time) = *last_capture {
                let elapsed = last_time.elapsed();
                if elapsed < Duration::from_millis(10) {
                    std::thread::sleep(Duration::from_millis(10) - elapsed);
                }
            }
            *last_capture = Some(Instant::now());
        }

        let mut inner = self.inner.lock().unwrap();
        
        // Check if device is still connected
        if !inner.is_connected {
            return Err("Device disconnected");
        }

        // Increment capture count for monitoring
        inner.capture_count += 1;
        
        // For now, return a dummy frame with proper dimensions
        // TODO: Implement actual capture when C++ bindings are stable
        Ok(Frame::new_with_data(640, 480, vec![128; 640 * 480]))
    }

    /// Get capture statistics
    pub fn get_capture_count(&self) -> u32 {
        let inner = self.inner.lock().unwrap();
        inner.capture_count
    }

    /// Check if device is connected using FFI
    pub fn is_connected(&self) -> DaiResult<bool> {
        unsafe {
            let connected = device_is_connected(self.ffi_handle);
            // Check if there was an FFI error
            let error_ptr = dai_get_last_error();
            if !error_ptr.is_null() {
                return Err(DaiError::from_ffi());
            }
            Ok(connected)
        }
    }

    /// Disconnect device gracefully
    pub fn disconnect(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.is_connected = false;
    }

    /// Get the raw handle for FFI operations
    /// 
    /// # Safety
    /// 
    /// This is intended for internal use only. The handle must not be used after
    /// the Device is dropped.
    pub(crate) unsafe fn as_raw(&self) -> DeviceHandle {
        self.ffi_handle
    }

    /// Get the device handle for pipeline operations
    pub fn get_handle(&self) -> DeviceHandle {
        self.ffi_handle
    }
}

// Implement proper cleanup - safer version
impl Drop for DeviceInner {
    fn drop(&mut self) {
        // Safe cleanup - no C++ deallocation to avoid crashes
        self.is_connected = false;
        self.device_ptr = std::ptr::null_mut();
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if !self.ffi_handle.is_null() {
            unsafe {
                device_destroy(self.ffi_handle);
            }
        }
    }
}

// Thread-safe device sharing
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        // Test that device creation returns a result (may fail without hardware)
        let device_result = Device::new();
        match device_result {
            Ok(_device) => {
                println!("Device created successfully");
                // Basic creation successful, this is enough for unit testing
            }
            Err(DaiError::FfiError(msg)) if msg.contains("No available devices") => {
                println!("Expected error: No devices available");
            }
            Err(e) => panic!("Unexpected error: {}", e),
        }
    }

    #[test]
    fn test_device_info_creation() {
        // Test device info structure creation (doesn't require hardware)
        let device_info = crate::device_info::DeviceInfo::new();
        
        // Just test that we can create a DeviceInfo without panicking
        // The actual values depend on the C++ implementation
        let _name = device_info.get_name();
        
        println!("DeviceInfo created successfully");
    }
}
