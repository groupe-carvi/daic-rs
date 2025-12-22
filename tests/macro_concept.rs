use depthai::Pipeline;
use depthai::camera::{CameraNode, CameraBoardSocket};
use depthai::stereo_depth::StereoDepthNode;

#[test]
fn test_new_api_concept() -> depthai::Result<()> {
    // TODO: Create mock so we can test node creation and linking without hardware.
    /*
    let pipeline = Pipeline::new()?;
    
    let cam = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamA)?;
    let stereo = pipeline.create::<StereoDepthNode>()?;
    
    cam.video()?.link(&stereo.left()?)?;
    */

    Ok(())
}
