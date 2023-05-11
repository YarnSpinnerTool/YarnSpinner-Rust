//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::prelude::types::TypeProperties;
use crate::prelude::*;
use std::ops::*;

/// A type that bridges to [`bool`]
pub(crate) fn boolean_type_properties() -> TypeProperties {
    TypeProperties::from_name("Bool").with_methods(yarn_fn_registry! {
        Operator::EqualTo => RustType::eq,
        Operator::NotEqualTo => RustType::ne,
        Operator::And => <RustType as BitAnd>::bitand,
        Operator::Or => <RustType as BitOr>::bitor,
        Operator::Xor => <RustType as BitXor>::bitxor,
        Operator::Not => RustType::not,
    })
}

type RustType = bool;
