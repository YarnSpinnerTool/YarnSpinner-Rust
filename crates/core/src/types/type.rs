use crate::prelude::types::*;
use crate::prelude::YarnFnRegistry;
use crate::types::any::any_type_properties;
use crate::types::boolean::boolean_type_properties;
use crate::types::number::number_type_properties;
use crate::types::string::string_type_properties;
use paste::paste;
use std::any::Any;
use std::fmt::{Debug, Display};

/// All types in the virtual machine, both built-in, i.e. usable in yarn scripts, and internal.
///
/// Whenever this appears in an `Option` with the `None` type,
/// treat it as an undefined type.
///
/// This value is not valid except during compilation. It
/// is used to represent values that have not yet been assigned a
/// type by the type system.
///
/// ## Implementation Notes
///
/// This type does not exist in the original implementation and was a added as a more idiomatic
/// representation of the types than dynamic dispatch. The `Undefined` "variant", which was a simple `null`,
/// was also replaced by the more idiomatic `Option::None`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Any,
    Boolean,
    Function(FunctionType),
    Number,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Any => write!(f, "any"),
            Type::Boolean => write!(f, "boolean"),
            Type::Function(function) => Display::fmt(function, f),
            Type::Number => write!(f, "number"),
            Type::String => write!(f, "string"),
        }
    }
}

pub trait TypeOptionFormat {
    fn format(&self) -> String;
}

impl TypeOptionFormat for Option<Type> {
    fn format(&self) -> String {
        if let Some(r#type) = self {
            format!("{}", r#type)
        } else {
            "undefined".to_string()
        }
    }
}

impl Type {
    pub fn properties(&self) -> TypeProperties {
        match self {
            Type::Any => any_type_properties(),
            Type::Boolean => boolean_type_properties(),
            Type::Function(_) => function_type_properties(),
            Type::Number => number_type_properties(),
            Type::String => string_type_properties(),
        }
    }

    /// Returns whether this type's methods contain the given method by name.
    ///
    /// ## Implementation notes
    /// Adapted from the `FindImplementingTypeForMethod`, but massively simplified because
    /// we removed type hierarchies.
    pub fn has_method(&self, name: &str) -> bool {
        self.properties().methods.contains_key(name)
    }

    pub const EXPLICITLY_CONSTRUCTABLE: &'static [Type] = &[
        Type::Any,
        Type::Number,
        Type::String,
        Type::Boolean,
        // Functions are not explicitly constructable
    ];
}

/// Defines properties that describe a type in the Yarn language.
///
/// ## Implementation Notes
///
/// - Represents the `IType` interface in the original implementation.
/// - `Parent` is not implemented because it is set to `AnyType` everywhere anyways.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeProperties {
    /// The name of this type.
    pub name: &'static str,

    /// A more verbose description of this type.
    pub description: &'static str,

    /// The collection of methods that are available on this type.
    pub methods: YarnFnRegistry,
}

impl TypeProperties {
    pub fn from_name(name: &'static str) -> Self {
        Self {
            name,
            description: name,
            methods: Default::default(),
        }
    }

    pub fn with_description(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn with_methods(mut self, registry: YarnFnRegistry) -> Self {
        self.methods = registry;
        self
    }
}

// The following is implemented on [`Types`] in the original implementation, but implementing it
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
                            $yarn_type
                        }
                    }

                    impl From<&$base_type> for Type {
                        fn from(_value: &$base_type) -> Self {
                            $yarn_type
                        }
                    }

                    impl From<$base_type> for Type {
                        fn from(_value: $base_type) -> Self {
                            $yarn_type
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
        Type::String
    }
}

impl From<&str> for Type {
    fn from(_value: &str) -> Self {
        Type::String
    }
}

pub trait AnyExt {
    fn r#type() -> Type;
}

impl AnyExt for Box<dyn Any> {
    fn r#type() -> Type {
        Type::Any
    }
}

impl From<&Box<dyn Any>> for Type {
    fn from(_value: &Box<dyn Any>) -> Self {
        Type::Any
    }
}
