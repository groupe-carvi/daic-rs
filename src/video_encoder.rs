use autocxx::c_int;

use depthai_sys::depthai;

use crate::common::ImageFrameType;
use crate::encoded_frame::validate_nv12_dimensions;
use crate::error::{clear_error_flag, take_error_if_any, Result};
use crate::output::Input;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoEncoderRateControlMode {
    Cbr = 0,
    Vbr = 1,
}

impl VideoEncoderRateControlMode {
    pub fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Cbr),
            1 => Some(Self::Vbr),
            _ => None,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoEncoderProfile {
    H264Baseline = 0,
    H264High = 1,
    H264Main = 2,
    H265Main = 3,
    Mjpeg = 4,
}

impl VideoEncoderProfile {
    pub fn from_raw(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::H264Baseline),
            1 => Some(Self::H264High),
            2 => Some(Self::H264Main),
            3 => Some(Self::H265Main),
            4 => Some(Self::Mjpeg),
            _ => None,
        }
    }
}

#[crate::native_node_wrapper(
    native = "dai::node::VideoEncoder",
    outputs(bitstream, out)
)]
pub struct VideoEncoderNode {
    node: crate::pipeline::Node,
}

impl VideoEncoderNode {
    /// Returns the input port.
    ///
    /// DepthAI's VideoEncoder input port is named `"in"` (keyword in Rust), so we expose it
    /// as `input()`.
    ///
    /// Note: VideoEncoder expects NV12 `ImgFrame`s.
    pub fn input(&self) -> Result<Input> {
        self.as_node().input("in")
    }

    /// Convenience helper: validate typical NV12 invariants before configuring camera/manip.
    pub fn validate_nv12_size(&self, width: u32, height: u32) -> Result<()> {
        validate_nv12_dimensions(width, height)
    }

    /// The required input frame type for the encoder.
    pub fn required_input_type(&self) -> ImageFrameType {
        ImageFrameType::NV12
    }

    pub fn set_default_profile_preset(&self, fps: f32, profile: VideoEncoderProfile) {
        clear_error_flag();
        unsafe {
            depthai::dai_video_encoder_set_default_profile_preset(self.node.handle(), fps, c_int(profile as i32))
        };
    }

    pub fn set_num_frames_pool(&self, frames: i32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_num_frames_pool(self.node.handle(), c_int(frames)) };
    }

    pub fn num_frames_pool(&self) -> Result<i32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_num_frames_pool(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get num frames pool") {
            Err(err)
        } else {
            Ok(v.into())
        }
    }

    pub fn set_rate_control_mode(&self, mode: VideoEncoderRateControlMode) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_rate_control_mode(self.node.handle(), c_int(mode as i32)) };
    }

    pub fn rate_control_mode(&self) -> Result<VideoEncoderRateControlMode> {
        clear_error_flag();
        let raw = unsafe { depthai::dai_video_encoder_get_rate_control_mode(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get rate control mode") {
            return Err(err);
        }
        Ok(VideoEncoderRateControlMode::from_raw(raw.into()).unwrap_or(VideoEncoderRateControlMode::Cbr))
    }

    pub fn set_profile(&self, profile: VideoEncoderProfile) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_profile(self.node.handle(), c_int(profile as i32)) };
    }

    pub fn profile(&self) -> Result<VideoEncoderProfile> {
        clear_error_flag();
        let raw = unsafe { depthai::dai_video_encoder_get_profile(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get encoder profile") {
            return Err(err);
        }
        Ok(VideoEncoderProfile::from_raw(raw.into()).unwrap_or(VideoEncoderProfile::H264Baseline))
    }

    pub fn set_bitrate(&self, bitrate_bps: i32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_bitrate(self.node.handle(), c_int(bitrate_bps)) };
    }

    pub fn bitrate(&self) -> Result<i32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_bitrate(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get bitrate") {
            Err(err)
        } else {
            Ok(v.into())
        }
    }

    pub fn set_bitrate_kbps(&self, bitrate_kbps: i32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_bitrate_kbps(self.node.handle(), c_int(bitrate_kbps)) };
    }

    pub fn bitrate_kbps(&self) -> Result<i32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_bitrate_kbps(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get bitrate (kbps)") {
            Err(err)
        } else {
            Ok(v.into())
        }
    }

    pub fn set_keyframe_frequency(&self, freq: i32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_keyframe_frequency(self.node.handle(), c_int(freq)) };
    }

    pub fn keyframe_frequency(&self) -> Result<i32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_keyframe_frequency(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get keyframe frequency") {
            Err(err)
        } else {
            Ok(v.into())
        }
    }

    pub fn set_num_bframes(&self, num_bframes: i32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_num_bframes(self.node.handle(), c_int(num_bframes)) };
    }

    pub fn num_bframes(&self) -> Result<i32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_num_bframes(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get num b-frames") {
            Err(err)
        } else {
            Ok(v.into())
        }
    }

    pub fn set_quality(&self, quality: i32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_quality(self.node.handle(), c_int(quality)) };
    }

    pub fn quality(&self) -> Result<i32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_quality(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get quality") {
            Err(err)
        } else {
            Ok(v.into())
        }
    }

    pub fn set_lossless(&self, lossless: bool) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_lossless(self.node.handle(), lossless) };
    }

    pub fn lossless(&self) -> Result<bool> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_lossless(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get lossless") {
            Err(err)
        } else {
            Ok(v)
        }
    }

    pub fn set_frame_rate(&self, frame_rate: f32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_frame_rate(self.node.handle(), frame_rate) };
    }

    pub fn frame_rate(&self) -> Result<f32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_frame_rate(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get frame rate") {
            Err(err)
        } else {
            Ok(v)
        }
    }

    pub fn set_max_output_frame_size(&self, max_frame_size: i32) {
        clear_error_flag();
        unsafe { depthai::dai_video_encoder_set_max_output_frame_size(self.node.handle(), c_int(max_frame_size)) };
    }

    pub fn max_output_frame_size(&self) -> Result<i32> {
        clear_error_flag();
        let v = unsafe { depthai::dai_video_encoder_get_max_output_frame_size(self.node.handle()) };
        if let Some(err) = take_error_if_any("failed to get max output frame size") {
            Err(err)
        } else {
            Ok(v.into())
        }
    }
}
