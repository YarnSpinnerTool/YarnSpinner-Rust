#![allow(dead_code)]
//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TestBase.cs#L49>
//! Not implemented yet:
//! - pretty much anything lol

use std::path::PathBuf;

pub struct TestBase {}

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
}
