use std::io::Result;
use std::path::PathBuf;
fn main() -> Result<()> {
    let include_dir_path: PathBuf = ["src", "generated"].iter().collect();
    let include_dir = include_dir_path.to_str().unwrap();
    let proto_file_path: PathBuf = [include_dir, "yarn_spinner.proto"].iter().collect();
    let proto_file = proto_file_path.to_str().unwrap();
    println!("cargo:rerun-if-changed={proto_file}");
    prost_build::compile_protos(&[proto_file], &[include_dir])?;
    Ok(())
}
