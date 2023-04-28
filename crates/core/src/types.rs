//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/IType.cs>
//! ## Implementation Notes
//! - `IBridgeableType` is not implemented because it is not actually used anywhere.

pub use {
    any::*, boolean::*, function::*, number::*, r#type::*, r#type::*, string::*, type_util::*,
};

mod any;
mod boolean;
mod function;
mod number;
mod string;
mod r#type;
mod type_util;
