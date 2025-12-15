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

fn main() -> Result<()> {
    println!("Creating pipeline with generic create() API...");
    
    // Create device (single connection)
    let device = Device::new()?;

    // Create pipeline bound to that device (matches DepthAI C++ `Pipeline(device)`)
    let pipeline = Pipeline::with_device(&device)?;
    
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

    // Demonstrate the generic node API: create a StereoDepth node and link the cameras into it.
    // (StereoDepth expects inputs named "left" and "right".)
    let stereo = pipeline.create_node(NodeKind::StereoDepth)?;
    left.as_node().link(None, None, &stereo, None, Some("left"))?;
    right.as_node().link(None, None, &stereo, None, Some("right"))?;
    println!("Linked cameras into StereoDepth");
    
    // Start the pipeline (mirrors DepthAI C++: pipeline.start())
    pipeline.start()?;
    println!("Pipeline started successfully!");
    
    Ok(())
}
