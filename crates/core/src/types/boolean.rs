//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::prelude::types::{type_util::*, TypeProperties};
use crate::prelude::*;
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// A type that bridges to [`bool`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BooleanType;

impl TypeProperties for BooleanType {
    const NAME: &'static str = "Bool";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo => bool::eq_by_value,
            Operator::NotEqualTo => bool::ne_by_value,
            Operator::And => <bool as BitAnd>::bitand,
            Operator::Or => <bool as BitOr>::bitor,
            Operator::Xor => <bool as BitXor>::bitxor,
            Operator::Not => bool::not,
        }
    }
}
