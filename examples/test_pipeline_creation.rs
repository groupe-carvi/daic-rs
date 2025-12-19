use depthai::Pipeline;

fn main() {
    match Pipeline::new() {
        Ok(_) => println!("Pipeline created successfully"),
        Err(e) => println!("Failed to create pipeline: {:?}", e),
    }
}
