//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/ProjectTests.cs>
//!
//! The following tests test behavior that is currently out of scope for this project and were thus omitted:
//! - TestDeclarationFilesAreGenerated: Tests functionality that, quote "Is intended to be called by tools that let the user manage variable declarations."
//! - AddTagsToLines: Tests functionality that, quote: "Given Yarn source code, adds line tags to the ends of all lines that need one and do not already have one."
//!   This is only used in a certain section of the Unity project importer.

use test_base::prelude::*;
use yarn_slinger::prelude::compiler::*;

mod test_base;

#[test]
fn test_loading_nodes() {
    let path = test_data_path().join("Projects/Basic/Test.yarn");
    let result = Compiler::new().read_file(path).compile().unwrap();

    let dialogue = TestBase::default().with_compilation(result).dialogue;

    // high-level test: load the file, verify it has the nodes we want,
    // and run one

    assert_eq!(3, dialogue.node_names().unwrap().len());
    assert!(dialogue.node_exists("TestNode"));
    assert!(dialogue.node_exists("AnotherTestNode"));
    assert!(dialogue.node_exists("ThirdNode"));
}

#[test]
fn test_debug_output_is_produced() {
    let file = File {
        file_name: "input".to_owned(),
        source: create_test_node_with_name("This is a test node.", "DebugTesting"),
    };
    let result = Compiler::new().add_file(file).compile().unwrap();

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
