//! Safe Rust wrapper for DepthAI DeviceInfo

use crate::{bindings::root::dai, misc, device::Platform};

/// Safe wrapper for DepthAI DeviceInfo
pub struct DeviceInfo {
    inner: dai::DeviceInfo,
    cached_name: Option<String>,
    cached_mxid: Option<String>,
    cached_device_id: Option<String>,
}

/// Device connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    /// Device is not connected
    Unconnected,
    /// Device is connected and ready
    Connected,
    /// Device is in bootloader mode
    Bootloader,
    /// Device is initializing
    Initializing,
    /// Device connection failed
    Failed,
    /// Unknown state
    Unknown(u32),
}

/// Device protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceProtocol {
    /// USB protocol
    Usb,
    /// TCP/IP protocol
    Tcp,
    /// Unknown protocol
    Unknown(u32),
}

impl DeviceInfo {
    /// Create a new DeviceInfo instance from existing dai::DeviceInfo
    pub fn from_inner(inner: dai::DeviceInfo) -> Self {
        DeviceInfo { 
            inner,
            cached_name: None,
            cached_mxid: None,
            cached_device_id: None,
        }
    }

    /// Create a new DeviceInfo instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Get device name from the actual C++ structure field
    pub fn get_name(&self) -> String {
        misc::opaque_string_to_rust_string(&self.inner.name)
    }

    /// Get device name (cached version)
    pub fn name(&mut self) -> &str {
        if self.cached_name.is_none() {
            self.cached_name = Some(self.get_name());
        }
        self.cached_name.as_ref().unwrap()
    }

    /// Get device ID from the actual C++ structure field
    pub fn get_device_id(&self) -> String {
        misc::opaque_string_to_rust_string(&self.inner.deviceId)
    }

    /// Get device ID (cached version)
    pub fn device_id(&mut self) -> &str {
        if self.cached_device_id.is_none() {
            self.cached_device_id = Some(self.get_device_id());
        }
        self.cached_device_id.as_ref().unwrap()
    }

    /// Get device MXID (unique identifier) using the actual C++ method
    pub fn get_mxid(&self) -> String {
        let cpp_string = unsafe { self.inner.getMxId() };
        misc::opaque_string_to_rust_string(&cpp_string)
    }

    /// Get device MXID (cached version)
    pub fn mxid(&mut self) -> &str {
        if self.cached_mxid.is_none() {
            self.cached_mxid = Some(self.get_mxid());
        }
        self.cached_mxid.as_ref().unwrap()
    }

    /// Get device state from the actual C++ structure field
    pub fn state(&self) -> DeviceState {
        match self.inner.state {
            0 => DeviceState::Unconnected,
            1 => DeviceState::Connected,
            2 => DeviceState::Bootloader,
            3 => DeviceState::Initializing,
            4 => DeviceState::Failed,
            other => DeviceState::Unknown(other as u32),
        }
    }

    /// Get device protocol from the actual C++ structure field
    pub fn protocol(&self) -> DeviceProtocol {
        match self.inner.protocol {
            0 => DeviceProtocol::Usb,
            1 => DeviceProtocol::Tcp,
            other => DeviceProtocol::Unknown(other as u32),
        }
    }

    /// Get device platform from the actual C++ structure field
    pub fn platform(&self) -> Platform {
        match self.inner.platform {
            0 => Platform::Rvc2,
            1 => Platform::Rvc3,
            2 => Platform::Rvc4,
            other => Platform::Unknown(other as u32),
        }
    }

    /// Get status/error code from the actual C++ structure field
    pub fn status(&self) -> u32 {
        self.inner.status as u32
    }

    /// Check if device is connected
    pub fn is_connected(&self) -> bool {
        matches!(self.state(), DeviceState::Connected)
    }

    /// Check if device is in bootloader mode
    pub fn is_bootloader(&self) -> bool {
        matches!(self.state(), DeviceState::Bootloader)
    }

    /// Check if device uses USB protocol
    pub fn is_usb(&self) -> bool {
        matches!(self.protocol(), DeviceProtocol::Usb)
    }

    /// Check if device uses TCP protocol
    pub fn is_tcp(&self) -> bool {
        matches!(self.protocol(), DeviceProtocol::Tcp)
    }

    /// Get device description as string using the actual C++ method
    pub fn to_string(&self) -> String {
        let cpp_string = unsafe { self.inner.toString() };
        misc::opaque_string_to_rust_string(&cpp_string)
    }

    /// Get a reference to the inner dai::DeviceInfo
    pub fn inner(&self) -> &dai::DeviceInfo {
        &self.inner
    }

