// Simple test build script to verify autocxx works
fn main() {
    println!("cargo:rerun-if-changed=wrapper/simple_test.h");
    println!("cargo:rerun-if-changed=src/simple_test.rs");
    
    let path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    
    // Build with autocxx
    let include_paths = vec![path.join("wrapper")];
    let include_refs: Vec<&std::path::Path> = include_paths.iter().map(|p| p.as_path()).collect();
    
    let mut builder = autocxx_build::Builder::new(
        "src/simple_test.rs",
        &include_refs
    );
    
    let mut build = builder.build().expect("Failed to build autocxx");
    build.flag_if_supported("-std=c++17");
    build.compile("autocxx-simple-test");
    
    println!("cargo:warning=autocxx simple test build completed successfully");
}
