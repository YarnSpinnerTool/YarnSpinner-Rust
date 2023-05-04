use crate::prelude::*;
use std::fmt::Debug;
use yarn_slinger_core::prelude::*;

/// Co-ordinates the execution of Yarn programs.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Dialogue {
    /// Gets the [`Library`] that this Dialogue uses to locate functions.
    ///
    /// When the Dialogue is constructed, the Library is initialized with
    /// the built-in operators like `+`, `-`, and so on.
    pub library: Library,

    /// The object that provides access to storing and retrieving the values of variables.
    pub variable_storage: Box<dyn VariableStorage>,

    /// Invoked when the Dialogue needs to report debugging information.
    pub log_debug_message: Box<Logger>,
}

impl Dialogue {
    /// Initializes a new instance of the [`Dialogue`] class.
    pub fn with_variable_storage(variable_storage: impl VariableStorage + 'static) -> Self {
        let mut library = Library::default();
        library.import(Library::standard_library());
        library.register_function("visited", |_node: String| -> bool { todo!() });
        library.register_function("visited_count", |_node: String| -> f32 { todo!() });
        Self {
            variable_storage: Box::new(variable_storage),
            library,
        }
    }
}
