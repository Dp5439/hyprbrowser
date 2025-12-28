use std::fs;
use std::path::Path;

fn main() {
    // Ensure output directories exist
    let dist_dir = Path::new("dist");
    let assets_src = Path::new("assets");
    let assets_dist = dist_dir.join("assets");

    let _ = fs::create_dir_all(dist_dir);
    let _ = fs::create_dir_all(&assets_dist);

    // Copy assets
    if assets_src.exists() {
        for entry in fs::read_dir(assets_src).unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let filename = path.file_name().unwrap();
                    let dest = assets_dist.join(filename);
                    let _ = fs::copy(&path, dest);
                }
            }
        }
    }

    // Set rustc flags for optimization
    println!("cargo:rustc-env=RUSTFLAGS=-C opt-level=3 -C lto");
}
