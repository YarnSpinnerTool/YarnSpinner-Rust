mod output;
pub mod project;
pub use output::*;
pub use project::*;

/// The supported version of the `ysc`. See https://github.com/YarnSpinnerTool/YarnSpinner-Console
pub const SUPPORTED_YSC_VERSION: &str = "2.5.0";
