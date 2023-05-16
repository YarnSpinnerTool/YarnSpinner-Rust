//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/AnyType.cs>

use crate::types::TypeProperties;

/// Represents any type. this type is used in circumstances when a type
/// is known to have a value, but the specific type is not known or
/// required to be known.
pub(crate) fn any_type_properties() -> TypeProperties {
    TypeProperties::from_name("Any").with_description("Any type.")
}
