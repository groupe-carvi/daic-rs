//! Safe Rust API for DepthAI common types
//!
//! This module provides safe Rust wrappers around DepthAI common structures
//! from the C++ depthai/common directory. It includes geometric types, camera
//! configurations, and various enumerations used throughout the DepthAI API.

use std::fmt;

// ============================================================================
// GEOMETRIC TYPES
// ============================================================================

/// 2D point with floating-point coordinates
/// 
/// Corresponds to dai::Point2f in the C++ API
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2f {
    pub x: f32,
    pub y: f32,
    pub normalized: bool,
}

impl Point2f {
    /// Create a new 2D point
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            normalized: false,
        }
    }

    /// Create a new normalized 2D point (coordinates in [0,1])
    pub fn new_normalized(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            normalized: true,
        }
    }

    /// Check if the point coordinates are normalized
    pub fn is_normalized(&self) -> bool {
        self.normalized
    }

    /// Convert to denormalized coordinates given image dimensions
    pub fn denormalize(&self, width: f32, height: f32) -> Point2f {
        if self.normalized {
            Point2f::new(self.x * width, self.y * height)
        } else {
            *self
        }
    }

    /// Convert to normalized coordinates given image dimensions
    pub fn normalize(&self, width: f32, height: f32) -> Point2f {
        if !self.normalized {
            Point2f::new_normalized(self.x / width, self.y / height)
        } else {
            *self
        }
    }
}

impl Default for Point2f {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl fmt::Display for Point2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

/// 3D point with floating-point coordinates
/// 
/// Corresponds to dai::Point3f in the C++ API
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3f {
    /// Create a new 3D point
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculate distance from origin
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &Point3f) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Default for Point3f {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Point3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

/// 2D size with floating-point dimensions
/// 
/// Corresponds to dai::Size2f in the C++ API
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size2f {
    pub width: f32,
    pub height: f32,
    pub normalized: bool,
}

impl Size2f {
    /// Create a new size
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            normalized: false,
        }
    }

    /// Create a new normalized size (dimensions in [0,1])
    pub fn new_normalized(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            normalized: true,
        }
    }

    /// Check if the size is normalized
    pub fn is_normalized(&self) -> bool {
        self.normalized || (self.width >= 0.0 && self.width <= 1.0 && self.height >= 0.0 && self.height <= 1.0)
    }

    /// Calculate area
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Calculate aspect ratio (width/height)
    pub fn aspect_ratio(&self) -> f32 {
        if self.height != 0.0 {
            self.width / self.height
        } else {
            0.0
        }
    }
}

impl Default for Size2f {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl fmt::Display for Size2f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/// Rectangle defined by position and size
/// 
/// Corresponds to dai::Rect in the C++ API
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub normalized: bool,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            normalized: false,
        }
    }

    /// Create a new normalized rectangle (coordinates in [0,1])
    pub fn new_normalized(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            normalized: true,
        }
    }

    /// Create rectangle from two corner points
    pub fn from_points(p1: Point2f, p2: Point2f) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let width = (p1.x - p2.x).abs();
        let height = (p1.y - p2.y).abs();
        
        Self {
            x,
            y,
            width,
            height,
            normalized: p1.normalized && p2.normalized,
        }
    }

    /// Create rectangle from origin point and size
    pub fn from_point_size(origin: Point2f, size: Size2f) -> Self {
        Self {
            x: origin.x,
            y: origin.y,
            width: size.width,
            height: size.height,
            normalized: origin.normalized && size.normalized,
        }
    }

    /// Get top-left corner
    pub fn top_left(&self) -> Point2f {
        Point2f {
            x: self.x,
            y: self.y,
            normalized: self.normalized,
        }
    }

    /// Get bottom-right corner
    pub fn bottom_right(&self) -> Point2f {
        Point2f {
            x: self.x + self.width,
            y: self.y + self.height,
            normalized: self.normalized,
        }
    }

    /// Get center point
    pub fn center(&self) -> Point2f {
        Point2f {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
            normalized: self.normalized,
        }
    }

    /// Get rectangle size
    pub fn size(&self) -> Size2f {
        Size2f {
            width: self.width,
            height: self.height,
            normalized: self.normalized,
        }
    }

    /// Calculate area
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Check if point is inside rectangle
    pub fn contains(&self, point: Point2f) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }

    /// Check if this rectangle intersects with another
    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.x + self.width < other.x || 
          other.x + other.width < self.x ||
          self.y + self.height < other.y ||
          other.y + other.height < self.y)
    }

    /// Check if the rectangle is normalized
    pub fn is_normalized(&self) -> bool {
        self.normalized
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rect({:.2}, {:.2}, {:.2}x{:.2})", 
               self.x, self.y, self.width, self.height)
    }
}

