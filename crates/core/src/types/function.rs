//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/FunctionType.cs>

use crate::prelude::types::TypeProperties;

/// Todo
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FunctionType;

impl TypeProperties for FunctionType {
    const NAME: &'static str = "Function";
}
