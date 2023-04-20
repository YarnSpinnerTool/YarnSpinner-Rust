//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::prelude::types::{type_util::*, TypeProperties};
use crate::prelude::*;
use crate::yarn_fn_registry;

/// A type that bridges to [`bool`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BooleanType;

impl TypeProperties for BooleanType {
    const NAME: &'static str = "Bool";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo.to_string() => bool::eq_by_value,
        }
    }
}
