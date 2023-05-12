//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Library.cs>

use crate::prelude::*;
use std::borrow::Cow;
use std::fmt::Display;

/// A collection of functions that can be called from Yarn scripts.
///
/// You do not create instances of this class yourself. The [`Dialogue`]
/// class creates one for you, and you can access it through the
/// [`Library`] property.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Library(YarnFnRegistry);

impl Extend<<YarnFnRegistry as IntoIterator>::Item> for Library {
    fn extend<T: IntoIterator<Item = <YarnFnRegistry as IntoIterator>::Item>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for Library {
    type Item = <YarnFnRegistry as IntoIterator>::Item;
    type IntoIter = <YarnFnRegistry as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }
    /// Loads functions from another [`Library`].
    ///
    /// If the other library contains a function with the same name as
    /// one in this library, the function in the other library takes
    /// precedence.
    pub fn import(&mut self, other: Self) {
        self.0.extend(other.0 .0);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &(dyn UntypedYarnFn + Send + Sync))> {
        self.0.iter()
    }

    pub fn get(&self, name: &str) -> Option<&(dyn UntypedYarnFn + Send + Sync)> {
        self.0.get(name)
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
            "string" => <String as From<YarnValue >>::from,
            "number" => |value: YarnValue| f32::try_from(value).expect("Failed to convert a Yarn value to a number"),
            "bool" => |value: YarnValue| bool::try_from(value).expect("Failed to convert a Yarn value to a bool"),
        ));
        for r#type in [Type::Number, Type::String, Type::Boolean] {
            library.register_methods(r#type);
        }
        library
    }

    /// Adds a new function to the registry. See [`YarnFn`]'s documentation for what kinds of functions are allowed.
    ///
    /// ## Examples
    /// ```
    /// # use yarn_slinger_core::prelude::*;
    /// # let mut library = Library::default();
    /// library.register_function("length_times_two", string_length(2));
    ///
    /// fn string_length(multiplier: usize) -> yarn_fn_type! { impl Fn(String) -> usize } {
    ///     move |s: String| s.len() * multiplier
    /// }
    /// ```
    pub fn register_function<Marker, F>(
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

    pub fn with_function<Marker, F>(
        mut self,
        name: impl Into<Cow<'static, str>>,
        function: F,
    ) -> Self
    where
        Marker: 'static,
        F: YarnFn<Marker> + 'static + Clone,
        F::Out: IntoYarnValueFromNonYarnValue + 'static + Clone,
    {
        self.register_function(name, function);
        self
    }

    /// Registers the methods found inside a type.
    fn register_methods(&mut self, r#type: Type) {
        for (name, function) in r#type.methods().into_iter() {
            let canonical_name = r#type.get_canonical_name_for_method(name.as_ref());
            self.0.add_boxed(canonical_name, function.clone());
        }
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
