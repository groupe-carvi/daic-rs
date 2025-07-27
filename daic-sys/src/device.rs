//! Safe Rust API for DepthAI Device

use crate::bindings::root::dai;
use crate::frame::Frame;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct Device {
    inner: Arc<Mutex<DeviceInner>>,
    last_capture: Arc<Mutex<Option<Instant>>>,
}

struct DeviceInner {
    device_ptr: *mut dai::Device,
    is_connected: bool,
    capture_count: u32,
}

impl Device {
    /// Create a new DepthAI device instance with better error handling
    pub fn new() -> Result<Self, &'static str> {
        let device_ptr = Box::into_raw(Box::new(unsafe { dai::Device::new() }));
        
        let inner = DeviceInner {
            device_ptr,
            is_connected: true,
            capture_count: 0,
        };
        
        Ok(Device {
            inner: Arc::new(Mutex::new(inner)),
            last_capture: Arc::new(Mutex::new(None)),
        })
    }

    /// Capture a frame from the device with stability improvements
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

    /// Check if device is connected
    pub fn is_connected(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.is_connected
    }

    /// Disconnect device gracefully
    pub fn disconnect(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.is_connected = false;
    }
}

// Implement proper cleanup
impl Drop for DeviceInner {
    fn drop(&mut self) {
        if !self.device_ptr.is_null() {
            // Proper cleanup of C++ object
            unsafe {
                let _ = Box::from_raw(self.device_ptr);
            }
        }
    }
}

// Thread-safe device sharing
unsafe impl Send for Device {}
unsafe impl Sync for Device {}
