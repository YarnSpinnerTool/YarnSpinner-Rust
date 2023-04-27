//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/NumberType.cs>

use crate::function_wrappers::*;
use crate::prelude::types::TypeProperties;
use crate::prelude::*;
use std::ops::*;

/// A type that bridges to [`f32`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NumberType;

impl TypeProperties for NumberType {
    type RustType = f32;
    const NAME: &'static str = "Number";
    fn methods() -> YarnFnRegistry {
        yarn_fn_registry! {
            Operator::EqualTo => Self::RustType::eq_by_value,
            Operator::NotEqualTo => Self::RustType::ne_by_value,
            Operator::Add => <Self::RustType as Add>::add,
            Operator::Subtract => <Self::RustType as Sub>::sub,
            Operator::Multiply => <Self::RustType as Mul>::mul,
            Operator::Divide => <Self::RustType as Div>::div,
            Operator::Modulo => <Self::RustType as Rem>::rem,
            Operator::UnarySubtract => <Self::RustType as Neg>::neg,
            Operator::GreaterThan => Self::RustType::gt_by_value,
            Operator::GreaterThanOrEqualTo => Self::RustType::ge_by_value,
            Operator::LessThan => Self::RustType::lt_by_value,
            Operator::LessThanOrEqualTo => Self::RustType::le_by_value,
        }
    }
}
