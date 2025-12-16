use std::time::Duration;

use daic_rs::camera::{CameraNode, CameraOutputConfig};
use daic_rs::common::{CameraBoardSocket, ImageFrameType, ResizeMode};
use daic_rs::device::Device;
use daic_rs::pipeline::Pipeline;
use daic_rs::Result;

fn main() -> Result<()> {
    let device = Device::new()?;
    let pipeline = Pipeline::with_device(&device)?;

    let cam = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    let out = cam.request_output(CameraOutputConfig {
        size: (640, 400),
        frame_type: Some(ImageFrameType::RGB888i),
        resize_mode: ResizeMode::Crop,
        fps: Some(30.0),
        enable_undistortion: None,
    })?;

    let q = out.create_queue(4, false)?;

    pipeline.start()?;

    for _ in 0..10 {
        if let Some(frame) = q.blocking_next(Some(Duration::from_millis(200)))? {
            println!("Got frame: {} ({} bytes)", frame.describe(), frame.byte_len());
        } else {
            println!("No frame yet");
        }
    }

    Ok(())
}
