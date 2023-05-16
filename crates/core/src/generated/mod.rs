#![allow(warnings)]
#![allow(clippy)]
//! Equivalent to <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.cs>

use crate::prelude::*;
mod ext;
pub use self::ext::*;

include!("yarn.rs");