// ============================================================================
// CAMERA ENUMERATIONS
// ============================================================================

/// Camera board socket enumeration
/// 
/// Corresponds to dai::CameraBoardSocket in the C++ API
/// - Auto: automatically detects the camera socket
/// - CamA: RGB or center camera
/// - CamB: left camera
/// - CamC: right camera
/// - CamD: top camera
/// - CamE: bottom camera
/// - CamF: depth camera
/// - CamG: IR camera
/// - CamH: ultra wide camera
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraBoardSocket {
    /// Auto-detect camera socket
    Auto,
    /// RGB or Center Camera
    CamA,
    /// Left Camera
    CamB,
    /// Right Camera
    CamC,
    /// Top Camera
    CamD,
    /// Bottom Camera
    CamE,
    /// Depth Camera
    CamF,
    /// IR Camera
    CamG,
    /// Ultra Wide Camera
    CamH,
}

impl CameraBoardSocket {
    /// Convert to C++ enum value
    pub fn to_c(&self) -> u32 {
        match self {
            CameraBoardSocket::Auto => 0,
            CameraBoardSocket::CamA => 1,
            CameraBoardSocket::CamB => 2,
            CameraBoardSocket::CamC => 3,
            CameraBoardSocket::CamD => 4,
            CameraBoardSocket::CamE => 5,
            CameraBoardSocket::CamF => 6,
            CameraBoardSocket::CamG => 7,
            CameraBoardSocket::CamH => 8,
        }
    }

    /// Create from C++ enum value
    pub fn from_c(value: u32) -> Option<Self> {
        match value {
            0 => Some(CameraBoardSocket::Auto),
            1 => Some(CameraBoardSocket::CamA),
            2 => Some(CameraBoardSocket::CamB),
            3 => Some(CameraBoardSocket::CamC),
            4 => Some(CameraBoardSocket::CamD),
            5 => Some(CameraBoardSocket::CamE),
            6 => Some(CameraBoardSocket::CamF),
            7 => Some(CameraBoardSocket::CamG),
            8 => Some(CameraBoardSocket::CamH),
            _ => None,
        }
    }
}

impl Default for CameraBoardSocket {
    fn default() -> Self {
        CameraBoardSocket::Auto
    }
}

impl fmt::Display for CameraBoardSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameraBoardSocket::Auto => write!(f, "AUTO"),
            CameraBoardSocket::CamA => write!(f, "CAM_A"),
            CameraBoardSocket::CamB => write!(f, "CAM_B"),
            CameraBoardSocket::CamC => write!(f, "CAM_C"),
            CameraBoardSocket::CamD => write!(f, "CAM_D"),
            CameraBoardSocket::CamE => write!(f, "CAM_E"),
            CameraBoardSocket::CamF => write!(f, "CAM_F"),
            CameraBoardSocket::CamG => write!(f, "CAM_G"),
            CameraBoardSocket::CamH => write!(f, "CAM_H"),
        }
    }
}

/// Camera sensor type enumeration
/// 
/// Corresponds to dai::CameraSensorType in the C++ API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraSensorType {
    Color,
    Mono,
    Thermal,
    Tof,
}

impl CameraSensorType {
    /// Convert to C++ enum value
    pub fn to_c(&self) -> u32 {
        match self {
            CameraSensorType::Color => 0,
            CameraSensorType::Mono => 1,
            CameraSensorType::Thermal => 2,
            CameraSensorType::Tof => 3,
        }
    }

    /// Create from C++ enum value
    pub fn from_c(value: u32) -> Option<Self> {
        match value {
            0 => Some(CameraSensorType::Color),
            1 => Some(CameraSensorType::Mono),
            2 => Some(CameraSensorType::Thermal),
            3 => Some(CameraSensorType::Tof),
            _ => None,
        }
    }
}

impl Default for CameraSensorType {
    fn default() -> Self {
        CameraSensorType::Color
    }
}

