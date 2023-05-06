//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TestBase.cs>

// Allowed because this is a common file and not all tests use the methods provided.
// Everything is actually used, but the checker doesn't recognize it because all integration test files
// are separately compiled as their own crate.
#![allow(dead_code)]

use crate::prelude::*;
use yarn_slinger::prelude::*;

mod extensions;
mod paths;
mod step;
mod test_plan;

pub mod prelude {
    pub use crate::test_base::{extensions::*, paths::*, step::*, test_plan::*, *};
}

#[derive(Debug, Clone, Default)]
pub struct TestBase {
    pub dialogue: Dialogue,
    pub test_plan: TestPlan,
}
