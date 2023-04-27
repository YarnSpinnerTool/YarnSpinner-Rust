//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Library.cs>

use crate::prelude::YarnFnRegistry;

/// A collection of functions that can be called from Yarn scripts.
///
/// You do not create instances of this class yourself. The [`Dialogue`]
/// class creates one for you, and you can access it through the
/// [`Library`] property.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Library {
    /// The functions that are available to Yarn scripts.
    functions: YarnFnRegistry,
}

impl Library {
    /// Generates a unique tracking variable name.
    /// This is intended to be used to generate names for visiting.
    /// Ideally these will very reproducible and sensible.
    /// For now it will be something terrible and easy.
    pub fn generate_unique_visited_variable_for_node(node_name: &str) -> String {
        format!("$Yarn.Internal.Visiting.{node_name}")
    }
}
