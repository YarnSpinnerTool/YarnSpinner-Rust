//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::prelude::types::{type_util::*, TypeProperties};
use crate::prelude::*;
use std::ops::*;

/// A type that bridges to [`bool`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BooleanType;

impl TypeProperties for BooleanType {
    type RustType = bool;
    const NAME: &'static str = "Bool";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo => Self::RustType::eq_by_value,
            Operator::NotEqualTo => Self::RustType::ne_by_value,
            Operator::And => <Self::RustType as BitAnd>::bitand,
            Operator::Or => <Self::RustType as BitOr>::bitor,
            Operator::Xor => <Self::RustType as BitXor>::bitxor,
            Operator::Not => Self::RustType::not,
        }
    }
}
