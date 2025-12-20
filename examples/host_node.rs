use std::time::Duration;

use depthai::camera::{CameraBoardSocket, CameraNode, CameraOutputConfig};
use depthai::{depthai_host_node, Buffer, MessageGroup, Pipeline, Result};

#[depthai_host_node]
struct FrameLogger;

impl FrameLogger {
    fn process(&mut self, group: &MessageGroup) -> Option<Buffer> {
        if let Ok(Some(frame)) = group.get_frame("in") {
            println!("host node frame: {}x{}", frame.width(), frame.height());
        }
        None
    }
}

fn main() -> Result<()> {
    let pipeline = Pipeline::new()?;
    let camera = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    let out = camera.request_output(CameraOutputConfig::new((640, 400)))?;

    let host = pipeline.create_host_node(FrameLogger)?;
    out.link(&host.input("in")?)?;

    pipeline.start()?;
    std::thread::sleep(Duration::from_secs(2));
    Ok(())
}
