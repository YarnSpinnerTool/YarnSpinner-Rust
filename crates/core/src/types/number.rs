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
            Operator::EqualTo => f32::eq_by_value,
            Operator::NotEqualTo => f32::ne_by_value,
            Operator::Add => <f32 as Add>::add,
            Operator::Subtract => <f32 as Sub>::sub,
            Operator::Multiply => <f32 as Mul>::mul,
            Operator::Divide => <f32 as Div>::div,
            Operator::Modulo => <f32 as Rem>::rem,
            Operator::UnarySubtract => f32::neg,
            Operator::GreaterThan => f32::gt_by_value,
            Operator::GreaterThanOrEqualTo => f32::ge_by_value,
            Operator::LessThan => f32::lt_by_value,
            Operator::LessThanOrEqualTo => f32::le_by_value,
        }
    }
}
