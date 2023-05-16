//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/StringType.cs>

use crate::prelude::*;
use crate::types::TypeProperties;

/// A type that bridges to [`String`]
pub(crate) fn string_type_properties() -> TypeProperties {
    TypeProperties::from_name("String").with_methods(yarn_fn_registry! {
        Operator::EqualTo => <RustType as PartialEq>::eq,
        Operator::NotEqualTo => <RustType as PartialEq>::ne,
        Operator::Add => |a: RustType, b: RustType| a + &b,
    })
}

type RustType = String;
