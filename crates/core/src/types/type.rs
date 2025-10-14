use crate::prelude::*;
use crate::types::any::any_type_properties;
use crate::types::boolean::boolean_type_properties;
use crate::types::number::number_type_properties;
use crate::types::string::string_type_properties;
use crate::types::*;
use core::any::TypeId;
use core::error::Error;
use core::fmt::{Debug, Display};

/// All types in the virtual machine, both built-in, i.e. usable in Yarn scripts, and internal.
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
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Default, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub enum Type {
    #[default]
    /// The type representing any value
    Any,
    /// The type representing booleans
    Boolean,
    /// The type representing functions
    Function(FunctionType),
    /// The type representing numbers
    Number,
    /// The type representing strings
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = self.name();
        match self {
            Type::Function(function) => Display::fmt(function, f),
            _ => write!(f, "{name}"),
        }
    }
}

/// A trait that provides a way to format both [`Type`] and `Option<Type>` as a string.
pub trait TypeFormat {
    /// Formats this type as a string.
    fn format(&self) -> String;
}

impl TypeFormat for Option<Type> {
    fn format(&self) -> String {
        if let Some(r#type) = self {
            r#type.format()
        } else {
            "undefined".to_string()
        }
    }
}

impl TypeFormat for Type {
    fn format(&self) -> String {
        self.to_string()
    }
}

impl Type {
    /// Returns the name of this type.
    pub fn name(&self) -> &'static str {
        self.properties().name
    }

    /// Returns a more verbose description of this type.
    pub fn description(&self) -> String {
        self.properties().description
    }

    /// Returns the methods of this that can be called from Yarn scripts.
    pub fn methods(&self) -> Library {
        self.properties().methods
    }

    fn properties(&self) -> TypeProperties {
        match self {
            Type::Any => any_type_properties(),
            Type::Boolean => boolean_type_properties(),
            Type::Function(function_type) => function_type_properties(function_type),
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
        self.methods().contains_function(name)
    }

    /// Does not check whether the method exists. Use [`Type::has_method`] for that.
    pub fn get_canonical_name_for_method(&self, method_name: &str) -> String {
        format!("{}.{}", self.name(), method_name)
    }

    /// The types that can be explicitly constructed in Yarn with variable assignments.
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
pub(crate) struct TypeProperties {
    /// The name of this type.
    pub name: &'static str,

    /// A more verbose description of this type.
    pub description: String,

    /// The collection of methods that are available on this type.
    pub methods: Library,
}

impl TypeProperties {
    pub(crate) fn from_name(name: &'static str) -> Self {
        Self {
            name,
            description: name.to_owned(),
            methods: Default::default(),
        }
    }

    pub(crate) fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub(crate) fn with_methods(mut self, methods: Library) -> Self {
        self.methods = methods;
        self
    }
}

// The following is implemented on [`Types`] in the original implementation, but implementing it
// on [`Type`] results in more compile-time safety.

/// A trait that assigns Rust values a Yarn [`Type`].
pub trait TypedValue {
    #[allow(missing_docs)]
    fn r#type(&self) -> Type;
}

macro_rules! impl_typed_value {
    ($([$($rust_type:ty),* $(,)?] => $yarn_type:expr), *$(,)?) => {
        $(
            $(
                impl TypedValue for $rust_type {
                    fn r#type(&self) -> Type {
                        $yarn_type
                    }
                }
            )*
        )*
    };
}

impl_typed_value! {
    [f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize] => Type::Number,
    [String, str] => Type::String,
    [bool] => Type::Boolean,
}

macro_rules! type_ids {
    ($($type:ty),*) => {
        type_ids![$($type,)*]
    };
    ($($type:ty,)*) => {
        vec![$(TypeId::of::<$type>(), TypeId::of::<&$type>()),*]
    };
}

impl TryFrom<TypeId> for Type {
    type Error = InvalidDowncastError;

    fn try_from(type_id: TypeId) -> Result<Self, Self::Error> {
        let string_types = type_ids![String, &str];
        let bool_types = type_ids![bool];
        let value_types = type_ids![YarnValue];
        let number_types = type_ids![
            f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,
        ];

        [
            (string_types, Type::String),
            (bool_types, Type::Boolean),
            (number_types, Type::Number),
            (value_types, Type::Any),
        ]
        .into_iter()
        .find_map(|(type_ids, r#type)| type_ids.contains(&type_id).then_some(r#type))
        .ok_or(InvalidDowncastError::InvalidTypeId(type_id))
    }
}

impl TypedValue for YarnValue {
    fn r#type(&self) -> Type {
        match self {
            YarnValue::Number(_) => Type::Number,
            YarnValue::String(_) => Type::String,
            YarnValue::Boolean(_) => Type::Boolean,
        }
    }
}

#[derive(Debug)]
/// Represents a failure to dynamically convert a [`TypeId`] to a [`Type`].
#[allow(missing_docs)]
pub enum InvalidDowncastError {
    InvalidTypeId(TypeId),
}

impl Error for InvalidDowncastError {}

impl Display for InvalidDowncastError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            InvalidDowncastError::InvalidTypeId(id) => {
                write!(f, "Cannot convert TypeId {id:?} to a Yarn Spinner `Type`")
            }
        }
    }
}
