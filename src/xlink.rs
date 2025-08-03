//! Safe Rust wrapper for XLink types and functions

/// XLink platform types corresponding to XLinkPlatform_t
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XLinkPlatform {
    /// Any platform
    AnyPlatform = 0,
    /// Myriad 2 platform
    Myriad2 = 2450,
    /// Myriad X platform
    MyriadX = 2480,
    /// RVC3 platform
    Rvc3 = 3000,
    /// RVC4 platform
    Rvc4 = 4000,
}

/// XLink device state corresponding to XLinkDeviceState_t
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XLinkDeviceState {
    /// Any state
    AnyState = 0,
    /// Device is booted
    Booted = 1,
    /// Device is not booted
    Unbooted = 2,
    /// Device is in bootloader mode
    Bootloader = 3,
    /// Device has flash booted / Device is booted (non-exclusive)
    FlashBooted = 4,
    /// Gateway state
    Gate = 5,
    /// Gateway booted state
    GateBooted = 6,
    /// Gateway setup state
    GateSetup = 7,
}

impl XLinkDeviceState {
    /// Alias for FlashBooted for backward compatibility
    pub const BOOTED_NON_EXCLUSIVE: XLinkDeviceState = XLinkDeviceState::FlashBooted;
}

/// XLink protocol types corresponding to XLinkProtocol_t  
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XLinkProtocol {
    /// USB over Vision Security Chip
    UsbVsc = 0,
    /// USB over Communication Device Class
    UsbCdc = 1,
    /// PCIe
    Pcie = 2,
    /// IPC
    Ipc = 3,
    /// TCP/IP
    TcpIp = 4,
    /// Local shared memory
    LocalShdmem = 5,
    /// TCP/IP or local shared memory
    TcpIpOrLocalShdmem = 6,
    /// Number of protocols
    NmbOfProtocols = 7,
    /// Any protocol
    AnyProtocol = 8,
}

/// XLink error codes corresponding to XLinkError_t
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XLinkError {
    /// Success
    Success = 0,
    /// Already open
    AlreadyOpen = 1,
    /// Communication not open
    CommunicationNotOpen = 2,
    /// Communication fail
    CommunicationFail = 3,
    /// Communication unknown error
    CommunicationUnknownError = 4,
    /// Device not found
    DeviceNotFound = 5,
    /// Timeout
    Timeout = 6,
    /// Error
    Error = 7,
    /// Out of memory
    OutOfMemory = 8,
    /// Insufficient permissions
    InsufficientPermissions = 9,
    /// Device already in use
    DeviceAlreadyInUse = 10,
    /// Not implemented
    NotImplemented = 11,
    /// USB initialization error
    InitUsbError = 12,
    /// TCP/IP initialization error
    InitTcpIpError = 13,
    /// Local shared memory initialization error
    InitLocalShdmemError = 14,
    /// TCP/IP or local shared memory initialization error
    InitTcpIpOrLocalShdmemError = 15,
    /// PCIe initialization error
    InitPcieError = 16,
}

/// Device descriptor structure corresponding to deviceDesc_t
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceDesc {
    /// Protocol used
    pub protocol: XLinkProtocol,
    /// Device platform
    pub platform: XLinkPlatform,
    /// Device name (64 bytes as per XLINK_MAX_NAME_SIZE)
    pub name: [i8; 64],
    /// Device state
    pub state: XLinkDeviceState,
    /// Device MXID (32 bytes as per XLINK_MAX_MX_ID_SIZE)
    pub mxid: [i8; 32],
    /// Status/error code
    pub status: XLinkError,
    /// Name hint only flag
    pub name_hint_only: bool,
}

impl Default for DeviceDesc {
    fn default() -> Self {
        DeviceDesc {
            protocol: XLinkProtocol::UsbVsc,
            platform: XLinkPlatform::AnyPlatform,
            name: [0; 64],
            state: XLinkDeviceState::AnyState,
            mxid: [0; 32],
            status: XLinkError::Success,
            name_hint_only: false,
        }
    }
}

impl DeviceDesc {
    /// Create a new device descriptor
    pub fn new() -> Self {
        Self::default()
    }

    /// Set device name
    pub fn with_name(mut self, name: &str) -> Self {
        let name_bytes = name.as_bytes();
        let len = std::cmp::min(name_bytes.len(), 63); // Leave space for null terminator
        
        // Clear the array first
        self.name = [0; 64];
        
        // Copy the name bytes
        for (i, &byte) in name_bytes.iter().take(len).enumerate() {
            self.name[i] = byte as i8;
        }
        
        self
    }

    /// Set device MXID
    pub fn with_mxid(mut self, mxid: &str) -> Self {
        let mxid_bytes = mxid.as_bytes();
        let len = std::cmp::min(mxid_bytes.len(), 31); // Leave space for null terminator
        
        // Clear the array first
        self.mxid = [0; 32];
        
        // Copy the MXID bytes
        for (i, &byte) in mxid_bytes.iter().take(len).enumerate() {
            self.mxid[i] = byte as i8;
        }
        
        self
    }

    /// Set device platform
    pub fn with_platform(mut self, platform: XLinkPlatform) -> Self {
        self.platform = platform;
        self
    }

