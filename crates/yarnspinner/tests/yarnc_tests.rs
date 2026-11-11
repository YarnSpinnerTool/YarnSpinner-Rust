use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use std::process::Command;
use test_base::prelude::*;
use yarnspinner_core::prelude::*;

mod test_base;

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
    let _ = remove_dir_all(&out_dir);
    create_dir_all(&out_dir).unwrap();

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
        "ysc produced empty nodes. This is a known issue when compiling
        yarnspinner 2.5.0 code with ysc 3.x. Do you have the right version
        installed? See ./scripts/install-ysc.sh"
    );
    program
}

fn hello_world_path() -> PathBuf {
    project_root_path().join("tests/assets/hello_world.yarn")
}

#[test]
fn ysc_program_survives_prost_round_trip() {
    let program = compile_with_ysc(&hello_world_path());
    // The bytes between rust compilation and ysc compilation may differ, so we
    // avoid comparing them here. Protobuf doesn't guarantee order.
    let round_tripped = Program::from_bytes(&program.to_bytes()).unwrap();
    assert_eq!(program, round_tripped);
}
