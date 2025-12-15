#![cfg(feature = "hit")]

/// Hardware Integration Tests for the generic pipeline.create() API
///
/// These tests require a working DepthAI runtime + device, and are disabled by default.
#[cfg(test)]
mod pipeline_create_tests {
    use daic_rs::camera::CameraNode;
    use daic_rs::common::CameraBoardSocket;
    use daic_rs::pipeline::Pipeline;
    use daic_rs::pipeline::{DeviceNode, DeviceNodeWithParams};

    #[test]
    #[ignore] // Requires hardware
    fn test_create_with_camera_node() {
        let pipeline = Pipeline::new().expect("Failed to create pipeline");
        
        // Using the new generic create_with API for creating camera nodes
        // This is similar to C++: auto camera = pipeline.create<dai::node::Camera>();
        let camera = pipeline
            .create_with::<CameraNode, _>(CameraBoardSocket::CamA)
            .expect("Failed to create camera node");
        
        // Verify we can configure the camera
        use daic_rs::camera::CameraOutputConfig;
        let config = CameraOutputConfig::new((640, 400));
        let _output = camera
            .request_output(config)
            .expect("Failed to request camera output");
    }

    #[test]
    fn test_trait_bounds_compile() {
        // This test ensures the traits are properly defined and can be used
        // in generic contexts. It doesn't run but proves the API compiles.
        fn _generic_create_test<T: DeviceNode>(pipeline: &Pipeline) -> daic_rs::Result<T> {
            pipeline.create::<T>()
        }

        fn _generic_create_with_test<T: DeviceNodeWithParams<P>, P>(
            pipeline: &Pipeline,
            params: P,
        ) -> daic_rs::Result<T> {
            pipeline.create_with::<T, P>(params)
        }
    }
}
