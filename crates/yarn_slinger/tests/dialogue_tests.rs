//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/DialogueTests.cs>
//!
//! ## Implementation notes
//! `TestDumpingCode` was not ported because `GetByteCode` is not used by a user directly and thus was not implemented at all.

use std::sync::{Arc, RwLock};
use test_base::prelude::*;
use yarn_slinger::prelude::*;

mod test_base;

#[test]
fn test_node_exists() {
    let path = space_demo_scripts_path().join("Sally.yarn");
    let test_base = TestBase::new();

    let compilation_job = CompilationJob::new()
        .read_file(path)
        .unwrap()
        .with_library(test_base.library().clone());
    let result = compile(compilation_job).unwrap_pretty();

    let mut dialogue = test_base.dialogue;
    dialogue.replace_program(result.program.unwrap());
    assert!(dialogue.node_exists("Sally"));

    // Test clearing everything
    dialogue.unload_all();
    assert!(!dialogue.node_exists("Sally"));
}

#[test]
#[ignore = "Requires analyzer to be implemented, see https://github.com/yarn-slinger/yarn-slinger/issues/85"]
fn test_analysis() {
    todo!("Not ported yet")
}

#[test]
#[should_panic(expected = "No node named \"THIS NODE DOES NOT EXIST\" has been loaded.")]
fn test_missing_node() {
    let path = test_data_path().join("TestCases").join("Smileys.yarn");

    let compilation_job = CompilationJob::new().read_file(path).unwrap();
    let result = compile(compilation_job).unwrap_pretty();

    let mut test_base = TestBase::new()
        .with_program(result.program.unwrap())
        .with_runtime_errors_do_not_cause_failure();
    test_base.dialogue.set_node("THIS NODE DOES NOT EXIST");
}

#[test]
fn test_getting_current_node_name() {
    let path = space_demo_scripts_path().join("Sally.yarn");
    let test_base = TestBase::new();

    let compilation_job = CompilationJob::new()
        .read_file(path)
        .unwrap()
        .with_library(test_base.library().clone());
    let result = compile(compilation_job).unwrap_pretty();

    let mut dialogue = test_base.dialogue;
    dialogue.replace_program(result.program.unwrap());

    // dialogue should not be running yet
    assert!(dialogue.current_node().is_none());

    dialogue.set_node("Sally");
    assert_eq!(dialogue.current_node(), Some("Sally".to_string()));

    dialogue.stop();
    // Current node should now be null
    assert!(dialogue.current_node().is_none());
}

#[test]
fn test_getting_raw_source() {
    let path = test_data_path().join("Example.yarn");
    let mut test_base = TestBase::new();

    let compilation_job = CompilationJob::new().read_file(path).unwrap();
    let result = compile(compilation_job).unwrap_pretty();

    test_base = test_base.with_compilation(result);
    let dialogue = &test_base.dialogue;

    let source_id = dialogue.get_string_id_for_node("LearnMore").unwrap();
    let source = test_base
        .string_table()
        .get(&LineId(source_id))
        .unwrap()
        .text
        .clone();

    assert_eq!(source, "A: HAHAHA\n");
}

#[test]
fn test_getting_tags() {
    let path = test_data_path().join("Example.yarn");
    let mut test_base = TestBase::new();

    let compilation_job = CompilationJob::new().read_file(path).unwrap();
    let result = compile(compilation_job).unwrap_pretty();

    test_base = test_base.with_program(result.program.unwrap());
    let dialogue = &test_base.dialogue;

    let tags = dialogue.get_tags_for_node("LearnMore").unwrap();

    assert_eq!(tags, vec!["rawText"]);
}

#[test]
fn test_prepare_for_line() {
    let path = test_data_path().join("TaggedLines.yarn");

    let compilation_job = CompilationJob::new().read_file(path).unwrap();
    let result = compile(compilation_job).unwrap_pretty();

    let mut dialogue = TestBase::new().with_compilation(result).dialogue;

    let prepare_for_lines_was_called = Arc::new(RwLock::new(false));
    let prepare_for_lines_was_called_clone = prepare_for_lines_was_called.clone();
    dialogue.set_prepare_for_lines_handler(move |lines, _| {
        // When the Dialogue realises it's about to run the Start
        // node, it will tell us that it's about to run these two
        // line IDs
        assert_eq!(lines.len(), 2);
        assert!(lines.contains(&"line:test1".into()));
        assert!(lines.contains(&"line:test2".into()));

        // Ensure that these asserts were actually called
        *prepare_for_lines_was_called_clone.write().unwrap() = true;
    });

    dialogue.set_node_to_start();

    assert!(*prepare_for_lines_was_called.read().unwrap());
}
