use std::time::Duration;

use depthai::camera::{CameraBoardSocket, CameraNode, CameraOutputConfig};
use depthai::{Pipeline, RerunHostNode, RerunHostNodeConfig, Result};

fn main() -> Result<()> {
    let pipeline = Pipeline::new()?;
    let camera = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    let out = camera.request_output(CameraOutputConfig::new((640, 400)))?;

    let host = pipeline.create_with::<RerunHostNode, _>(RerunHostNodeConfig::default())?;
    out.link(&host.input("in")?)?;

    pipeline.start()?;
    std::thread::sleep(Duration::from_secs(5));
    Ok(())
}
