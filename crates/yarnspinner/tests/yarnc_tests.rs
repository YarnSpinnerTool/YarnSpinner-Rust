//! Tests that `.yarnc` files produced by the reference compiler (`ysc`) can be loaded,
//! and that they match what the Rust compiler produces for the same source.
//!
//! Requires `ysc` 2.x, see `scripts/install-ysc.sh`.

use std::path::{Path, PathBuf};
use std::process::Command;
use test_base::prelude::*;
use yarnspinner::compiler::*;
use yarnspinner_core::prelude::*;

mod test_base;

/// Prefer the repo-local ysc (built by `scripts/install-ysc.sh`) over whatever is on `PATH`,
/// so a globally installed Yarn Spinner v3 ysc doesn't get picked up by accident.
fn ysc() -> PathBuf {
    let local = project_root_path().join("../../tools/ysc");
    if local.exists() {
        local
    } else {
        PathBuf::from("ysc")
    }
}

/// Compiles `yarn_file` with ysc into a fresh temp dir and returns the decoded [`Program`].
fn compile_with_ysc(yarn_file: &Path) -> Program {
    let out_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("yarnc_tests");
    let _ = std::fs::remove_dir_all(&out_dir);
    std::fs::create_dir_all(&out_dir).unwrap();

    let output = Command::new(ysc())
        .arg("compile")
        .arg(yarn_file)
        .arg("-o")
        .arg(&out_dir)
        .arg("-n")
        .arg("out")
        .output()
        .expect("ysc should be installed, see ./scripts/install-ysc.sh");
    assert!(
        output.status.success(),
        "ysc failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let bytes = std::fs::read(out_dir.join("out.yarnc")).unwrap();
    let program = Program::from_bytes(&bytes).unwrap();
    assert!(
        program.nodes.values().all(|n| !n.instructions.is_empty()),
        "ysc produced nodes without instructions; it is probably Yarn Spinner v3, which is unsupported. \
         See ./scripts/install-ysc.sh"
    );
    program
}

fn hello_world_path() -> PathBuf {
    project_root_path().join("tests/assets/hello_world.yarn")
}

#[test]
fn ysc_program_survives_prost_round_trip() {
    let program = compile_with_ysc(&hello_world_path());
    // Compare decoded values, not bytes: protobuf map order on the wire is unspecified,
    // so prost (sorted BTreeMap) and C# (insertion order) legitimately emit different bytes.
    let round_tripped = Program::from_bytes(&program.to_bytes()).unwrap();
    assert_eq!(program, round_tripped);
}

#[test]
fn ysc_program_matches_rust_compiler() {
    let path = hello_world_path();
    let from_ysc = compile_with_ysc(&path);
    let from_rust = Compiler::new()
        .read_file(&path)
        .compile()
        .unwrap()
        .program
        .unwrap();
    assert_eq!(from_ysc, from_rust);
}
