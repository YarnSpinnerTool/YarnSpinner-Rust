//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BuiltinTypes.cs>

use crate::prelude::types::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, strum_macros::Display)]
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
    Boolean(BooleanType),
    /// An undefined type.
    ///
    /// This value is not valid except during compilation. It
    /// is used to represent values that have not yet been assigned a
    /// type by the type system.
    #[default]
    Undefined,
}

impl BuiltinType {
    pub const EXPLICITLY_CONSTRUCTABLE: &'static [BuiltinType] = &[
        BuiltinType::Any(AnyType),
        BuiltinType::Number(NumberType),
        BuiltinType::String(StringType),
        BuiltinType::Boolean(BooleanType),
        // Undefined types are not explicitly constructable
    ];
}

impl TryFrom<Type> for BuiltinType {
    type Error = ();

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        match value {
            Type::Any(any) => Ok(BuiltinType::Any(any)),
            Type::Boolean(boolean) => Ok(BuiltinType::Boolean(boolean)),
            Type::Number(number) => Ok(BuiltinType::Number(number)),
            Type::String(string) => Ok(BuiltinType::String(string)),
            Type::Undefined => Ok(BuiltinType::Undefined),
            Type::Function(_) => Err(()),
        }
    }
}

impl From<AnyType> for BuiltinType {
    fn from(any: AnyType) -> Self {
        BuiltinType::Any(any)
    }
}

impl From<BooleanType> for BuiltinType {
    fn from(boolean: BooleanType) -> Self {
        BuiltinType::Boolean(boolean)
    }
}

impl From<NumberType> for BuiltinType {
    fn from(number: NumberType) -> Self {
        BuiltinType::Number(number)
    }
}

impl From<StringType> for BuiltinType {
    fn from(string: StringType) -> Self {
        BuiltinType::String(string)
    }
}
