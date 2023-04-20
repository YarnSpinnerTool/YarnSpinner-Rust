//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::prelude::types::TypeProperties;
use crate::prelude::*;

/// A type that bridges to [`bool`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BooleanType;

impl TypeProperties for BooleanType {
    type RustType = bool;
    const NAME: &'static str = "Bool";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo => Value::eq_by_value::<Self::RustType>,
            Operator::NotEqualTo => Value::ne_by_value::<Self::RustType>,
            Operator::And => Value::and::<Self::RustType>,
            Operator::Or => Value::or::<Self::RustType>,
            Operator::Xor => Value::xor::<Self::RustType>,
            Operator::Not => Value::not::<Self::RustType>,
        }
    }
}
