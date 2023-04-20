//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/StringType.cs>

use crate::prelude::types::TypeProperties;

/// A type that bridges to [`String`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StringType;

impl TypeProperties for StringType {
    const NAME: &'static str = "String";
}
