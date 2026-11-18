pub mod project;
pub use project::*;

/// The `ysc` release this crate's compiled-file formats are tested against.
///
/// Matches the `YarnSpinner.Compiler.dll` version printed by `ysc version`. The `ysc version`
/// line itself is unreliable: the v2.5.0 tag of YarnSpinner-Console still reports `2.4.0`.
pub const SUPPORTED_YSC_VERSION: &str = "2.5.0";
