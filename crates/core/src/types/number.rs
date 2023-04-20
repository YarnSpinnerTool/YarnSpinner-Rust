//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/NumberType.cs>

use crate::prelude::types::type_util::*;
use crate::prelude::types::TypeProperties;
use crate::prelude::*;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// A type that bridges to [`f32`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NumberType;

impl TypeProperties for NumberType {
    type RustType = f32;
    const NAME: &'static str = "Number";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo => Value::eq_by_value::<Self::RustType>,
            Operator::NotEqualTo => Value::ne_by_value::<Self::RustType>,
            Operator::Add => Value::add::<Self::RustType>,
            Operator::Subtract => Value::sub::<Self::RustType>,
            Operator::Multiply => Value::mul::<Self::RustType>,
            Operator::Divide => Value::div::<Self::RustType>,
            Operator::Modulo => Value::rem::<Self::RustType>,
            Operator::UnarySubtract => Value::neg::<Self::RustType>,
            Operator::GreaterThan => Value::gt_by_value::<Self::RustType>,
            Operator::GreaterThanOrEqualTo => Value::ge_by_value::<Self::RustType>,
            Operator::LessThan => Value::lt_by_value::<Self::RustType>,
            Operator::LessThanOrEqualTo => Value::le_by_value::<Self::RustType>,
        }
    }
}
