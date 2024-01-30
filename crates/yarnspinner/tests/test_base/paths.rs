//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TestBase.cs>

use std::path::PathBuf;

pub fn create_test_node(source: &str) -> String {
    create_test_node_with_name(source, "Start")
}

pub fn create_test_node_with_name(source: &str, name: &str) -> String {
    format!("title: {name}\n---\n{source}\n===")
}

pub fn project_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn test_data_path() -> PathBuf {
    project_root_path().join("../../third-party/YarnSpinner/Tests")
}

pub fn space_demo_scripts_path() -> PathBuf {
    test_data_path().join("Projects/Space")
}
