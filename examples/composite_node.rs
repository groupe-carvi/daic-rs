use depthai::camera::{CameraNode, CameraBoardSocket};
use depthai::stereo_depth::StereoDepthNode;
use depthai::pipeline::Pipeline;
use depthai::device::Device;
use depthai::{Result, depthai_composite};

/// A composite node that bundles a camera and stereo depth.
#[depthai_composite]
pub struct CameraStereoBundle {
    pub left: CameraNode,
    pub right: CameraNode,
    pub stereo: StereoDepthNode,
}

impl CameraStereoBundle {
    pub fn new(pipeline: &Pipeline) -> Result<Self> {
        // Create native nodes
        let left = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamB)?;
        let right = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamC)?;
        let stereo = pipeline.create::<StereoDepthNode>()?;

        // Link them
        left.isp()?.link(&stereo.left()?)?;
        right.isp()?.link(&stereo.right()?)?;

        Ok(Self { left, right, stereo })
    }
}

fn main() -> Result<()> {
    let device = Device::new()?;
    let pipeline = Pipeline::with_device(&device)?;

    // Create the composite node using the generic API
    let bundle = pipeline.create::<CameraStereoBundle>()?;
    
    println!("Created composite bundle with 2 cameras and 1 stereo node");
    
    pipeline.start()?;
    println!("Pipeline started!");

    Ok(())
}
