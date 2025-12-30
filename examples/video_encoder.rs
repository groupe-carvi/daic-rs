use std::fs::File;
use std::io::Write;
use std::time::Duration;

use depthai::camera::{CameraNode, CameraOutputConfig};
use depthai::common::{CameraBoardSocket, ImageFrameType, ResizeMode};
use depthai::{Device, Pipeline, Result, VideoEncoderNode, VideoEncoderProfile};

fn main() -> Result<()> {
    // Device (single connection)
    let device = Device::new()?;

    // Pipeline bound to that device
    let pipeline = Pipeline::new().with_device(&device).build()?;

    // Camera -> NV12 frames
    let cam = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;

    let fps = 30.0;
    let (w, h) = (640, 400);

    let nv12 = cam.request_output(CameraOutputConfig {
        size: (w, h),
        frame_type: Some(ImageFrameType::NV12),
        resize_mode: ResizeMode::Crop,
        fps: Some(fps),
        enable_undistortion: None,
    })?;

    // Video encoder (expects NV12)
    let enc = pipeline.create::<VideoEncoderNode>()?;
    enc.validate_nv12_size(w, h)?;

    // Pick a sensible profile preset for the requested FPS.
    // (You can override bitrate/quality/etc. after this call.)
    enc.set_default_profile_preset(fps, VideoEncoderProfile::H264Main);

    // Link camera output into encoder input (port name is "in")
    nv12.link(&enc.input()?)?;

    // Create an EncodedFrame output queue.
    let q = enc.out()?.create_encoded_frame_queue(8, true)?;

    // Start the pipeline
    pipeline.start()?;

    // Dump a short sample to disk
    let mut f = File::create("out.h264").unwrap();

    for i in 0..120 {
        if let Some(frame) = q.blocking_next(Some(Duration::from_secs(2)))? {
            let bytes = frame.bytes();
            f.write_all(&bytes).unwrap();
            println!("encoded frame {i}: {}", frame.describe());
        } else {
            println!("timeout waiting for encoded frame {i}");
        }
    }

    println!("Wrote out.h264 (raw bitstream)");
    Ok(())
}
