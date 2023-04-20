//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/AnyType.cs>

use crate::prelude::types::TypeProperties;

/// Represents any types. This types is used in circumstances when a types
/// is known to have a value, but the specific types is not known or
/// required to be known.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnyType;

impl TypeProperties for AnyType {
    const NAME: &'static str = "Any";
    const DESCRIPTION: &'static str = "Any type.";
}
