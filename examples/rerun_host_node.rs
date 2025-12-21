use std::time::Duration;

use depthai::camera::{CameraBoardSocket, CameraNode, CameraOutputConfig, ImageFrameType};
use depthai::{Pipeline, RerunHostNode, RerunHostNodeConfig, RerunViewer, RerunWebConfig, Result};

fn main() -> Result<()> {
    let pipeline = Pipeline::new()?;
    let camera = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    // Important: the default frame_type is "native" (often NV12), which the RerunHostNode
    // intentionally ignores. Request an explicit RGB output so it can be logged.
    let out = camera.request_output(CameraOutputConfig {
        frame_type: Some(ImageFrameType::RGB888i),
        ..CameraOutputConfig::new((640, 400))
    })?;

    let host = pipeline.create_with::<RerunHostNode, _>(RerunHostNodeConfig {
        // In many dev setups (VS Code remote/containers), auto-opening a browser is confusing.
        // We'll print the URL instead.
        viewer: RerunViewer::Web(RerunWebConfig {
            open_browser: false,
            ..Default::default()
        }),
        ..Default::default()
    })?;
    out.link(&host.input("in")?)?;

    pipeline.start()?;
    eprintln!("rerun_host_node running (press Ctrl-C to stop)...");
    eprintln!("If the web viewer can't fetch data, make sure the gRPC /proxy port (default 9876) is reachable from your browser (e.g. port-forward it if you're remote).");
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