impl fmt::Display for CameraSensorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameraSensorType::Color => write!(f, "COLOR"),
            CameraSensorType::Mono => write!(f, "MONO"),
            CameraSensorType::Thermal => write!(f, "THERMAL"),
            CameraSensorType::Tof => write!(f, "TOF"),
        }
    }
}

/// Camera image orientation enumeration
/// 
/// Corresponds to dai::CameraImageOrientation in the C++ API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraImageOrientation {
    Auto,
    Normal,
    Rotate90Deg,
    Rotate180Deg,
    Rotate270Deg,
    HorizontalFlip,
    VerticalFlip,
}

impl CameraImageOrientation {
    /// Convert to C++ enum value
    pub fn to_c(&self) -> u32 {
        match self {
            CameraImageOrientation::Auto => 0,
            CameraImageOrientation::Normal => 1,
            CameraImageOrientation::Rotate90Deg => 2,
            CameraImageOrientation::Rotate180Deg => 3,
            CameraImageOrientation::Rotate270Deg => 4,
            CameraImageOrientation::HorizontalFlip => 5,
            CameraImageOrientation::VerticalFlip => 6,
        }
    }

    /// Create from C++ enum value
    pub fn from_c(value: u32) -> Option<Self> {
        match value {
            0 => Some(CameraImageOrientation::Auto),
            1 => Some(CameraImageOrientation::Normal),
            2 => Some(CameraImageOrientation::Rotate90Deg),
            3 => Some(CameraImageOrientation::Rotate180Deg),
            4 => Some(CameraImageOrientation::Rotate270Deg),
            5 => Some(CameraImageOrientation::HorizontalFlip),
            6 => Some(CameraImageOrientation::VerticalFlip),
            _ => None,
        }
    }
}

impl Default for CameraImageOrientation {
    fn default() -> Self {
        CameraImageOrientation::Auto
    }
}

/// Camera model enumeration
/// 
/// Corresponds to dai::CameraModel in the C++ API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraModel {
    Perspective,
    Fisheye,
    Equirectangular,
    RadialDivision,
    Thinprism,
    Tilted,
}

impl CameraModel {
    /// Convert to C++ enum value
    pub fn to_c(&self) -> u32 {
        match self {
            CameraModel::Perspective => 0,
            CameraModel::Fisheye => 1,
            CameraModel::Equirectangular => 2,
            CameraModel::RadialDivision => 3,
            CameraModel::Thinprism => 4,
            CameraModel::Tilted => 5,
        }
    }

    /// Create from C++ enum value
    pub fn from_c(value: u32) -> Option<Self> {
        match value {
            0 => Some(CameraModel::Perspective),
            1 => Some(CameraModel::Fisheye),
            2 => Some(CameraModel::Equirectangular),
            3 => Some(CameraModel::RadialDivision),
            4 => Some(CameraModel::Thinprism),
            5 => Some(CameraModel::Tilted),
            _ => None,
        }
    }
}

impl Default for CameraModel {
    fn default() -> Self {
        CameraModel::Perspective
    }
}

/// USB speed enumeration
/// 
/// Corresponds to dai::UsbSpeed in the C++ API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbSpeed {
    Unknown,
    Low,
    Full,
    High,
    Super,
    SuperPlus,
}

impl UsbSpeed {
    /// Convert to C++ enum value
    pub fn to_c(&self) -> u32 {
        match self {
            UsbSpeed::Unknown => 0,
            UsbSpeed::Low => 1,
            UsbSpeed::Full => 2,
            UsbSpeed::High => 3,
            UsbSpeed::Super => 4,
            UsbSpeed::SuperPlus => 5,
        }
    }

    /// Create from C++ enum value
    pub fn from_c(value: u32) -> Option<Self> {
        match value {
            0 => Some(UsbSpeed::Unknown),
            1 => Some(UsbSpeed::Low),
            2 => Some(UsbSpeed::Full),
            3 => Some(UsbSpeed::High),
            4 => Some(UsbSpeed::Super),
            5 => Some(UsbSpeed::SuperPlus),
            _ => None,
        }
    }

    /// Get human-readable speed description
    pub fn description(&self) -> &'static str {
        match self {
            UsbSpeed::Unknown => "Unknown",
            UsbSpeed::Low => "Low Speed (1.5 Mbps)",
            UsbSpeed::Full => "Full Speed (12 Mbps)",
            UsbSpeed::High => "High Speed (480 Mbps)",
            UsbSpeed::Super => "SuperSpeed (5 Gbps)",
            UsbSpeed::SuperPlus => "SuperSpeed+ (10 Gbps)",
        }
    }
}

