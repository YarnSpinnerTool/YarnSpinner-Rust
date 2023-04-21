//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/StringType.cs>

use crate::prelude::types::{type_util::*, TypeProperties};
use crate::prelude::*;

/// A type that bridges to [`String`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StringType;

impl TypeProperties for StringType {
    type RustType = String;
    const NAME: &'static str = "String";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo => Self::RustType::eq_by_value,
            Operator::NotEqualTo => Self::RustType::ne_by_value,
            Operator::Add => |a: Self::RustType, b: Self::RustType| a + &b,
        }
    }
}
