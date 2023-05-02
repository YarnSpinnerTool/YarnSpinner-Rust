//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/LanguageTests.cs>

use crate::test_base::TestBase;
use yarn_slinger_compiler::prelude::*;
use yarn_slinger_core::prelude::Program;

mod test_base;

#[test]
#[ignore]
fn test_example_script() {
    todo!("Not ported yet")
}

#[test]
fn test_merging_nodes() -> std::io::Result<()> {
    let test_base = TestBase::default();
    let sally_path = TestBase::space_demo_scripts_path().join("Sally.yarn");
    let ship_path = TestBase::space_demo_scripts_path().join("Ship.yarn");

    let compilation_job_sally = CompilationJob::default()
        .read_file(&sally_path)?
        .with_library(test_base.dialogue.library.clone());
    let compilation_job_sally_and_ship = CompilationJob::default()
        .read_file(&sally_path)?
        .read_file(&ship_path)?
        .with_library(test_base.dialogue.library.clone());

    let result_sally = compile(compilation_job_sally).unwrap();
    let result_sally_and_ship = compile(compilation_job_sally_and_ship).unwrap();

    // Loading code with the same contents should throw
    let combined_not_working = Program::combine(vec![
        result_sally.program.unwrap(),
        result_sally_and_ship.program.unwrap(),
    ]);
    assert!(combined_not_working.is_none());
    Ok(())
}

#[test]
#[ignore]
fn test_end_of_notes_with_options_not_added() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_node_headers() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_invalid_characters_in_node_title() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_number_plurals() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_compilation_should_not_be_culture_dependent() {
    todo!("Not ported yet")
}

#[test]
#[ignore]
fn test_sources() {
    todo!("Not ported yet")
}
