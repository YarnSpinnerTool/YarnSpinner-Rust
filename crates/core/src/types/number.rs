//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/NumberType.cs>

use crate::prelude::types::TypeProperties;

/// A type that bridges to [`f32`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NumberType;

impl TypeProperties for NumberType {
    const NAME: &'static str = "Number";
}
