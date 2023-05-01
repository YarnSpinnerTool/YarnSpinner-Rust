#![allow(warnings)]
#![allow(clippy)]
//! Equivalent to <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.cs>

include!(concat!(env!("OUT_DIR"), "/yarn.rs"));

mod ext;
pub use ext::*;