impl Default for UsbSpeed {
    fn default() -> Self {
        UsbSpeed::Unknown
    }
}

impl fmt::Display for UsbSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

// ============================================================================
// CAMERA CONFIGURATION STRUCTURES
// ============================================================================

/// Camera sensor configuration
/// 
/// Corresponds to dai::CameraSensorConfig in the C++ API
#[derive(Debug, Clone, PartialEq)]
pub struct CameraSensorConfig {
    pub width: i32,
    pub height: i32,
    pub min_fps: f32,
    pub max_fps: f32,
    pub fov: Rect,
    pub sensor_type: CameraSensorType,
}

impl CameraSensorConfig {
    /// Create a new camera sensor configuration
    pub fn new(
        width: i32,
        height: i32,
        min_fps: f32,
        max_fps: f32,
        fov: Rect,
        sensor_type: CameraSensorType,
    ) -> Self {
        Self {
            width,
            height,
            min_fps,
            max_fps,
            fov,
            sensor_type,
        }
    }

    /// Get resolution as a tuple
    pub fn resolution(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    /// Get pixel count
    pub fn pixel_count(&self) -> i32 {
        self.width * self.height
    }

    /// Get aspect ratio
    pub fn aspect_ratio(&self) -> f32 {
        if self.height != 0 {
            self.width as f32 / self.height as f32
        } else {
            0.0
        }
    }

    /// Check if configuration supports given framerate
    pub fn supports_fps(&self, fps: f32) -> bool {
        fps >= self.min_fps && fps <= self.max_fps
    }
}

impl Default for CameraSensorConfig {
    fn default() -> Self {
        Self {
            width: -1,
            height: -1,
            min_fps: -1.0,
            max_fps: -1.0,
            fov: Rect::default(),
            sensor_type: CameraSensorType::default(),
        }
    }
}

impl fmt::Display for CameraSensorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{} {:.1}-{:.1}fps {}", 
               self.width, self.height, self.min_fps, self.max_fps, self.sensor_type)
    }
}

/// Camera features and capabilities
/// 
/// Corresponds to dai::CameraFeatures in the C++ API
#[derive(Debug, Clone)]
pub struct CameraFeatures {
    pub socket: CameraBoardSocket,
    pub sensor_name: String,
    pub width: i32,
    pub height: i32,
    pub orientation: CameraImageOrientation,
    pub supported_types: Vec<CameraSensorType>,
    pub has_autofocus_ic: bool,
    pub has_autofocus: bool,
    pub name: String,
    pub additional_names: Vec<String>,
    pub configs: Vec<CameraSensorConfig>,
    pub calibration_resolution: Option<CameraSensorConfig>,
}

impl CameraFeatures {
    /// Create a new camera features instance
    pub fn new(socket: CameraBoardSocket, sensor_name: String) -> Self {
        Self {
            socket,
            sensor_name,
            width: -1,
            height: -1,
            orientation: CameraImageOrientation::Auto,
            supported_types: vec![],
            has_autofocus_ic: false,
            has_autofocus: false,
            name: String::new(),
            additional_names: vec![],
            configs: vec![],
            calibration_resolution: None,
        }
    }

    /// Get maximum resolution
    pub fn max_resolution(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    /// Get pixel count for maximum resolution
    pub fn max_pixel_count(&self) -> i32 {
        self.width * self.height
    }

    /// Check if camera supports a specific sensor type
    pub fn supports_type(&self, sensor_type: CameraSensorType) -> bool {
        self.supported_types.contains(&sensor_type)
    }

    /// Get configuration for specific resolution
    pub fn get_config_for_resolution(&self, width: i32, height: i32) -> Option<&CameraSensorConfig> {
        self.configs.iter().find(|config| config.width == width && config.height == height)
    }

    /// Get all available resolutions
    pub fn available_resolutions(&self) -> Vec<(i32, i32)> {
        self.configs.iter().map(|config| (config.width, config.height)).collect()
    }

    /// Check if autofocus is available
    pub fn supports_autofocus(&self) -> bool {
        self.has_autofocus || self.has_autofocus_ic
    }
}

impl Default for CameraFeatures {
    fn default() -> Self {
        Self {
            socket: CameraBoardSocket::Auto,
            sensor_name: String::new(),
            width: -1,
            height: -1,
            orientation: CameraImageOrientation::Auto,
            supported_types: vec![],
            has_autofocus_ic: false,
            has_autofocus: false,
            name: String::new(),
            additional_names: vec![],
            configs: vec![],
            calibration_resolution: None,
        }
    }
}

impl fmt::Display for CameraFeatures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} ({}x{}) on {}", 
               self.name, self.sensor_name, self.width, self.height, self.socket)
    }
}

