use std::io::Result;
fn main() -> Result<()> {
    const PROTO_FILE: &str = "src/output/yarn_spinner.proto";
    println!("cargo:rerun-if-changed={PROTO_FILE}");
    prost_build::compile_protos(&[PROTO_FILE], &["src/compiler"])?;
    Ok(())
}
