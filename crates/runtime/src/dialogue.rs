//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>

use crate::markup::{DialogueTextProcessor, LineParser, MarkupParseError};
use crate::prelude::*;
use crate::text_provider::TextProvider;
use log::error;
use std::fmt::Debug;
use thiserror::Error;
use yarn_slinger_core::prelude::*;

/// Co-ordinates the execution of Yarn programs.
///
/// ## Implementation notes
///
/// The original implementation allows calling [`Dialogue`] from handlers freely.
/// Rust's ownership rules rightfully prevent us from doing that, so the methods that
/// are useful to call from handlers are exposed in [`ReadOnlyDialogue`].
///
/// It implements [`Send`] and [`Sync`], so it can be freely moved into handlers after being retrieved via [`Dialogue::get_read_only`].
/// [`Dialogue`] also implements [`Deref`] for [`ReadOnlyDialogue`], so you don't need to worry about this distinction if
/// you're only calling the [`Dialogue`] from outside handlers.
#[derive(Debug)]
pub struct Dialogue {
    vm: VirtualMachine,
    language_code: Option<String>,
}

pub type Result<T> = std::result::Result<T, DialogueError>;

#[derive(Debug, Error)]
pub enum DialogueError {
    #[error(transparent)]
    MarkupParseError(#[from] MarkupParseError),
    #[error("Line ID {id} not found in line provider with language code {language_code:?}")]
    LineProviderError {
        id: LineId,
        language_code: Option<String>,
    },
}

impl Dialogue {
    #[must_use]
    pub fn new(
        variable_storage: Box<dyn VariableStorage + Send + Sync>,
        text_provider: Box<dyn TextProvider + Send + Sync>,
    ) -> Self {
        let library = Library::standard_library()
            .with_function("visited", visited(variable_storage.clone()))
            .with_function("visited_count", visited_count(variable_storage.clone()));

        let dialogue_text_processor = Box::new(DialogueTextProcessor::new());
        let line_parser = LineParser::new()
            .register_marker_processor("select", dialogue_text_processor.clone())
            .register_marker_processor("plural", dialogue_text_processor.clone())
            .register_marker_processor("ordinal", dialogue_text_processor);

        Self {
            vm: VirtualMachine::new(library, variable_storage, line_parser, text_provider),
            language_code: Default::default(),
        }
    }
}

fn visited(
    storage: Box<dyn VariableStorage + Send + Sync>,
) -> yarn_fn_type! { impl Fn(String) -> bool } {
    move |node: String| -> bool {
        let name = Library::generate_unique_visited_variable_for_node(&node);
        if let Some(YarnValue::Number(count)) = storage.get(&name) {
            count > 0.0
        } else {
            false
        }
    }
}

fn visited_count(
    storage: Box<dyn VariableStorage + Send + Sync>,
) -> yarn_fn_type! { impl Fn(String) -> f32 } {
    move |node: String| {
        let name = Library::generate_unique_visited_variable_for_node(&node);
        if let Some(YarnValue::Number(count)) = storage.get(&name) {
            count
        } else {
            0.0
        }
    }
}

impl Iterator for Dialogue {
    type Item = Vec<DialogueEvent>;

    /// Panicking version of [`Dialogue::continue_`].
    #[must_use = "All dialogue events that are returned by the dialogue must be handled or explicitly ignored"]
    fn next(&mut self) -> Option<Self::Item> {
        self.vm.next()
    }
}

// Builder API
impl Dialogue {
    #[must_use]
    pub fn with_language_code(mut self, language_code: impl Into<String>) -> Self {
        self.set_language_code(language_code);
        self
    }

    #[must_use]
    pub fn with_extended_library(mut self, library: Library) -> Self {
        self.library_mut().extend(library.into_iter());
        self
    }

    #[must_use]
    pub fn with_node_at(mut self, node_name: &str) -> Self {
        self.set_node(node_name);
        self
    }

    #[must_use]
    pub fn with_node_at_start(mut self) -> Self {
        self.set_node_to_start();
        self
    }

    /// Activates [`Dialogue::next`] being able to return [`DialogueEvent::LineHints`] events.
    /// Note that line hints for [`with_node_at_start`] and [`with_node_at`] will only be sent if this
    /// method was called beforehand.
    #[must_use]
    pub fn with_should_send_line_hints(mut self) -> Self {
        self.vm.should_send_line_hints = true;
        self
    }
}

// Accessors
impl Dialogue {
    /// The [`Dialogue`]'s locale, as an IETF BCP 47 code.
    ///
    /// This code is used to determine how the `plural` and `ordinal`
    /// markers determine the plural class of numbers.
    ///
    /// For example, the code "en-US" represents the English language as
    /// used in the United States.
    ///
    /// ## Returns
    ///
    /// Returns the last language code.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        self.language_code.as_deref()
    }