/// Camera intrinsic and extrinsic parameters
/// 
/// Corresponds to dai::CameraInfo in the C++ API
#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub width: u32,
    pub height: u32,
    pub intrinsic_matrix: [[f64; 3]; 3],
    pub distortion_coefficients: Vec<f64>,
    pub extrinsic_matrix: [[f64; 4]; 4],
    pub camera_type: CameraModel,
}

impl CameraInfo {
    /// Create a new camera info instance
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            intrinsic_matrix: [[0.0; 3]; 3],
            distortion_coefficients: vec![],
            extrinsic_matrix: [[0.0; 4]; 4],
            camera_type: CameraModel::Perspective,
        }
    }

    /// Get resolution
    pub fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get focal length from intrinsic matrix
    pub fn focal_length(&self) -> (f64, f64) {
        (self.intrinsic_matrix[0][0], self.intrinsic_matrix[1][1])
    }

    /// Get principal point from intrinsic matrix
    pub fn principal_point(&self) -> (f64, f64) {
        (self.intrinsic_matrix[0][2], self.intrinsic_matrix[1][2])
    }

    /// Check if camera is calibrated
    pub fn is_calibrated(&self) -> bool {
        // Camera is considered calibrated if focal length is set
        let (fx, fy) = self.focal_length();
        fx != 0.0 && fy != 0.0
    }
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

// ============================================================================
// VERSION INFORMATION
// ============================================================================

/// Version information structure
/// 
/// Corresponds to dai::Version in the C++ API
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release_type: String,
    pub pre_release_version: u32,
    pub build_info: String,
}

impl Version {
    /// Create a new version instance
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release_type: String::new(),
            pre_release_version: 0,
            build_info: String::new(),
        }
    }

    /// Get version as string (e.g., "2.1.3")
    pub fn version_string(&self) -> String {
        if self.pre_release_type.is_empty() {
            format!("{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            format!("{}.{}.{}-{}.{}", 
                    self.major, self.minor, self.patch, 
                    self.pre_release_type, self.pre_release_version)
        }
    }

    /// Compare versions (returns true if this version is newer)
    pub fn is_newer_than(&self, other: &Version) -> bool {
        if self.major != other.major {
            return self.major > other.major;
        }
        if self.minor != other.minor {
            return self.minor > other.minor;
        }
        if self.patch != other.patch {
            return self.patch > other.patch;
        }
        // If pre-release types differ, consider semantic versioning rules
        match (&self.pre_release_type.is_empty(), &other.pre_release_type.is_empty()) {
            (true, false) => true,  // Release is newer than pre-release
            (false, true) => false, // Pre-release is older than release
            (false, false) => self.pre_release_version > other.pre_release_version,
            (true, true) => false,  // Same version
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version_string())
    }
}

// ============================================================================
// DEVICE INFORMATION
// ============================================================================

/// Device information structure
/// 
/// Contains information about a connected DepthAI device
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_name: String,
    pub mx_id: String,
    pub protocol: String,
    pub platform: String,
    pub product_name: String,
    pub board_name: String,
    pub board_rev: String,
    pub state: String,
}

impl DeviceInfo {
    /// Create a new device info instance
    pub fn new() -> Self {
        Self {
            device_name: String::new(),
            mx_id: String::new(),
            protocol: String::new(),
            platform: String::new(),
            product_name: String::new(),
            board_name: String::new(),
            board_rev: String::new(),
            state: String::new(),
        }
    }

    /// Check if device is connected via USB
    pub fn is_usb(&self) -> bool {
        self.protocol.to_lowercase().contains("usb")
    }

    /// Check if device is connected via Ethernet
    pub fn is_ethernet(&self) -> bool {
        self.protocol.to_lowercase().contains("tcp") || 
        self.protocol.to_lowercase().contains("ethernet")
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) - {} via {}", 
               self.device_name, self.mx_id, self.state, self.protocol)
    }
}

