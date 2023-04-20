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
            Operator::EqualTo.to_string() => bool::eq_by_value,
            Operator::NotEqualTo.to_string() => bool::ne_by_value,
            Operator::And.to_string() => <bool as BitAnd>::bitand,
            Operator::Or.to_string() => <bool as BitOr>::bitor,
            Operator::Xor.to_string() => <bool as BitXor>::bitxor,
            Operator::Not.to_string() => bool::not,
        }
    }
}