    /// Get a mutable reference to the inner dai::DeviceInfo
    pub fn inner_mut(&mut self) -> &mut dai::DeviceInfo {
        // Clear cache when inner is modified
        self.cached_name = None;
        self.cached_mxid = None;
        self.cached_device_id = None;
        &mut self.inner
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        let inner = unsafe {
            let mut device_info = std::mem::zeroed::<dai::DeviceInfo>();
            // Initialize with default values
            device_info.state = 0; // Unconnected
            device_info.protocol = 0; // USB
            device_info.platform = 0; // RVC2
            device_info.status = 0; // No error
            device_info
        };
        
        DeviceInfo {
            inner,
            cached_name: None,
            cached_mxid: None,
            cached_device_id: None,
        }
    }
}

impl Clone for DeviceInfo {
    fn clone(&self) -> Self {
        DeviceInfo {
            inner: self.inner,
            cached_name: self.cached_name.clone(),
            cached_mxid: self.cached_mxid.clone(),
            cached_device_id: self.cached_device_id.clone(),
        }
    }
}

impl std::fmt::Debug for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceInfo")
            .field("name", &self.get_name())
            .field("device_id", &self.get_device_id())
            .field("state", &self.state())
            .field("protocol", &self.protocol())
            .field("platform", &self.platform())
            .field("status", &self.status())
            .finish()
    }
}

impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use safe field access instead of C++ toString() method
        write!(f, "DeviceInfo(name: {}, id: {}, state: {:?}, protocol: {:?})", 
               self.get_name(), 
               self.get_device_id(), 
               self.state(), 
               self.protocol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_info_default() {
        let device_info = DeviceInfo::default();
        assert_eq!(device_info.state(), DeviceState::Unconnected);
        assert_eq!(device_info.protocol(), DeviceProtocol::Usb);
        assert_eq!(device_info.platform(), Platform::Rvc2);
        assert_eq!(device_info.status(), 0);
    }

    #[test]
    fn test_device_info_state_conversion() {
        let mut device_info = DeviceInfo::default();
        device_info.inner_mut().state = 1;
        assert_eq!(device_info.state(), DeviceState::Connected);
        assert!(device_info.is_connected());
        
        device_info.inner_mut().state = 2;
        assert_eq!(device_info.state(), DeviceState::Bootloader);
        assert!(device_info.is_bootloader());
    }

    #[test]
    fn test_device_info_protocol_conversion() {
        let mut device_info = DeviceInfo::default();
        device_info.inner_mut().protocol = 0;
        assert_eq!(device_info.protocol(), DeviceProtocol::Usb);
        assert!(device_info.is_usb());
        
        device_info.inner_mut().protocol = 1;
        assert_eq!(device_info.protocol(), DeviceProtocol::Tcp);
        assert!(device_info.is_tcp());
    }

    #[test]
    fn test_device_info_platform_conversion() {
        let mut device_info = DeviceInfo::default();
        device_info.inner_mut().platform = 0;
        assert_eq!(device_info.platform(), Platform::Rvc2);
        
        device_info.inner_mut().platform = 1;
        assert_eq!(device_info.platform(), Platform::Rvc3);
    }

    #[test]
    fn test_device_info_cache_invalidation() {
        let mut device_info = DeviceInfo::default();
        
        // Access cached value
        let _name = device_info.name();
        assert!(device_info.cached_name.is_some());
        
        // Modify inner - should clear cache
        device_info.inner_mut();
        assert!(device_info.cached_name.is_none());
    }

    #[test]
    fn test_device_info_clone() {
        let mut device_info = DeviceInfo::default();
        device_info.inner_mut().state = 1;
        device_info.inner_mut().protocol = 1;
        
        let cloned = device_info.clone();
        assert_eq!(cloned.state(), DeviceState::Connected);
        assert_eq!(cloned.protocol(), DeviceProtocol::Tcp);
    }

    #[test]
    fn test_device_info_debug_display() {
        let device_info = DeviceInfo::default();
        let debug_str = format!("{:?}", device_info);
        assert!(debug_str.contains("DeviceInfo"));
        assert!(debug_str.contains("state"));
        assert!(debug_str.contains("protocol"));
        
        // Note: Avoid Display formatting on default instance as it calls C++ toString()
        // which requires proper initialization
    }

    #[test]
    fn test_device_info_string_access() {
        let device_info = DeviceInfo::default();
        
        // Test direct access to C++ fields (not methods)
        let name = device_info.get_name();
        let device_id = device_info.get_device_id();
        
        // These should not panic even if strings are empty
        let _ = format!("Name: {}, ID: {}", name, device_id);
        
        // Note: Avoid calling get_mxid() or to_string() on default instance
        // as they call C++ methods on uninitialized data
    }

    #[test]
    fn test_device_info_cached_access() {
        let mut device_info = DeviceInfo::default();
        
        // Test cached access
        let name1 = device_info.name();
        let name1_ptr = name1.as_ptr();
        let name2 = device_info.name();
        // Should be the same reference
        assert_eq!(name1_ptr, name2.as_ptr());
        
        let device_id1 = device_info.device_id();
        let device_id1_ptr = device_id1.as_ptr();
        let device_id2 = device_info.device_id();
        assert_eq!(device_id1_ptr, device_id2.as_ptr());
    }
}