// ============================================================================
// CONVERSION TRAITS
// ============================================================================

/// Trait for converting between Rust and C++ representations
pub trait FromCpp<T> {
    fn from_cpp(value: T) -> Self;
}

/// Trait for converting from Rust to C++ representations  
pub trait ToCpp<T> {
    fn to_cpp(&self) -> T;
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2f_creation() {
        let p1 = Point2f::new(10.0, 20.0);
        assert_eq!(p1.x, 10.0);
        assert_eq!(p1.y, 20.0);
        assert!(!p1.is_normalized());

        let p2 = Point2f::new_normalized(0.5, 0.8);
        assert_eq!(p2.x, 0.5);
        assert_eq!(p2.y, 0.8);
        assert!(p2.is_normalized());
    }

    #[test]
    fn test_point2f_normalization() {
        let p = Point2f::new(100.0, 50.0);
        let normalized = p.normalize(200.0, 100.0);
        assert_eq!(normalized.x, 0.5);
        assert_eq!(normalized.y, 0.5);
        assert!(normalized.is_normalized());

        let denormalized = normalized.denormalize(200.0, 100.0);
        assert_eq!(denormalized.x, 100.0);
        assert_eq!(denormalized.y, 50.0);
        assert!(!denormalized.is_normalized());
    }

    #[test]
    fn test_rect_creation() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(rect.area(), 5000.0);
        assert_eq!(rect.center(), Point2f::new(60.0, 45.0));
        
        let top_left = rect.top_left();
        let bottom_right = rect.bottom_right();
        assert_eq!(top_left, Point2f::new(10.0, 20.0));
        assert_eq!(bottom_right, Point2f::new(110.0, 70.0));
    }

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10.0, 10.0, 80.0, 60.0);
        
        assert!(rect.contains(Point2f::new(50.0, 40.0))); // Inside
        assert!(rect.contains(Point2f::new(10.0, 10.0))); // Top-left corner
        assert!(rect.contains(Point2f::new(90.0, 70.0))); // Bottom-right corner
        assert!(!rect.contains(Point2f::new(5.0, 40.0))); // Outside left
        assert!(!rect.contains(Point2f::new(95.0, 40.0))); // Outside right
    }

    #[test]
    fn test_camera_board_socket_conversion() {
        let socket = CameraBoardSocket::CamA;
        let c_value = socket.to_c();
        let back_converted = CameraBoardSocket::from_c(c_value).unwrap();
        assert_eq!(socket, back_converted);
    }

    #[test]
    fn test_camera_sensor_config() {
        let config = CameraSensorConfig::new(
            1920, 1080, 30.0, 60.0, 
            Rect::new_normalized(0.0, 0.0, 1.0, 1.0),
            CameraSensorType::Color
        );
        
        assert_eq!(config.resolution(), (1920, 1080));
        assert_eq!(config.pixel_count(), 1920 * 1080);
        assert!(config.supports_fps(45.0));
        assert!(!config.supports_fps(70.0));
        assert!(!config.supports_fps(25.0));
    }

    #[test]
    fn test_camera_features() {
        let mut features = CameraFeatures::new(CameraBoardSocket::CamA, "IMX378".to_string());
        features.width = 4056;
        features.height = 3040;
        features.supported_types.push(CameraSensorType::Color);
        
        assert_eq!(features.max_resolution(), (4056, 3040));
        assert!(features.supports_type(CameraSensorType::Color));
        assert!(!features.supports_type(CameraSensorType::Mono));
    }

    #[test]
    fn test_version_comparison() {
        let v1 = Version::new(2, 1, 0);
        let v2 = Version::new(2, 0, 5);
        let v3 = Version::new(2, 1, 0);
        
        assert!(v1.is_newer_than(&v2));
        assert!(!v2.is_newer_than(&v1));
        assert!(!v1.is_newer_than(&v3));
        assert!(!v3.is_newer_than(&v1));
        
        assert_eq!(v1.version_string(), "2.1.0");
    }

    #[test]
    fn test_usb_speed() {
        let speed = UsbSpeed::Super;
        assert_eq!(speed.description(), "SuperSpeed (5 Gbps)");
        
        let c_value = speed.to_c();
        let back_converted = UsbSpeed::from_c(c_value).unwrap();
        assert_eq!(speed, back_converted);
    }
}
