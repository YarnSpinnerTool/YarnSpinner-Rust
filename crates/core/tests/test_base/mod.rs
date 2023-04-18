//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.3.0/YarnSpinner.Tests/TestBase.cs#L49>
//! Not implemented yet:
//! - pretty much anything lol

use std::path::PathBuf;

pub struct TestBase {}

impl TestBase {
    pub fn project_root_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    pub fn test_data_path() -> PathBuf {
        Self::project_root_path().join("assets")
    }
}
