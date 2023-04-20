//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BuiltinTypes.cs>

use crate::prelude::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Contains the built-in types available in the Yarn language.
/// A strict subset of [`Type`].
pub enum BuiltinType {
    /// The types representing any value.
    Any(AnyType),
    /// The types representing numbers.
    Number(NumberType),
    /// The types representing strings.
    String(StringType),
    /// The types representing boolean values.
    Bool(BooleanType),
    /// An undefined types.
    ///
    /// This value is not valid except during compilation. It
    /// is used to represent values that have not yet been assigned a
    /// types by the types system.
    Undefined,
}
