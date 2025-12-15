use std::time::Duration;

use daic_rs::camera::{CameraBoardSocket, CameraOutputConfig};
use daic_rs::device::{Device};
use daic_rs::pipeline::Pipeline;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::new()?;
    let pipeline = Pipeline::new()?;

    let camera = pipeline.create_camera(CameraBoardSocket::Auto)?;
    let preview = camera.request_output(CameraOutputConfig::new((640, 400)))?;
    let queue = preview.create_queue(8, false)?;

    pipeline.start_with_device(&device)?;

    for index in 0..100 {
        match queue.blocking_next(Some(Duration::from_millis(500)))? {
            Some(frame) => {
                println!("Frame #{index}: {}", frame.describe());
            }
            None => {
                println!("Frame #{index}: timeout");
            }
        }
    }

    Ok(())
}
