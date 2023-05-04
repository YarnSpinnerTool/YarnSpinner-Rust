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
    pub log_debug_message: Option<Logger>,
}

impl Default for Dialogue {
    fn default() -> Self {
        let library = Library::standard_library()
            .with_function("visited", |_node: String| -> bool { todo!() })
            .with_function("visited_count", |_node: String| -> f32 { todo!() });
        let default_variable_storage = Box::new(MemoryVariableStore::default());

        Self {
            library,
            variable_storage: default_variable_storage,
            log_debug_message: Default::default(),
        }
    }
}

impl Dialogue {
    /// Initializes a new instance of the [`Dialogue`] class.
    pub fn with_variable_storage(self, variable_storage: impl VariableStorage + 'static) -> Self {
        Self {
            variable_storage: Box::new(variable_storage),
            ..self
        }
    }

    pub fn with_logger(self, logger: impl Fn(String) + Clone + 'static) -> Self {
        Self {
            log_debug_message: Some(Logger(Box::new(logger))),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_handler() {
        let dialogue = Dialogue::default().with_logger(|_| {});
        let _cloned = dialogue.clone();
    }
}
