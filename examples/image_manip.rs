use std::time::Duration;

use depthai::camera::{CameraNode, CameraOutputConfig};
use depthai::common::{CameraBoardSocket, ImageFrameType, ResizeMode};
use depthai::{
    Colormap, ImageManipNode, Pipeline, RerunHostNode, RerunHostNodeConfig, RerunViewer,
    RerunWebConfig, Result,
};

/// Example demonstrating the `ImageManip` node, visualized using `RerunHostNode`.
///
/// This example:
/// - captures a camera stream
/// - applies an ImageManip initial configuration (crop/resize/format)
/// - logs the manipulated frames to Rerun (Web viewer by default)
fn main() -> Result<()> {
    const CAM_W: u32 = 640;
    const CAM_H: u32 = 480;

    // Use an implicit/default device (keeps the example short).
    let pipeline = Pipeline::new()?;

    // Camera (built immediately by create_with).
    let cam = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;

    // IMPORTANT: don't feed RAW frames into ImageManip.
    //
    // `cam.raw()` is typically RAW10 (ImageFrameType value 17), and many devices/firmware
    // variants don't support RAW inputs for ImageManip (they error with "Frame type 17 not supported").
    //
    // Instead, request a processed stream from the camera (NV12 is widely supported) and then
    // do crop/resize/format conversion via the explicit `ImageManip` node.
    //
    // We also request a modest output size to keep device-side buffer requirements reasonable.
    let cam_out = cam.request_output(CameraOutputConfig {
        // Keep ImageManip output at the same resolution as this input stream.
        size: (CAM_W, CAM_H),
        frame_type: Some(ImageFrameType::NV12),
        resize_mode: ResizeMode::Crop,
        fps: Some(30.0),
        enable_undistortion: None,
    })?;

    // Image manipulation node.
    let manip = pipeline.create::<ImageManipNode>()?;
    manip.set_num_frames_pool(4);
    // DepthAI allocates output buffers based on this limit.
    // If it's larger than the actual image payload, some backends will report the full
    // allocated buffer length, which then triggers RerunHostNode's "buffer larger than expected"
    // truncation note.
    //
    // Our output is CAM_W x CAM_H RGB888i => CAM_W * CAM_H * 3 bytes.
    // Keeping this equal to the expected payload helps avoid host-side
    // warnings caused by padded allocations.
    manip.set_max_output_frame_size((CAM_W * CAM_H * 3) as i32);

    // Configure the node's initial config.
    // (This handle is shared with the node; mutations affect the pipeline.)
    let mut cfg = manip.initial_config()?;
    cfg.clear_ops()
        // Rotate the image 180 degrees.
        .add_rotate_deg(180.0)
        // Set output frame format.
        // NOTE: Some devices/firmware variants don't support BGR888i output from ImageManip.
        // RGB888i is widely supported and works with the RerunHostNode.
        .set_frame_type(ImageFrameType::RGB888i)
        // Optional visualization: apply a colormap (mostly useful for grayscale).
        .set_colormap(Colormap::None)
        .set_undistort(false);

    // Link camera -> ImageManip.
    cam_out.link(&manip.inputImage()?)?;

    // Visualize using Rerun.
    let host = pipeline.create_with::<RerunHostNode, _>(RerunHostNodeConfig {
        entity_path: "image_manip".to_string(),
        viewer: RerunViewer::Web(RerunWebConfig {
            // In VS Code remote/containers, auto-opening a browser is often confusing.
            // We'll print the URL instead.
            open_browser: false,
            ..Default::default()
        }),
        ..Default::default()
    })?;
    manip.out()?.link(&host.input("in")?)?;

    // Start the pipeline.
    pipeline.start()?;

    eprintln!("image_manip running (press Ctrl-C to stop)...");
    eprintln!("If the web viewer can't fetch data, make sure the gRPC /proxy port (default 9876) is reachable from your browser (e.g. port-forward it if you're remote).");
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
