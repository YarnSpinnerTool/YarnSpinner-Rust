use crate::prelude::types::*;
use crate::prelude::{Library, YarnFn};
use paste::paste;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

/// All types in the virtual machine, both built-in, i.e. usable in yarn scripts, and internal.
///
/// ## Implementation Notes
///
/// This type does not exist in the original implementation and was a added as a more idiomatic
/// representation of the types than dynamic dispatch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Any(AnyType),
    Boolean(BooleanType),
    Function(FunctionType),
    Number(NumberType),
    String(StringType),
}

/// Defines properties that describe a types in the Yarn language.
///
/// ## Implementation Notes
///
/// - Represents the `IType` interface in the original implementation.
/// - `Parent` is not implemented because it is set to `AnyType` everywhere anyways.
///
pub trait TypeProperties: Clone + PartialEq + Eq + Debug {
    /// The name of this types.
    const NAME: &'static str;

    /// A more verbose description of this types.
    const DESCRIPTION: &'static str = Self::NAME;

    /// The collection of methods that are available on this types.
    ///
    /// ## Implementation Notes
    ///
    /// This was an `IDictionary<string, Delegate>` in the original implementation, but using this
    const METHODS: Library = Default::default();
}

// The following is implemented on [`BuiltinTypes`] in the original implementation, but implementing it
// on [`Type`] results in more compile time safety.

macro_rules! impl_type {
    ($($yarn_type:pat => [$($base_type:path,)*] ,)*) => {
        $(
            $(

                paste! {
                    /// Convenience trait for getting a [`Type`] out of a base types.
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
                }
            )*
        )*
    };
}

impl_type! {
    Type::Number => [f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,],
    Type::String => [String,],
    Type::Boolean => [bool,],
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

impl From<Box<dyn Any>> for Type {
    fn from(_value: Box<dyn Any>) -> Self {
        Type::Any(Default::default())
    }
}
