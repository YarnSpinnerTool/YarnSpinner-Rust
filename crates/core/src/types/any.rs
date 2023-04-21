//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/AnyType.cs>

use crate::prelude::types::TypeProperties;
use std::any::Any;

/// Represents any type. this type is used in circumstances when a type
/// is known to have a value, but the specific type is not known or
/// required to be known.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AnyType;

impl TypeProperties for AnyType {
    type RustType = Box<dyn Any>;
    const NAME: &'static str = "Any";
    const DESCRIPTION: &'static str = "Any type.";
}
