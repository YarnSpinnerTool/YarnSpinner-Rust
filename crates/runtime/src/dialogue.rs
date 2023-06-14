//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>

use crate::markup::{DialogueTextProcessor, LineParser, MarkupParseError};
use crate::prelude::*;
use log::error;
use std::fmt::Debug;
use thiserror::Error;
use yarn_slinger_core::prelude::*;

/// Co-ordinates the execution of Yarn programs.
///
/// The main functions of interest are [`Dialogue::continue_`] and [`Dialogue::set_selected_option`].
#[derive(Debug)]
pub struct Dialogue {
    vm: VirtualMachine,
    language_code: Option<Language>,
}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, DialogueError>;

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum DialogueError {
    #[error(transparent)]
    MarkupParseError(#[from] MarkupParseError),
    #[error("Line ID \"{id}\" not found in line provider with language code {language_code:?}")]
    LineProviderError {
        id: LineId,
        language_code: Option<Language>,
    },
    #[error("{selected_option_id:?} is not a valid option ID (expected a number between 0 and {max_id}.")]
    InvalidOptionIdError {
        selected_option_id: OptionId,
        max_id: usize,
    },
    #[error("An option was selected, but the dialogue wasn't waiting for a selection. \
            This method should only be called after the Dialogue is waiting for the user to select an option.")]
    UnexpectedOptionSelectionError,
    #[error("Dialogue was asked to continue running, but it is waiting for the user to select an option first.")]
    ContinueOnOptionSelectionError,
    #[error("Cannot continue running dialogue. No node has been selected.")]
    NoNodeSelectedOnContinue,
    #[error("No node named \"{node_name}\" has been loaded.")]
    InvalidNode { node_name: String },
    #[error(transparent)]
    VariableStorageError(#[from] VariableStorageError),
}

impl Dialogue {
    #[must_use]
    pub fn new(
        variable_storage: Box<dyn VariableStorage>,
        text_provider: Box<dyn TextProvider>,
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

fn visited(storage: Box<dyn VariableStorage>) -> yarn_fn_type! { impl Fn(String) -> bool } {
    move |node: String| -> bool {
        let name = Library::generate_unique_visited_variable_for_node(&node);
        if let Ok(YarnValue::Number(count)) = storage.get(&name) {
            count > 0.0
        } else {
            false
        }
    }
}

fn visited_count(storage: Box<dyn VariableStorage>) -> yarn_fn_type! { impl Fn(String) -> f32 } {
    move |node: String| {
        let name = Library::generate_unique_visited_variable_for_node(&node);
        if let Ok(YarnValue::Number(count)) = storage.get(&name) {
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
    /// Sets the language code for the [`Dialogue`].
    pub fn with_language_code(mut self, language_code: impl Into<Option<Language>>) -> Self {
        self.set_language_code(language_code);
        self
    }

    /// Extends the [`Dialogue`]'s [`Library`] with the given [`Library`].
    #[must_use]
    pub fn with_extended_library(mut self, library: Library) -> Self {
        self.library_mut().extend(library.into_iter());
        self
    }

    /// Sets the current node to the one with the given name.
    pub fn with_node_at(mut self, node_name: &str) -> Result<Self> {
        self.set_node(node_name)?;
        Ok(self)
    }

    /// Sets the current node to the node named [`Dialogue::DEFAULT_START_NODE_NAME`], i.e. `"Start"`.
    pub fn with_node_at_start(mut self) -> Result<Self> {
        self.set_node_to_start()?;
        Ok(self)
    }

    /// Activates [`Dialogue::next`] being able to return [`DialogueEvent::LineHints`] events.
    /// Note that line hints for [`Dialogue::with_node_at_start`] and [`Dialogue::with_node_at`] will only be sent if this
    /// method was called beforehand.
    #[must_use]
    pub fn with_line_hints_enabled(mut self, enabled: bool) -> Self {
        self.vm.line_hints_enabled = enabled;
        self
    }

    #[must_use]
    pub fn with_program(mut self, program: Program) -> Self {
        self.replace_program(program);
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
    pub fn language_code(&self) -> Option<&Language> {
        self.language_code.as_ref()
    }

    pub fn set_language_code(
        &mut self,
        language_code: impl Into<Option<Language>>,
    ) -> Option<Language> {
        let language_code = language_code.into();
        self.vm.set_language_code(language_code.clone());
        std::mem::replace(&mut self.language_code, language_code)
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
    pub fn line_hints_enabled(&self) -> bool {
        self.vm.line_hints_enabled
    }

    /// Mutable gets whether [`Dialogue::next`] is able able to return [`DialogueEvent::LineHints`] events.
    /// The default is `false`.
    #[must_use]
    pub fn line_hints_enabled_mut(&mut self) -> &mut bool {
        &mut self.vm.line_hints_enabled
    }

    pub fn text_provider(&self) -> &dyn TextProvider {
        self.vm.text_provider()
    }

    pub fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.vm.text_provider_mut()
    }

    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.vm.variable_storage()
    }

    pub fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.vm.variable_storage_mut()
    }
}

// VM proxy
impl Dialogue {
    /// The name used by [`Dialogue::set_node_to_start`] and [`Dialogue::with_node_at_start`].
    pub const DEFAULT_START_NODE_NAME: &'static str = "Start";

    /// Starts, or continues, execution of the current program.
    ///
    /// Calling this method returns a batch of [`DialogueEvent`]s that should be handled by the caller before calling [`Dialogue::continue_`] again.
    /// Some events can be ignored, however this method will error if the following events are not properly handled:
    /// - [`DialogueEvent::Options`] indicates that the program is waiting for the user to select an option.
    /// The user's selection must be passed to [`Dialogue::set_selected_option`] before calling [`Dialogue::continue_`] again.
    /// - [`DialogueEvent::DialogueComplete`] means that the program reached its end.
    /// When this occurs, [`Dialogue::set_node`] or [`Dialogue::set_node_to_start`] must be called before [`Dialogue::continue_`] is called again.
    ///
    /// See the documentation of [`DialogueEvent`] for more information on how to handle each event.
    ///
    /// The [`Iterator`] implementation of [`Dialogue`] is a convenient way to call [`Dialogue::next`] repeatedly, although it panics if an error occurs.
    ///
    /// ## Implementation Notes
    ///
    /// All handlers in the original were converted to [`DialogueEvent`]s because registration of complex callbacks is very unidiomatic in Rust.
    /// Specifically, we cannot guarantee [`Send`] and [`Sync`] properly without a lot of [`std::sync::RwLock`] boilerplate. The original implementation
    /// also allows unsound parallel mutation of [`Dialogue`]'s state, which would result in a deadlock in our case.
    pub fn continue_(&mut self) -> Result<Vec<DialogueEvent>> {
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
    /// If [`Dialogue::line_hints_enabled`] has been set, the next [`Dialogue::next`] call will return a [`DialogueEvent::LineHints`],
    /// as the Dialogue determines which lines may be delivered during the `node_name` node's execution.
    ///
    /// ## Errors
    ///
    /// Returns an error if no node with the value of `node_name` has been loaded.
    pub fn set_node(&mut self, node_name: impl Into<String>) -> Result<&mut Self> {
        self.vm.set_node(node_name)?;
        Ok(self)
    }

    /// Calls [`Dialogue::set_node`] with the [`Dialogue::DEFAULT_START_NODE_NAME`].
    ///
    /// ## Errors
    ///
    /// Returns an error if no node with the value of [`Dialogue::DEFAULT_START_NODE_NAME`] has been loaded.
    pub fn set_node_to_start(&mut self) -> Result<&mut Self> {
        self.set_node(Self::DEFAULT_START_NODE_NAME)?;
        Ok(self)
    }

    /// Attempts to pop the line hints that were generated by the last [`Dialogue::set_node_to_start`] or [`Dialogue::set_node`] call.
    ///
    /// Panics if [`Dialogue::line_hints_enabled`] is `false`.
    pub fn pop_line_hints(&mut self) -> Option<Vec<LineId>> {
        assert!(
            self.line_hints_enabled(),
            "Tried to call pop_line_hints when line hints are disabled."
        );
        self.vm.pop_line_hints()
    }

    /// Immediately stops the [`Dialogue`]
    ///
    /// Returns unfinished [`DialogueEvent`]s that should be handled by the caller. The last is guaranteed to be [`DialogueEvent::DialogueComplete`].
    pub fn stop(&mut self) -> Vec<DialogueEvent> {
        self.vm.stop()
    }

    /// Unloads all nodes from the Dialogue.
    pub fn unload_all(&mut self) {
        self.vm.unload_programs()
    }

    /// Gets the names of the nodes in the currently loaded Program, if there is one.
    #[must_use]
    pub fn node_names(&self) -> Option<impl Iterator<Item = &str>> {
        self.vm
            .program
            .as_ref()
            .map(|program| program.nodes.keys().map(|s| s.as_str()))
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

    /// Gets a value indicating whether a specified node exists in the [`Program`].
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
    /// After the Dialogue emitted a [`DialogueEvent::Options`] in [`Dialogue::continue_`], this method must be called before [`Dialogue::next`] is called.
    ///
    /// The ID number that should be passed as the parameter to this method should be the [`OptionId`]
    /// field in the [`DialogueOption`] that represents the user's selection.
    ///
    /// ## Panics
    /// - If the Dialogue is not expecting an option to be selected.
    /// - If the option ID is not found in the vector of [`DialogueOption`] provided by [`DialogueEvent::Options`].
    ///
    /// ## See Also
    /// - [`Dialogue::continue_`]
    pub fn set_selected_option(&mut self, selected_option_id: OptionId) -> Result<&mut Self> {
        self.vm.set_selected_option(selected_option_id)?;
        Ok(self)
    }

    /// Gets a value indicating whether the Dialogue is currently executing Yarn instructions.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.vm.is_active()
    }

    pub fn is_waiting_for_option_selection(&self) -> bool {
        self.vm.is_waiting_for_option_selection()
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
