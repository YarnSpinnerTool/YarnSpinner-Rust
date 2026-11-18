use crate::prelude::*;
use std::path::Path;
use std::process::Command;
use yarnspinner::core::CompiledFile;

pub fn compile(inputs: &[&Path]) -> Result<CompiledFile, String> {
    let output_dir = tempfile::tempdir().unwrap();
    let output = Command::new("ysc")
        .arg("compile")
        .args(inputs)
        .arg("-o")
        .arg(output_dir.path())
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to run ysc ({e}). Install it with scripts/install-ysc.sh and add tools/ to the PATH.")
        });

    if !output.status.success() {
        return Err(format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let name = match inputs {
        [input] => input.file_stem().unwrap().to_string_lossy().into_owned(),
        _ => "Output".to_owned(),
    };

    Ok(CompiledFile::read(output_dir.path(), &name).unwrap())
}

pub fn compile_file(path: impl AsRef<Path>) -> CompiledFile {
    let path = path.as_ref();
    compile(&[path]).unwrap_or_else(|e| panic!("Failed to compile {}:\n{e}", path.display()))
}

/// Wraps `source` in a `Start` node and compiles it with `ysc`.
/// cf [`create_test_node`]
pub fn compile_test_source(source: &str) -> Result<CompiledFile, String> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("input.yarn");
    std::fs::write(&path, create_test_node(source)).unwrap();
    compile(&[&path])
}