    /// Set device state
    pub fn with_state(mut self, state: XLinkDeviceState) -> Self {
        self.state = state;
        self
    }

    /// Set device protocol
    pub fn with_protocol(mut self, protocol: XLinkProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Set device status
    pub fn with_status(mut self, status: XLinkError) -> Self {
        self.status = status;
        self
    }

    /// Set name hint only flag
    pub fn with_name_hint_only(mut self, hint_only: bool) -> Self {
        self.name_hint_only = hint_only;
        self
    }

    /// Get device name as a string
    pub fn get_name(&self) -> String {
        // Find the null terminator
        let end = self.name.iter()
            .position(|&c| c == 0)
            .unwrap_or(self.name.len());
        
        // Convert to bytes and then to string
        let bytes: Vec<u8> = self.name[..end]
            .iter()
            .map(|&c| c as u8)
            .collect();
        
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Get device MXID as a string
    pub fn get_mxid(&self) -> String {
        // Find the null terminator
        let end = self.mxid.iter()
            .position(|&c| c == 0)
            .unwrap_or(self.mxid.len());
        
        // Convert to bytes and then to string
        let bytes: Vec<u8> = self.mxid[..end]
            .iter()
            .map(|&c| c as u8)
            .collect();
        
        String::from_utf8_lossy(&bytes).to_string()
    }
}

// Note: From/Into conversions with deviceDesc_t will be added when 
// the C bindings are properly generated and available

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_desc_default() {
        let desc = DeviceDesc::default();
        assert_eq!(desc.platform, XLinkPlatform::AnyPlatform);
        assert_eq!(desc.state, XLinkDeviceState::AnyState);
        assert_eq!(desc.protocol, XLinkProtocol::UsbVsc);
        assert_eq!(desc.status, XLinkError::Success);
        assert_eq!(desc.name_hint_only, false);
        assert_eq!(desc.get_name(), "");
        assert_eq!(desc.get_mxid(), "");
    }

    #[test]
    fn test_device_desc_with_name() {
        let desc = DeviceDesc::new().with_name("test_device");
        assert_eq!(desc.get_name(), "test_device");
    }

    #[test]
    fn test_device_desc_with_mxid() {
        let desc = DeviceDesc::new().with_mxid("1844301041C3D907");
        assert_eq!(desc.get_mxid(), "1844301041C3D907");
    }

    #[test]
    fn test_device_desc_with_platform() {
        let desc = DeviceDesc::new().with_platform(XLinkPlatform::Rvc4);
        assert_eq!(desc.platform, XLinkPlatform::Rvc4);
    }

    #[test]
    fn test_device_desc_long_name() {
        let long_name = "this_is_a_very_long_device_name_that_exceeds_63_characters_limit";
        let desc = DeviceDesc::new().with_name(long_name);
        let result_name = desc.get_name();
        
        // Name should be truncated to fit in 63 characters (with null terminator)
        assert!(result_name.len() <= 63);
        assert!(long_name.starts_with(&result_name));
    }

    #[test]
    fn test_device_desc_builder_pattern() {
        let desc = DeviceDesc::new()
            .with_name("my_device")
            .with_mxid("1844301041C3D907")
            .with_platform(XLinkPlatform::Rvc3)
            .with_state(XLinkDeviceState::Booted)
            .with_protocol(XLinkProtocol::TcpIp)
            .with_status(XLinkError::Success)
            .with_name_hint_only(false);
        
        assert_eq!(desc.get_name(), "my_device");
        assert_eq!(desc.get_mxid(), "1844301041C3D907");
        assert_eq!(desc.platform, XLinkPlatform::Rvc3);
        assert_eq!(desc.state, XLinkDeviceState::Booted);
        assert_eq!(desc.protocol, XLinkProtocol::TcpIp);
        assert_eq!(desc.status, XLinkError::Success);
        assert_eq!(desc.name_hint_only, false);
    }

    #[test]
    fn test_xlink_enums() {
        // Test that enums have the correct discriminant values from XLink headers
        assert_eq!(XLinkPlatform::AnyPlatform as u32, 0);
        assert_eq!(XLinkPlatform::Myriad2 as u32, 2450);
        assert_eq!(XLinkPlatform::MyriadX as u32, 2480);
        assert_eq!(XLinkPlatform::Rvc3 as u32, 3000);
        assert_eq!(XLinkPlatform::Rvc4 as u32, 4000);

        assert_eq!(XLinkDeviceState::AnyState as u32, 0);
        assert_eq!(XLinkDeviceState::Booted as u32, 1);
        assert_eq!(XLinkDeviceState::Unbooted as u32, 2);
        assert_eq!(XLinkDeviceState::Bootloader as u32, 3);
        assert_eq!(XLinkDeviceState::FlashBooted as u32, 4);

        assert_eq!(XLinkProtocol::UsbVsc as u32, 0);
        assert_eq!(XLinkProtocol::UsbCdc as u32, 1);
        assert_eq!(XLinkProtocol::Pcie as u32, 2);
        assert_eq!(XLinkProtocol::TcpIp as u32, 4);

        assert_eq!(XLinkError::Success as u32, 0);
        assert_eq!(XLinkError::DeviceNotFound as u32, 5);
        assert_eq!(XLinkError::DeviceAlreadyInUse as u32, 10);
    }
}