    pub fn set_language_code(&mut self, language_code: impl Into<String>) -> Option<String> {
        let language_code = language_code.into();
        let previous = self.language_code.replace(language_code.clone());
        self.vm.set_language_code(language_code);
        previous
    }

    /// Gets the [`Library`] that this Dialogue uses to locate functions.
    ///
    /// When the Dialogue is constructed, the Library is initialized with
    /// the built-in operators like `+`, `-`, and so on.
    #[must_use]
    pub fn library(&self) -> &Library {
        &self.vm.library
    }

    /// See [`Dialogue::library`].
    #[must_use]
    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.vm.library
    }

    /// Gets whether [`Dialogue::next`] is able able to return [`DialogueEvent::LineHints`] events.
    /// The default is `false`.
    #[must_use]
    pub fn should_send_line_hints(&self) -> bool {
        self.vm.should_send_line_hints
    }

    /// Mutable gets whether [`Dialogue::next`] is able able to return [`DialogueEvent::LineHints`] events.
    /// The default is `false`.
    #[must_use]
    pub fn should_send_line_hints_mut(&mut self) -> &mut bool {
        &mut self.vm.should_send_line_hints
    }
}

// VM proxy
impl Dialogue {
    /// The name used by [`Dialogue::set_node_to_start`] and [`Dialogue::with_node_at_start`].
    pub const DEFAULT_START_NODE_NAME: &'static str = "Start";

    /// Starts, or continues, execution of the current program.
    ///
    /// This method repeatedly executes instructions until one of the following conditions is encountered:
    /// - The [`LineHandler`] or [`CommandHandler`] is called. After calling either of these handlers, the Dialogue will wait until [`Dialogue::next`] is called.
    /// - The [`OptionsHandler`] is called. When this occurs, the Dialogue is waiting for the user to specify which of the options has been selected,
    /// and [`Dialogue::set_selected_option`] must be called before [`Dialogue::next`] is called.
    /// - The program reaches its end. When this occurs, [`Dialogue::set_node`] must be called before [`Dialogue::next`] is called again.
    /// - An error occurs while executing the program
    ///
    /// This method has no effect if it is called while the [`Dialogue`] is currently in the process of executing instructions.
    ///
    /// ## See Also
    /// - [`LineHandler`]
    /// - [`OptionsHandler`]
    /// - [`CommandHandler`]
    /// - [`NodeCompleteHandler`]
    /// - [`DialogueCompleteHandler`]
    /// The [`Iterator`] implementation of [`Dialogue`] is a convenient way to call [`Dialogue::next`] repeatedly, although it panics if an error occurs.
    ///
    /// ## Implementation Notes
    ///
    /// The original states that the [`LineHandler`] and [`CommandHandler`] may call [`Dialogue::next`]. Because of the borrow checker,
    /// this is action is very unidiomatic and impossible to do without introducing a lot of interior mutability all along the API.
    /// For this reason, we disallow mutating the [`Dialogue`] within any handler.
    pub fn continue_(&mut self) -> Result<Option<Vec<DialogueEvent>>> {
        self.vm.continue_()
    }

    pub fn replace_program(&mut self, program: Program) -> &mut Self {
        self.vm.program.replace(program);
        self.vm.reset_state();
        self
    }

    pub fn add_program(&mut self, program: Program) -> &mut Self {
        if let Some(existing_program) = self.vm.program.as_mut() {
            *existing_program = Program::combine(vec![existing_program.clone(), program]).unwrap();
        } else {
            self.vm.program.replace(program);
            self.vm.reset_state();
        }

        self
    }

    /// Prepares the [`Dialogue`] that the user intends to start running a node.
    ///
    /// After this method is called, you call [`Dialogue::next`] to start executing it.
    ///
    /// If [`Dialogue::should_send_line_hints`] has been set, the next [`Dialogue::next`] call will return a [`DialogueEvent::LineHints`],
    /// as the Dialogue determines which lines may be delivered during the `node_name` node's execution.
    ///
    /// ## Panics
    ///
    /// Panics if no node named `node_name` has been loaded.
    pub fn set_node(&mut self, node_name: &str) -> &mut Self {
        self.vm.set_node(node_name);
        self
    }

