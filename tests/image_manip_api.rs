use depthai::{Colormap, ImageManipConfig, ImageManipNode, ImageManipResizeMode, Pipeline, Result};
use depthai::common::ImageFrameType;

#[cfg(feature = "hit")]
#[test]
fn image_manip_api_smoke() -> Result<()> {
    let pipeline = Pipeline::new().build()?;

    // Node creation via generic `pipeline.create::<T>()`.
    let manip = pipeline.create::<ImageManipNode>()?;

    // Node property setters.
    manip.set_num_frames_pool(4);
    manip.set_max_output_frame_size(1024 * 1024);

    // Initial config access + mutation (shared with the node).
    let mut initial = manip.initial_config()?;
    initial
        .clear_ops()
        .add_crop_xywh(0, 0, 100, 100)
        .set_output_size(300, 300, ImageManipResizeMode::CenterCrop)
        .set_output_center(true)
        .set_background_color_rgb(0, 0, 0)
        .set_colormap(Colormap::None)
        .set_frame_type(ImageFrameType::BGR888i)
        .set_undistort(false);

    // Standalone config message.
    let mut cfg = ImageManipConfig::new()?;
    cfg.add_rotate_deg(15.0)
        .add_flip_horizontal()
        .add_transform_affine([1.0, 0.0, 0.0, 1.0]);

    // The config must be usable as a generic Buffer message.
    let _as_buffer = cfg.as_buffer();

    Ok(())
}
