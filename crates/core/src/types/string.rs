//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/StringType.cs>

use crate::function_wrappers::*;
use crate::prelude::types::TypeProperties;
use crate::prelude::*;

/// A type that bridges to [`String`]
pub(crate) fn string_type_properties() -> TypeProperties {
    TypeProperties::from_name("String").with_methods(yarn_fn_registry! {
        Operator::EqualTo => RustType::eq_by_value,
        Operator::NotEqualTo => RustType::ne_by_value,
        Operator::Add => |a: RustType, b: RustType| a + &b,
    })
}

type RustType = String;
