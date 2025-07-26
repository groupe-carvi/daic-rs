use daic_sys::camera::Camera;
use ndarray::Array2;
use rerun::{RecordingStream, MsgSender};
use kornia::filters::gaussian_blur;

fn main() {
    // Initialize the camera
    let camera = match Camera::new() {
        Ok(cam) => cam,
        Err(e) => {
            eprintln!("Camera initialization error: {}", e);
            return;
        }
    };

    // Capture an image
    match camera.capture() {
        Ok(image) => {
            println!("Captured image: {} bytes", image.len());
            // Convert the buffer to ndarray (640x480)
            let arr = Array2::<u8>::from_shape_vec((480, 640), image)
                .expect("Failed to convert to ndarray");

            // Apply gaussian blur with Kornia
            let blurred = gaussian_blur(&arr, (5, 5), (1.0, 1.0));

            // Display the image with Rerun
            let rec = RecordingStream::new("camera_all");
            rec.log_image("image", &blurred);
            println!("Image displayed with Rerun.");
        }
        Err(e) => {
            eprintln!("Capture error: {}", e);
        }
    }
}
