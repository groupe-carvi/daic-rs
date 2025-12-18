use std::error::Error;
use std::time::Duration;

use depthai::camera::{CameraNode, CameraOutputConfig};
use depthai::common::{CameraBoardSocket, ImageFrameType, ResizeMode};
use depthai::pipeline::{NodeKind, Pipeline};
use depthai::{DepthUnit, Device, DevicePlatform, Output, RgbdNode, StereoDepthNode, StereoPresetMode};
use depthai::pointcloud::rgba32_from_rgba;

use rerun as rr;

fn main() -> Result<(), Box<dyn Error>> {
    // Start a rerun recording.
    let rec = rr::RecordingStreamBuilder::new("depthai_rgbd").spawn()?;

    // Create a single device connection and bind the pipeline to it.
    let device = Device::new()?;
    let platform = device.platform()?;

    // The IR dot projector is useful on many OAK stereo devices.
    // (No-op on devices that don't support it.)
    let _ = device.set_ir_laser_dot_projector_intensity(0.3);

    let pipeline = Pipeline::with_device(&device)?;

    // Cameras: typical OAK-D layout (CamA = color, CamB/CamC = mono stereo).
    let cam_color = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    let cam_left = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamB)?;
    let cam_right = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamC)?;

    let out_color = cam_color.request_output(CameraOutputConfig {
        size: (640, 400),
        frame_type: Some(ImageFrameType::RGB888i),
        resize_mode: ResizeMode::Crop,
        fps: Some(30.0),
        enable_undistortion: None,
    })?;

    let out_left = cam_left.request_output(CameraOutputConfig {
        size: (640, 400),
        frame_type: Some(ImageFrameType::GRAY8),
        resize_mode: ResizeMode::Crop,
        fps: Some(30.0),
        enable_undistortion: None,
    })?;

    let out_right = cam_right.request_output(CameraOutputConfig {
        size: (640, 400),
        frame_type: Some(ImageFrameType::GRAY8),
        resize_mode: ResizeMode::Crop,
        fps: Some(30.0),
        enable_undistortion: None,
    })?;

    // Stereo depth.
    let stereo = pipeline.create::<StereoDepthNode>()?;
    stereo.set_default_profile_preset(StereoPresetMode::Robotics);
    stereo.set_left_right_check(true);

    out_left.link_to(stereo.as_node(), Some("left"))?;
    out_right.link_to(stereo.as_node(), Some("right"))?;

    // Optionally align depth to the RGB camera.
    let aligned_depth_out: Output = match platform {
        DevicePlatform::Rvc4 => {
            // On some platforms the dedicated ImageAlign node is preferred.
            let align = pipeline.create_node(NodeKind::ImageAlign)?;
            let depth_out = stereo.as_node().output("depth")?;
            depth_out.link_to(&align, Some("input"))?;
            out_color.link_to(&align, Some("inputAlignTo"))?;
            align.output("outputAligned")?
        }
        _ => {
            // StereoDepth can align internally using inputAlignTo.
            out_color.link_to(stereo.as_node(), Some("inputAlignTo"))?;
            stereo.as_node().output("depth")?
        }
    };

    // RGBD host node: combines RGB + depth into point cloud + paired frames.
    let rgbd = pipeline.create::<RgbdNode>()?;
    rgbd.set_depth_unit(DepthUnit::Meter);
    rgbd.build()?;

    out_color.link_to(rgbd.as_node(), Some("inColorSync"))?;
    aligned_depth_out.link_to(rgbd.as_node(), Some("inDepthSync"))?;

    // Output queues.
    let q_pcl = rgbd.as_node().output("pcl")?.create_queue(2, false)?;
    let q_rgbd = rgbd.as_node().output("rgbd")?.create_queue(2, false)?;

    pipeline.start()?;

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
