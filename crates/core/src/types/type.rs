use crate::prelude::types::*;
use crate::prelude::{YarnFn, YarnFnRegistry};
use paste::paste;
use std::any::Any;
use std::fmt::Debug;

/// All types in the virtual machine, both built-in, i.e. usable in yarn scripts, and internal.
///
/// ## Implementation Notes
///
/// This type does not exist in the original implementation and was a added as a more idiomatic
/// representation of the types than dynamic dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Default, strum_macros::Display)]
pub enum Type {
    Any(AnyType),
    Boolean(BooleanType),
    Function(FunctionType),
    Number(NumberType),
    String(StringType),
    /// An undefined type.
    ///
    /// This value is not valid except during compilation. It
    /// is used to represent values that have not yet been assigned a
    /// type by the type system.
    #[default]
    Undefined,
}

/// Defines properties that describe a type in the Yarn language.
///
/// ## Implementation Notes
///
/// - Represents the `IType` interface in the original implementation.
/// - `Parent` is not implemented because it is set to `AnyType` everywhere anyways.
///
pub trait TypeProperties: Clone + PartialEq + Eq + Debug {
    /// The Rust type that this type represents. Since the [`Value`] type converts freely between
    /// most types, Yarn scripts accept inputs that Rust wouldn't accept for this type.
    ///
    /// ## Implementation Notes
    ///
    /// Not part of the original implementation, but added for clearer code.
    type RustType;

    /// The name of this type.
    const NAME: &'static str;

    /// A more verbose description of this type.
    const DESCRIPTION: &'static str = Self::NAME;

    /// The collection of methods that are available on this type.
    fn methods() -> YarnFnRegistry {
        Default::default()
    }
}

// The following is implemented on [`BuiltinTypes`] in the original implementation, but implementing it
// on [`Type`] results in more compile-time safety.

macro_rules! impl_type {
    ($($yarn_type:pat => [$($base_type:path,)*] ,)*) => {
        $(
            $(

                paste! {
                    /// Convenience trait for getting a [`Type`] out of a base type.
                    #[allow(non_camel_case_types)]
                    pub trait [<$base_type Ext>] {
                        /// Get the corresponding [`Type`]
                        fn r#type() -> Type;
                    }
                    impl [<$base_type Ext>] for $base_type {
                        fn r#type() -> Type {
                            $yarn_type(Default::default())
                        }
                    }

                    impl From<&$base_type> for Type {
                        fn from(_value: &$base_type) -> Self {
                            $yarn_type(Default::default())
                        }
                    }

                    impl From<$base_type> for Type {
                        fn from(_value: $base_type) -> Self {
                            $yarn_type(Default::default())
                        }
                    }
                }
            )*
        )*
    };
}

impl From<BuiltinType> for Type {
    fn from(value: BuiltinType) -> Self {
        match value {
            BuiltinType::Any(any) => Type::Any(any),
            BuiltinType::Boolean(boolean) => Type::Boolean(boolean),
            BuiltinType::Number(number) => Type::Number(number),
            BuiltinType::String(string) => Type::String(string),
            BuiltinType::Undefined => Type::Undefined,
        }
    }
}

impl_type! {
    Type::Number => [f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize, NumberType,],
    Type::String => [String, StringType,],
    Type::Boolean => [bool, BooleanType,],
}

// The macro has problems with the following expansions

pub trait StrRefExt {
    fn r#type() -> Type;
}

impl StrRefExt for &str {
    fn r#type() -> Type {
        Type::String(Default::default())
    }
}

impl From<&str> for Type {
    fn from(_value: &str) -> Self {
        Type::String(Default::default())
    }
}

pub trait AnyExt {
    fn r#type() -> Type;
}

impl AnyExt for Box<dyn Any> {
    fn r#type() -> Type {
        Type::Any(Default::default())
    }
}

impl From<&Box<dyn Any>> for Type {
    fn from(_value: &Box<dyn Any>) -> Self {
        Type::Any(Default::default())
    }
}

pub trait YarnFnExt {
    fn r#type() -> Type;
}

impl YarnFnExt for Box<dyn YarnFn> {
    fn r#type() -> Type {
        Type::Function(Default::default())
    }
}

impl From<&Box<dyn YarnFn>> for Type {
    fn from(_value: &Box<dyn YarnFn>) -> Self {
        Type::Function(Default::default())
    }
}