    /// Calls [`Dialogue::set_node`] with the [`Dialogue::DEFAULT_START_NODE_NAME`].
    pub fn set_node_to_start(&mut self) -> &mut Self {
        self.set_node(Self::DEFAULT_START_NODE_NAME);
        self
    }

    /// Immediately stops the [`Dialogue`]
    ///
    /// The [`DialogueCompleteHandler`] will not be called if the
    /// dialogue is ended this way.
    pub fn stop(&mut self) -> &mut Self {
        self.vm.stop();
        self
    }

    /// Unloads all nodes from the Dialogue.
    pub fn unload_all(&mut self) {
        self.vm.unload_programs()
    }

    /// Gets the names of the nodes in the currently loaded Program, if there is one.
    #[must_use]
    pub fn node_names(&self) -> Option<Vec<String>> {
        self.vm
            .program
            .as_ref()
            .map(|program| program.nodes.keys().cloned().collect())
    }

    /// Returns the line ID that contains the original, uncompiled source
    /// text for a node.
    ///
    /// A node's source text will only be present in the string table if its
    /// `tags` header contains `rawText`.
    ///
    /// Because the [`Dialogue`] API is designed to be unaware
    /// of the contents of the string table, this method does not test to
    /// see if the string table contains an entry with the line ID. You will
    /// need to test for that yourself.
    #[must_use]
    pub fn get_line_id_for_node(&self, node_name: &str) -> Option<LineId> {
        self.get_node_logging_errors(node_name)
            .map(|_| format!("line:{node_name}").into())
    }

    /// Returns the tags for the node `node_name`.
    ///
    /// The tags for a node are defined by setting the `tags` header in
    /// the node's source code. This header must be a space-separated list
    ///
    /// Returns [`None`] if the node is not present in the program.
    #[must_use]
    pub fn get_tags_for_node(&self, node_name: &str) -> Option<Vec<String>> {
        self.get_node_logging_errors(node_name)
            .map(|node| node.tags)
    }

    /// Gets a value indicating whether a specified node exists in the Program.
    #[must_use]
    pub fn node_exists(&self, node_name: &str) -> bool {
        // Not calling `get_node_logging_errors` because this method does not write errors when there are no nodes.
        if let Some(program) = self.vm.program.as_ref() {
            program.nodes.contains_key(node_name)
        } else {
            error!("Tried to call NodeExists, but no program has been loaded");
            false
        }
    }

    /// Gets the name of the node that this Dialogue is currently executing.
    ///
    /// If [`Dialogue::next`] has never been called, this value will be [`None`].
    #[must_use]
    pub fn current_node(&self) -> Option<String> {
        self.vm.current_node()
    }

    pub fn analyse(&self, context: &mut Context) -> &Self {
        let program = self
            .vm
            .program
            .as_ref()
            .expect("Failed to analyse program: No program loaded");
        context.diagnose_program(program);
        self
    }

    fn get_node_logging_errors(&self, node_name: &str) -> Option<Node> {
        if let Some(program) = self.vm.program.as_ref() {
            if program.nodes.is_empty() {
                error!("No nodes are loaded");
                None
            } else if let Some(node) = program.nodes.get(node_name) {
                Some(node.clone())
            } else {
                error!("No node named {node_name}");
                None
            }
        } else {
            error!("No program is loaded");
            None
        }
    }

    /// Signals to the [`Dialogue`] that the user has selected a specified [`DialogueOption`].
    ///
    /// After the Dialogue delivers an [`OptionSet`], this method must be called before [`Dialogue::next`] is called.
    ///
    /// The ID number that should be passed as the parameter to this method should be the [`DialogueOption::Id`]
    /// field in the [`DialogueOption`] that represents the user's selection.
    ///
    /// ## Panics
    /// - If the Dialogue is not expecting an option to be selected.
    /// - If the option ID is not found in the current [`OptionSet`].
    ///
    /// ## See Also
    /// - [`Dialogue::next`]
    /// - [`OptionsHandler`]
    /// - [`OptionSet`]
    pub fn set_selected_option(&mut self, selected_option_id: OptionId) -> &mut Self {
        self.vm.set_selected_option(selected_option_id);
        self
    }

    /// Gets a value indicating whether the Dialogue is currently executing Yarn instructions.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.vm.is_active()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_send_sync() {
        let variable_storage = Box::new(MemoryVariableStore::new());
        let text_provider = Box::new(StringTableTextProvider::new());
        let dialogue = Dialogue::new(variable_storage, text_provider);
        accept_send_sync(dialogue);
    }

    fn accept_send_sync(_: impl Send + Sync) {}
}
