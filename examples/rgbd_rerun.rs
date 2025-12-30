use std::error::Error;
use std::time::Duration;

use depthai::camera::{CameraNode, CameraOutputConfig};
use depthai::common::{CameraBoardSocket, ImageFrameType, ResizeMode};
use depthai::pipeline::Pipeline;
use depthai::{DepthUnit, Device, DevicePlatform, ImageAlignNode, RgbdNode, StereoDepthNode, StereoPresetMode};
use depthai::pointcloud::rgba32_from_rgba;
use depthai::{RerunHostNode, RerunHostNodeConfig, RerunViewer, RerunWebConfig};

use rerun as rr;

fn main() -> Result<(), Box<dyn Error>> {
    // Use the existing `RerunHostNode` to serve the web viewer + gRPC server.
    // This is headless-friendly and does NOT require Wayland/X11.
    //
    // By default the host node prints the URLs; for SSH, port-forward 9090 and 9876.

    // Create a single device connection and bind the pipeline to it.
    let device = Device::new()?;
    let platform = device.platform()?;
    let is_rvc4 = matches!(platform, DevicePlatform::Rvc4);

    // Controls (via environment variables):
    // - DEPTHAI_DISABLE_POINTCLOUD=1|0
    //
    // Notes:
    // - Pointcloud generation can be heavy; It can be disabled to improve performance.
    let disable_pointclound = std::env::var("DEPTHAI_DISABLE_POINTCLOUD")
        .ok()
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);

    // Keep the pipeline lighter on RVC4 to improve the odds of starting successfully.
    // Note: StereoDepth has constraints around stride/width alignment; widths divisible by 128
    // are broadly safe (e.g. 640, 1280).
    let (frame_w, frame_h) = if is_rvc4 { (640, 400) } else { (640, 400) };
    let fps = if is_rvc4 { 15.0 } else { 30.0 };

    // The IR dot projector is useful on many OAK stereo devices.
    // (No-op on devices that don't support it.)
    let _ = device.set_ir_laser_dot_projector_intensity(0.3);

    let pipeline = Pipeline::new().with_device(&device).build()?;

    // Start the web viewer server + gRPC server inside the pipeline (host-side).
    // Note: we use a separate `app_id` so this infrastructure stream doesn't collide with the
    // recording we produce below.
    let _web = pipeline.create_with::<RerunHostNode, _>(RerunHostNodeConfig {
        app_id: "depthai_rgbd_server".to_string(),
        viewer: RerunViewer::Web(RerunWebConfig {
            // In remote/SSH setups, auto-opening a browser is rarely desirable.
            open_browser: false,
            ..Default::default()
        }),
        ..Default::default()
    })?;

    // Connect a recording stream to the gRPC /proxy endpoint exposed by the host node.
    // This avoids spawning any GUI viewer and keeps the example usable over SSH.
    let rec = rr::RecordingStreamBuilder::new("depthai_rgbd").connect_grpc()?;

    // Cameras: typical OAK-D layout (CamA = color, CamB/CamC = mono stereo).
    let cam_color = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    let cam_left = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamB)?;
    let cam_right = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamC)?;

    let out_color = cam_color.request_output(CameraOutputConfig {
        size: (frame_w, frame_h),
        frame_type: Some(ImageFrameType::RGB888i),
        resize_mode: ResizeMode::Crop,
        fps: Some(fps),
        enable_undistortion: None,
    })?;

    let out_left = cam_left.request_output(CameraOutputConfig {
        size: (frame_w, frame_h),
        frame_type: Some(ImageFrameType::GRAY8),
        resize_mode: ResizeMode::Crop,
        fps: Some(fps),
        enable_undistortion: None,
    })?;

    let out_right = cam_right.request_output(CameraOutputConfig {
        size: (frame_w, frame_h),
        frame_type: Some(ImageFrameType::GRAY8),
        resize_mode: ResizeMode::Crop,
        fps: Some(fps),
        enable_undistortion: None,
    })?;

    // Stereo depth.
    let stereo = pipeline.create::<StereoDepthNode>()?;
    stereo.set_default_profile_preset(if is_rvc4 {
        StereoPresetMode::Default
    } else {
        StereoPresetMode::Robotics
    });
    stereo.set_left_right_check(!is_rvc4);
    // Ensure depth output matches the expected RGB size.
    // (By default, some presets/platforms may downscale the depth output.)
    stereo.set_output_size(frame_w as i32, frame_h as i32);
    stereo.set_output_keep_aspect_ratio(true);

    out_left.link_to(stereo.as_node(), Some("left"))?;
    out_right.link_to(stereo.as_node(), Some("right"))?;

    // Depth output from StereoDepth.
    let depth_out = stereo.as_node().output("depth")?;

    // Align depth to color.
    // - Non-RVC4: use `StereoDepth.inputAlignTo` (device-side, supported).
    // - RVC4: `StereoDepth.inputAlignTo` is unsupported; use `ImageAlign`, but run it on the host
    //   to avoid device DSP failures.
    let depth_to_rgbd = if is_rvc4 {
        let align = pipeline.create::<ImageAlignNode>()?;
        align.set_run_on_host(true);
        align.set_output_size(frame_w as i32, frame_h as i32);
        align.set_out_keep_aspect_ratio(true);

        depth_out.link_to(align.as_node(), Some("input"))?;
        out_color.link_to(align.as_node(), Some("inputAlignTo"))?;

        align.as_node().output("outputAligned")?
    } else {
        out_color.link_to(stereo.as_node(), Some("inputAlignTo"))?;
        depth_out
    };

    // RGBD host node: combines RGB + depth into point cloud + paired frames.
    let rgbd = pipeline.create::<RgbdNode>()?;
    rgbd.set_depth_unit(DepthUnit::Meter);
    rgbd.build_ex(
        false,
        if is_rvc4 { StereoPresetMode::Default } else { StereoPresetMode::Robotics },
        (frame_w as i32, frame_h as i32),
        Some(fps),
    )?;

    out_color.link_to(rgbd.as_node(), Some("inColorSync"))?;
    depth_to_rgbd.link_to(rgbd.as_node(), Some("inDepthSync"))?;

    // Output queues.
    // Optional debugging: print the *actual* frame sizes coming out of the pipeline.
    // Enable with `DEPTHAI_DEBUG_SIZES=1`.
    let debug_sizes = std::env::var("DEPTHAI_DEBUG_SIZES")
        .ok()
        .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false);
    let q_dbg_color = if debug_sizes {
        Some(out_color.create_queue(2, false)?)
    } else {
        None
    };
    let q_dbg_depth = if debug_sizes {
        Some(depth_to_rgbd.create_queue(2, false)?)
    } else {
        None
    };

    let q_pcl = if !disable_pointclound {
        Some(rgbd.as_node().output("pcl")?.create_queue(2, false)?)
    } else {
        eprintln!("Pointcloud logging disabled (set DEPTHAI_DISABLE_POINTCLOUD=1 to enable)");
        None
    };
    let q_rgbd = rgbd.as_node().output("rgbd")?.create_queue(2, false)?;

    pipeline.start()?;

    if let (Some(qc), Some(qd)) = (q_dbg_color.as_ref(), q_dbg_depth.as_ref()) {
        // Grab a couple of frames to avoid racing at startup.
        for i in 0..2 {
            let c = qc.blocking_next(Some(Duration::from_millis(500)))?;
            let d = qd.blocking_next(Some(Duration::from_millis(500)))?;
            eprintln!("debug_sizes[{i}]: color={:?} depth={:?}", c.as_ref().map(|f| f.describe()), d.as_ref().map(|f| f.describe()));
        }
    }

    let mut frame_nr: i64 = 0;

    loop {
        rec.set_time_sequence("frame", frame_nr);
        frame_nr += 1;

        // Pull RGBD frames.
        if let Some(rgbd_msg) = q_rgbd.blocking_next_rgbd(Some(Duration::from_millis(200)))? {
            let rgb = rgbd_msg.rgb_frame()?;

            let w = rgb.width();
            let h = rgb.height();
            let bytes = rgb.bytes();

            // Log RGB image.
            rec.log("rgb", &rr::Image::from_rgb24(bytes, [w, h]))?;
        }

        // Pull point cloud.
        if let Some(q_pcl) = q_pcl.as_ref() {
            if let Some(pcl) = q_pcl.try_next_pointcloud()? {
                // Downsample to keep logging responsive.
                let pts = pcl.points();
                let step = 4usize;

                let mut positions = Vec::with_capacity(pts.len() / step + 1);
                let mut colors = Vec::with_capacity(pts.len() / step + 1);

                for p in pts.iter().step_by(step) {
                    if !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite() {
                        continue;
                    }
                    if p.z == 0.0 {
                        continue;
                    }

                    positions.push(rr::Position3D::from([p.x, p.y, p.z]));
                    colors.push(rr::Color::from(rr::Rgba32(rgba32_from_rgba(p.r, p.g, p.b, p.a))));
                }

                rec.log("pcl", &rr::Points3D::new(positions).with_colors(colors))?;
            }
        }
    }
}
