use std::time::Duration;

use depthai::camera::{CameraBoardSocket, CameraNode, CameraOutputConfig};
use depthai::{depthai_threaded_host_node, Pipeline, Result, ThreadedHostNodeContext};

#[depthai_threaded_host_node]
struct FrameTap {
    input: depthai::Input,
}

impl FrameTap {
    fn run(&mut self, ctx: &ThreadedHostNodeContext) {
        while ctx.is_running() {
            match self.input.get_frame() {
                Ok(frame) => {
                    println!("threaded host node frame: {}x{}", frame.width(), frame.height());
                }
                Err(_) => break,
            }
        }
    }
}

fn main() -> Result<()> {
    let pipeline = Pipeline::new().build()?;
    let camera = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    let out = camera.request_output(CameraOutputConfig::new((640, 400)))?;

    let host = pipeline.create_threaded_host_node(|node| {
        let input = node.create_input(Some("in"))?;
        Ok(FrameTap { input })
    })?;

    out.link(&host.as_node().input("in")?)?;

    pipeline.start()?;
    std::thread::sleep(Duration::from_secs(2));
    Ok(())
}
