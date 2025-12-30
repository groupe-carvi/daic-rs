use std::error::Error;
use std::time::Duration;

use depthai::camera::{CameraNode, CameraOutputConfig};
use depthai::common::{CameraBoardSocket, ImageFrameType, ResizeMode};
use depthai::{
    Device, Pipeline, RerunHostNode, RerunHostNodeConfig, RerunViewer, RerunWebConfig,
    VideoEncoderNode, VideoEncoderProfile,
};

use rerun as rr;

fn main() -> Result<(), Box<dyn Error>> {
    // This example streams a live H.265 (HEVC) video stream to Rerun's web viewer.
    //
    // - We use `RerunHostNode` to host the web viewer + gRPC proxy (headless-friendly).
    // - We then connect a `rerun::RecordingStream` to that proxy and log a `VideoStream`.

    // Device (single connection)
    let device = Device::new()?;

    // Pipeline bound to that device
    let pipeline = Pipeline::new().with_device(&device).build()?;

    // Start the web viewer server + gRPC server inside the pipeline (host-side).
    // For remote dev/SSH, port-forward 9090 (web) and 9876 (gRPC proxy).
    let _web = pipeline.create_with::<RerunHostNode, _>(RerunHostNodeConfig {
        app_id: "depthai_h265_server".to_string(),
        viewer: RerunViewer::Web(RerunWebConfig {
            // In remote/SSH setups, auto-opening a browser is rarely desirable.
            open_browser: false,
            ..Default::default()
        }),
        ..Default::default()
    })?;

    // Connect a recording stream to the gRPC /proxy endpoint exposed by the host node.
    let rec = rr::RecordingStreamBuilder::new("depthai_h265").connect_grpc()?;

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

    // Pick a sensible preset for the requested FPS.
    enc.set_default_profile_preset(fps, VideoEncoderProfile::H265Main);

    // Link camera output into encoder input (port name is "in")
    nv12.link(&enc.input()?)?;

    // Create an EncodedFrame output queue.
    let q = enc.out()?.create_encoded_frame_queue(8, true)?;

    // Log the codec once per entity.
    // Samples are then appended over time on the "frame" timeline.
    rec.log_static("video", &rr::VideoStream::new(rr::components::VideoCodec::H265))?;

    // Start the pipeline
    pipeline.start()?;

    eprintln!("Streaming H.265 to Rerun (press Ctrl-C to stop)...");
    eprintln!("If the web viewer can't fetch data, make sure the gRPC /proxy port (default 9876) is reachable from your browser (e.g. port-forward it if you're remote).");

    let mut frame_nr: i64 = 0;

    loop {
        if let Some(frame) = q.blocking_next(Some(Duration::from_millis(500)))? {
            rec.set_time_sequence("frame", frame_nr);
            frame_nr += 1;

            // `VideoSample` takes ownership of the bytes.
            let bytes = frame.bytes();
            rec.log(
                "video",
                &rr::VideoStream::update_fields()
                    .with_sample(rr::components::VideoSample::from(bytes)),
            )?;
        }
    }
}
