//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/IType.cs>
//! ## Implementation Notes
//! - `IBridgeableType` is not implemented because it is not actually used anywhere.

pub use {
    any::*, boolean::*, builtin_types::*, function::*, number::*, r#type::*, r#type::*, string::*,
};

mod any;
mod boolean;
mod builtin_types;
mod function;
mod number;
mod string;
mod r#type;
pub(crate) mod type_util;
