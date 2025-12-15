/// Example demonstrating the generic `Pipeline::create*` API.
///
/// This example shows:
/// - typed creation (`CameraNode`)
/// - and how to build a small “composite” using generic nodes (`NodeKind` + `Node::link`)
use daic_rs::camera::{CameraNode, CameraOutputConfig};
use daic_rs::common::CameraBoardSocket;
use daic_rs::device::Device;
use daic_rs::pipeline::{NodeKind, Pipeline};
use daic_rs::Result;

struct RgbdComposite {
    _rgbd: daic_rs::pipeline::Node,
}

impl daic_rs::pipeline::CreateInPipeline for RgbdComposite {
    fn create(pipeline: &Pipeline) -> Result<Self> {
        // Minimal demo: create a couple nodes and link a default output -> default input.
        // (Port names are highly node-specific; this uses DepthAI default ports.)
        let cam = pipeline.create_node(NodeKind::Camera)?;
        let rgbd = pipeline.create_node(NodeKind::Rgbd)?;
        cam.link(None, None, &rgbd, None, None)?;
        Ok(Self { _rgbd: rgbd })
    }
}

fn main() -> Result<()> {
    println!("Creating pipeline with generic create() API...");
    
    // Create pipeline
    let pipeline = Pipeline::new()?;
    let device = Device::new()?;
    
    // Using the generic create_with API for creating camera nodes
    let left = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamB)?;
    let right = pipeline.create_with::<CameraNode, _>(CameraBoardSocket::CamC)?;
    
    println!("Created left camera node");
    println!("Created right camera node");
    
    // Configure camera outputs
    let left_config = CameraOutputConfig::new((640, 400));
    let _left_output = left.request_output(left_config)?;
    
    let right_config = CameraOutputConfig::new((640, 400));
    let _right_output = right.request_output(right_config)?;
    
    println!("Configured camera outputs");

    // Create a Rust composite (built from generic nodes)
    let _composite = pipeline.create::<RgbdComposite>()?;
    println!("Created composite node");
    
    // Start the pipeline
    pipeline.start(&device)?;
    println!("Pipeline started successfully!");
    
    Ok(())
}
