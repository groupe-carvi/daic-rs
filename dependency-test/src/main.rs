use depthai::Device;

fn main() {
    println!("Testing depthai dependency resolution...");
    // Just try to call a function or use a type to ensure it compiles
    let result = Device::new();
    match result {
        Ok(_) => println!("Successfully created Device (or attempted to)"),
        Err(e) => println!("Note: Device creation failed (expected if no hardware is present): {}", e),
    }
}
