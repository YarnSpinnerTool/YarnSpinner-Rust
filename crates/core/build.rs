use std::io::Result;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let include_dir: &Path = "../../third-party/YarnSpinner/YarnSpinner".as_ref();
    let proto_file: PathBuf = include_dir.join("yarn_spinner.proto");
    let file_string = proto_file.to_str().unwrap();
    println!("cargo:rerun-if-changed={file_string}");
    prost_build::compile_protos(&[proto_file], &[include_dir])?;
    Ok(())
}
