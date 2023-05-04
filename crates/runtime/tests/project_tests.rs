//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/ProjectTests.cs>
//!
//! The following tests test behavior that is currently out of scope for this project and were thus omitted:
//! - TestDeclarationFilesAreGenerated: Tests functionality that, quote "Is intended to be called by tools that let the user manage variable declarations."
//! - AddTagsToLines: Tests functionality that, quote: "Given Yarn source code, adds line tags to the ends of all lines that need one and do not already have one."
//!   This is only used in a certain section of the Unity project importer.

use crate::test_base::*;
use yarn_slinger_compiler::prelude::*;

mod test_base;

#[test]
#[ignore]
fn test_loading_nodes() {
    todo!("Not ported yet")
}

#[test]
fn test_debug_output_is_produced() {
    let file = File {
        file_name: "input".to_owned(),
        source: create_test_node_with_name("This is a test node.", "DebugTesting"),
    };
    let compilation_job = CompilationJob::default().with_file(file);

    let result = compile(compilation_job).unwrap_pretty();

    // We should have a single DebugInfo object, because we compiled a single node
    assert_eq!(1, result.debug_info.len());

    // The first instruction of the only node should begin on the third line
    println!("{:?}", result.debug_info);
    let first_line_info = result.debug_info.values().next().unwrap().get_line_info(0);

    assert_eq!("input", first_line_info.file_name);
    assert_eq!("DebugTesting", first_line_info.node_name);
    assert_eq!(2, first_line_info.position.unwrap().line);
    assert_eq!(0, first_line_info.position.unwrap().character);
}
