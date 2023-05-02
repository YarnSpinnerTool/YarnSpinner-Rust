//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Library.cs>

use crate::prelude::*;
use std::ops::{Deref, DerefMut};

/// A collection of functions that can be called from Yarn scripts.
///
/// You do not create instances of this class yourself. The [`Dialogue`]
/// class creates one for you, and you can access it through the
/// [`Library`] property.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Library(pub YarnFnRegistry);

impl Deref for Library {
    type Target = YarnFnRegistry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Library {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Library {
    /// Loads functions from another [`Library`].
    ///
    /// If the other library contains a function with the same name as
    /// one in this library, the function in the other library takes
    /// precedence.
    pub fn import(&mut self, other: Self) {
        self.0.extend(other.0 .0);
    }

    /// Generates a unique tracking variable name.
    /// This is intended to be used to generate names for visiting.
    /// Ideally these will very reproducible and sensible.
    /// For now it will be something terrible and easy.
    pub fn generate_unique_visited_variable_for_node(node_name: &str) -> String {
        format!("$Yarn.Internal.Visiting.{node_name}")
    }

    pub fn standard_library() -> Self {
        let mut library = Library(yarn_fn_registry!(
            "string" => <String as From<UntypedValue >>::from,
            "number" => |value: UntypedValue| f32::try_from(value).expect("Failed to convert a Yarn value to a number"),
            "bool" => |value: UntypedValue| bool::try_from(value).expect("Failed to convert a Yarn value to a bool"),
        ));
        for r#type in [Type::Number, Type::String, Type::Boolean] {
            library.register_methods(r#type);
        }
        library
    }

    /// Registers the methods found inside a type.
    fn register_methods(&mut self, r#type: Type) {
        for (name, function) in r#type.properties().methods.iter() {
            let canonical_name = r#type.get_canonical_name_for_method(name);
            self.add_boxed(canonical_name, function.clone());
        }
    }
}
