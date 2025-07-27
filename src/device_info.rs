//! Safe Rust wrapper for DepthAI DeviceInfo

use crate::{bindings::root::dai, misc};

/// Safe wrapper for DepthAI DeviceInfo
pub struct DeviceInfo {
    inner: dai::DeviceInfo,
}

impl DeviceInfo {
    /// Create a new DeviceInfo instance with default values
    pub fn new() -> Self {
        let inner = unsafe {
            dai::DeviceInfo::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default()
            )
        };
        DeviceInfo { inner }
    }

    /// Create a DeviceInfo with custom parameters
    pub fn with_params(
        _name: &str,
        _protocol: i32,
        _platform: i32,
        _device_id: u32,
        _state: i32,
        _usb_speed: bool,
    ) -> Result<Self, &'static str> {
        // For now, just use default values since the exact parameter types are unclear
        let inner = unsafe {
            dai::DeviceInfo::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default()
            )
        };
        Ok(DeviceInfo { inner })
    }

    /// Get device name (placeholder - would need proper implementation)
    pub fn name(&self) -> String {
        // Use our helper function to convert the opaque std::string to Rust String
        misc::opaque_string_to_rust_string(&self.inner.name)
    }

    /// Get device protocol (placeholder)
    pub fn protocol(&self) -> i32 {
        // Placeholder - would access inner.protocol or similar
        0
    }

    /// Get device platform (placeholder)
    pub fn platform(&self) -> i32 {
        // Placeholder - would access inner.platform or similar
        0
    }

    /// Get device ID (placeholder)
    pub fn device_id(&self) -> u32 {
        // Placeholder - would access inner.device_id or similar
        0
    }

    /// Get device state (placeholder)
    pub fn state(&self) -> i32 {
        // Placeholder - would access inner.state or similar
        0
    }

    /// Check if device supports high USB speed (placeholder)
    pub fn usb_speed(&self) -> bool {
        // Placeholder - would access inner.usb_speed or similar
        false
    }

    /// Check if device is connected
    pub fn is_connected(&self) -> bool {
        // Placeholder implementation
        true
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeviceInfo(name: {}, id: {})", self.name(), self.device_id())
    }
}

impl std::fmt::Debug for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceInfo")
            .field("name", &self.name())
            .field("protocol", &self.protocol())
            .field("platform", &self.platform())
            .field("device_id", &self.device_id())
            .field("state", &self.state())
            .field("usb_speed", &self.usb_speed())
            .finish()
    }
}
