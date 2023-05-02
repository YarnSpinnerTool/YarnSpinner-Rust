#![allow(dead_code)]
//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TestBase.cs#L49>
//! Not implemented yet:
//! - pretty much anything lol

use std::path::PathBuf;
use yarn_slinger_runtime::prelude::*;

pub struct TestBase {
    pub dialogue: Dialogue,
}

impl Default for TestBase {
    fn default() -> Self {
        let variable_storage = MemoryVariableStore::default();
        let dialogue = Dialogue::with_variable_storage(variable_storage);
        Self { dialogue }
    }
}

impl TestBase {
    pub fn project_root_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    pub fn test_data_path() -> PathBuf {
        let project_root_path = Self::project_root_path();
        let project_root = project_root_path.to_str().unwrap();
        [
            project_root,
            "..",
            "..",
            "third-party",
            "YarnSpinner",
            "Tests",
        ]
        .iter()
        .collect()
    }

    pub fn space_demo_scripts_path() -> PathBuf {
        let test_data_path = Self::test_data_path();
        let test_data = test_data_path.to_str().unwrap();
        [test_data, "Projects", "Space"].iter().collect()
    }
}
