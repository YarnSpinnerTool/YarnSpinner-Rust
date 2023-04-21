//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BuiltinTypes.cs>

use crate::prelude::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains the built-in types available in the Yarn language.
/// A strict subset of [`Type`].
pub enum BuiltinType {
    /// The type representing any value.
    Any(AnyType),
    /// The type representing numbers.
    Number(NumberType),
    /// The type representing strings.
    String(StringType),
    /// The type representing boolean values.
    Bool(BooleanType),
    /// An undefined type.
    ///
    /// This value is not valid except during compilation. It
    /// is used to represent values that have not yet been assigned a
    /// type by the type system.
    Undefined,
}
