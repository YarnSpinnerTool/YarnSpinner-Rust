//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::prelude::*;
use crate::types::TypeProperties;
use core::ops::*;

/// A type that bridges to [`bool`]
pub(crate) fn boolean_type_properties() -> TypeProperties {
    TypeProperties::from_name("Bool").with_methods(yarn_library! {
        Operator::EqualTo => <RustType as PartialEq>::eq,
        Operator::NotEqualTo => <RustType as PartialEq>::ne,
        Operator::And => <RustType as BitAnd>::bitand,
        Operator::Or => <RustType as BitOr>::bitor,
        Operator::Xor => <RustType as BitXor>::bitxor,
        Operator::Not => RustType::not,
    })
}

type RustType = bool;
