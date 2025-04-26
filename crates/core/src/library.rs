//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Library.cs>

use crate::prelude::*;
use alloc::borrow::Cow;
use core::fmt::Display;

use hashbrown::hash_map;

/// A collection of functions that can be called from Yarn scripts.
///
/// Can be conveniently created with the [`yarn_library!`] macro.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Library(YarnFnRegistry);

impl Extend<<YarnFnRegistry as IntoIterator>::Item> for Library {
    fn extend<T: IntoIterator<Item = (Cow<'static, str>, Box<dyn UntypedYarnFn>)>>(
        &mut self,
        iter: T,
    ) {
        self.0.extend(iter);
    }
}

impl IntoIterator for Library {
    type Item = (Cow<'static, str>, Box<dyn UntypedYarnFn>);
    type IntoIter = hash_map::IntoIter<Cow<'static, str>, Box<dyn UntypedYarnFn>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Library {
    /// Creates a new empty library. Does not include the functions of [`Library::standard_library`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads functions from another [`Library`].
    ///
    /// Will overwrite any functions that have the same name.
    ///
    /// ## Implementation Notes
    ///
    /// The original implementation throws an exception if a function with the same name already exists.
    pub fn import(&mut self, other: Self) {
        self.0.extend(other.0 .0);
    }

    /// Iterates over the names and functions in the library.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &(dyn UntypedYarnFn))> {
        self.0.iter()
    }

    /// Gets a function by name.
    pub fn get(&self, name: &str) -> Option<&(dyn UntypedYarnFn)> {
        self.0.get(name)
    }

    /// Generates a unique tracking variable name.
    /// This is intended to be used to generate names for visiting.
    /// Ideally these will very reproducible and sensible.
    /// For now it will be something terrible and easy.
    pub fn generate_unique_visited_variable_for_node(node_name: &str) -> String {
        format!("$Yarn.Internal.Visiting.{node_name}")
    }

    /// Creates a [`Library`] with the standard functions that are included in Yarn Spinner.
    /// These are:
    /// - `string`: Converts a value to a string.
    /// - `number`: Converts a value to a number.
    /// - `bool`: Converts a value to a boolean.
    /// - Comparison operators for numbers, strings, and booleans. (`==`, `!=`, `<`, `<=`, `>`, `>=`)
    pub fn standard_library() -> Self {
        let mut library = yarn_library!(
            "string" => <String as From<YarnValue >>::from,
            "number" => |value: YarnValue| f32::try_from(value).expect("Failed to convert a Yarn value to a number"),
            "bool" => |value: YarnValue| bool::try_from(value).expect("Failed to convert a Yarn value to a bool"),
        );
        for r#type in [Type::Number, Type::String, Type::Boolean] {
            library.add_methods(r#type);
        }
        library
    }

    /// Adds a new function to the registry. See [`YarnFn`]'s documentation for what kinds of functions are allowed.
    ///
    /// ## Examples
    /// Registering a function:
    ///
    /// When the `bevy` feature is set it is possible to register Bevy systems as functions.
    /// ```
    /// # use yarnspinner_core::prelude::*;
    /// # let mut library = Library::default();
    /// # use bevy::prelude::*;
    /// # let mut world = World::default();
    /// library.add_function("string_length", world.register_system(how_many_things));
    ///
    /// fn how_many_things(In(thing_type): In<String>, things: Query<&Name>) -> u32 {
    ///     let mut count = 0;
    ///     for name in &things {
    ///         if name.as_str() == thing_type {
    ///             count += 1;
    ///         }
    ///     }
    ///     count
    /// }
    /// ```
    ///
    /// Usage without Bevy:
    ///
    /// ```
    /// # use yarnspinner_core::prelude::*;
    /// # let mut library = Library::default();
    /// library.add_function("string_length", string_length);
    ///
    /// fn string_length(string: String) -> usize {
    ///     string.len()
    /// }
    /// ```
    ///
    /// Registering a function using a factory
    /// (the return type can be specified using the [`yarn_fn_type`] macro):
    /// ```
    /// # use yarnspinner_core::prelude::*;
    /// # let mut library = Library::default();
    /// library.add_function("length_times_two", string_length_multiplied(2));
    ///
    /// fn string_length_multiplied(factor: usize) -> yarn_fn_type! { impl Fn(String) -> usize } {
    ///     move |s: String| s.len() * factor
    /// }
    /// ```
    ///
    pub fn add_function<Marker, F>(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        function: F,
    ) -> &mut Self
    where
        Marker: 'static,
        F: YarnFn<Marker> + 'static + Clone,
        F::Out: IntoYarnValueFromNonYarnValue + 'static + Clone,
    {
        self.0.register_function(name, function);
        self
    }

    /// Returns `true` if the library contains a function with the given name.
    pub fn contains_function(&self, name: &str) -> bool {
        self.0.contains_function(name)
    }

    /// Iterates over the names of all functions in the library.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.0.names()
    }

    /// Iterates over all functions in the library.
    pub fn functions(&self) -> impl Iterator<Item = &(dyn UntypedYarnFn)> {
        self.0.functions()
    }

    /// Registers the methods found inside a type.
    fn add_methods(&mut self, r#type: Type) {
        for (name, function) in r#type.methods().into_iter() {
            let canonical_name = r#type.get_canonical_name_for_method(name.as_ref());
            self.0.add_boxed(canonical_name, function.clone());
        }
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut functions: Vec<_> = self.0.iter().collect();
        functions.sort_by_key(|(name, _)| name.to_string());
        writeln!(f, "{{")?;
        for (name, function) in functions {
            writeln!(f, "    {}: {}", name, function)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

/// Create a [`Library`] from a list of named functions.
///
/// ## Example
///
/// ```rust
/// # use yarnspinner_core::yarn_library;
/// # use yarnspinner_core::prelude::*;
///
/// let library = yarn_library! {
///    "pow" => pow,
/// };
///
/// fn pow(base: f32, exponent: i32) -> f32 {
///    base.powi(exponent)
/// }
///```
#[macro_export]
macro_rules! yarn_library {
    ($($name:expr => $function:expr,)*) => {
        {
            let mut map = Library::default();
            $(
                map.add_function($name, $function);
            )*
            map
        }
    };
}
pub use yarn_library;
