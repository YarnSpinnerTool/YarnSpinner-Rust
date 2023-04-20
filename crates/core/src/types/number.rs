//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/NumberType.cs>

use crate::prelude::types::type_util::*;
use crate::prelude::types::TypeProperties;
use crate::prelude::*;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// A type that bridges to [`f32`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NumberType;

impl TypeProperties for NumberType {
    const NAME: &'static str = "Number";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo.to_string() => f32::eq_by_value,
            Operator::NotEqualTo.to_string() => f32::ne_by_value,
            Operator::Add.to_string() => <f32 as Add>::add,
            Operator::Subtract.to_string() => <f32 as Sub>::sub,
            Operator::Multiply.to_string() => <f32 as Mul>::mul,
            Operator::Divide.to_string() => <f32 as Div>::div,
            Operator::Modulo.to_string() => <f32 as Rem>::rem,
            Operator::UnarySubtract.to_string() => f32::neg,
            Operator::GreaterThan.to_string() => f32::gt_by_value,
            Operator::GreaterThanOrEqualTo.to_string() => f32::ge_by_value,
            Operator::LessThan.to_string() => f32::lt_by_value,
            Operator::LessThanOrEqualTo.to_string() => f32::le_by_value,
        }
    }
}
