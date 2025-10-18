//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>

use crate::markup::{DialogueTextProcessor, LineParser, MarkupParseError};
use crate::prelude::*;
#[cfg(feature = "bevy")]
use bevy::prelude::World;
use bevy_platform::collections::HashMap;
use core::error::Error;
use core::fmt::{self, Debug, Display};
use log::error;
use yarnspinner_core::prelude::*;

/// Co-ordinates the execution of Yarn programs.
///
/// The main functions of interest are [`Dialogue::continue_`] and [`Dialogue::set_selected_option`].
#[derive(Debug, Clone)]
pub struct Dialogue {
    vm: VirtualMachine,
    language_code: Option<Language>,
}

#[allow(missing_docs)]
pub type Result<T> = core::result::Result<T, DialogueError>;

#[allow(missing_docs)]
#[derive(Debug)]
pub enum DialogueError {
    MarkupParseError(MarkupParseError),
    LineProviderError {
        id: LineId,
        language_code: Option<Language>,
    },
    InvalidOptionIdError {
        selected_option_id: OptionId,
        max_id: usize,
    },
    InvalidLineIdError {
        selected_line_id: LineId,
        line_ids: Vec<LineId>,
    },
    UnexpectedOptionSelectionError,
    ContinueOnOptionSelectionError,
    NoNodeSelectedOnContinue,
    NoProgramLoaded,
    InvalidNode {
        node_name: String,
    },
    VariableStorageError(VariableStorageError),
    FunctionNotFound {
        function_name: String,
        library: Library,
    },
}

impl Error for DialogueError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use DialogueError::*;
        match self {
            MarkupParseError(e) => e.source(),
            VariableStorageError(e) => e.source(),
            _ => None,
        }
    }
}

impl Display for DialogueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DialogueError::*;
        match self {
            MarkupParseError(e) => Display::fmt(e, f),
            LineProviderError { id, language_code } => write!(f, "Line ID \"{id}\" not found in line provider with language code {language_code:?}"),
            InvalidOptionIdError { selected_option_id, max_id } => write!(f, "{selected_option_id:?} is not a valid option ID (expected a number between 0 and {max_id}."),
            InvalidLineIdError { selected_line_id, line_ids } => {
                let line_ids = line_ids.iter().map(|id| id.0.clone()).collect::<Vec<_>>().join(", ");
                write!(f, "{selected_line_id:?} is not a valid line ID of options (expected line ids: {line_ids}.")
            },
            UnexpectedOptionSelectionError => f.write_str("An option was selected, but the dialogue wasn't waiting for a selection. This method should only be called after the Dialogue is waiting for the user to select an option."),
            ContinueOnOptionSelectionError => f.write_str("Dialogue was asked to continue running, but it is waiting for the user to select an option first."),
            NoNodeSelectedOnContinue => f.write_str("Cannot continue running dialogue. No node has been selected."),
            NoProgramLoaded => f.write_str("No program has been loaded. Cannot continue running dialogue."),
            InvalidNode { node_name } => write!(f, "No node named \"{node_name}\" has been loaded."),
            VariableStorageError(e) => Display::fmt(e, f),
            FunctionNotFound { function_name, library } => write!(f, "Function \"{function_name}\" not found in library: {library}"),
        }
    }
}

impl From<MarkupParseError> for DialogueError {
    fn from(source: MarkupParseError) -> Self {
        DialogueError::MarkupParseError(source)
    }
}

impl From<VariableStorageError> for DialogueError {
    fn from(source: VariableStorageError) -> Self {
        DialogueError::VariableStorageError(source)
    }
}

impl Dialogue {
    /// Creates a new [`Dialogue`] instance with the given [`VariableStorage`] and [`TextProvider`].
    /// - The [`TextProvider`] is used to retrieve the text of lines and options.
    /// - The [`VariableStorage`] is used to store and retrieve variables.
    ///
    /// If you don't need any fancy behavior, you can use [`StringTableTextProvider`] and [`MemoryVariableStorage`].
    #[must_use]
    pub fn new(
        variable_storage: Box<dyn VariableStorage>,
        text_provider: Box<dyn TextProvider>,
    ) -> Self {
        let mut library = Library::standard_library();
        library
            .add_function("visited", visited(variable_storage.clone()))
            .add_function("visited_count", visited_count(variable_storage.clone()));

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

    /// Sets the [`Dialogue`]'s language. A value of `None` means that you are using the base language, i.e. the one the Yarn files are written in.
    /// Returns the last language code.
    pub fn set_language_code(
        &mut self,
        language_code: impl Into<Option<Language>>,
    ) -> Option<Language> {
        let language_code = language_code.into();
        self.vm.set_language_code(language_code.clone());
        core::mem::replace(&mut self.language_code, language_code)
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

    /// Gets whether [`Dialogue::continue_`] is able able to return [`DialogueEvent::LineHints`] events.
    /// The default is `false`.
    #[must_use]
    pub fn line_hints_enabled(&self) -> bool {
        self.vm.line_hints_enabled
    }

    /// Mutable gets whether [`Dialogue::continue_`] is able able to return [`DialogueEvent::LineHints`] events.
    /// The default is `false`.
    pub fn set_line_hints_enabled(&mut self, enabled: bool) -> &mut Self {
        self.vm.line_hints_enabled = enabled;
        self
    }

    /// Gets the currently registered [`TextProvider`].
    pub fn text_provider(&self) -> &dyn TextProvider {
        self.vm.text_provider()
    }

    /// Mutable gets the currently registered [`TextProvider`].
    pub fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.vm.text_provider_mut()
    }

    /// Gets the currently registered [`VariableStorage`].
    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.vm.variable_storage()
    }

    /// Mutable gets the currently registered [`VariableStorage`].
    pub fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.vm.variable_storage_mut()
    }
}

// VM proxy
impl Dialogue {
    /// Starts, or continues, execution of the current program.
    ///
    /// Note that when compiling with the `bevy` feature, you should use [`Dialogue::continue_with_world`] instead.
    ///
    /// Calling this method returns a batch of [`DialogueEvent`]s that should be handled by the caller before calling [`Dialogue::continue_`] again.
    /// Some events can be ignored, however this method will error if the following events are not properly handled:
    /// - [`DialogueEvent::Options`] indicates that the program is waiting for the user to select an option.
    ///   The user's selection must be passed to [`Dialogue::set_selected_option`] before calling [`Dialogue::continue_`] again.
    /// - [`DialogueEvent::DialogueComplete`] means that the program reached its end.
    ///   When this occurs, [`Dialogue::set_node`] must be called before [`Dialogue::continue_`] is called again.
    ///
    /// See the documentation of [`DialogueEvent`] for more information on how to handle each event.
    ///
    /// ## Implementation Notes
    ///
    /// All handlers in the original were converted to [`DialogueEvent`]s because registration of complex callbacks is very unidiomatic in Rust.
    /// Specifically, we cannot guarantee [`Send`] and [`Sync`] properly without a lot of [`std::sync::RwLock`] boilerplate. The original implementation
    /// also allows unsound parallel mutation of [`Dialogue`]'s state, which would result in a deadlock in our case.
    pub fn continue_(&mut self) -> Result<Vec<DialogueEvent>> {
        #[cfg(feature = "bevy")]
        bevy::prelude::warn!(
            "Called `continue_` on a dialogue that was compiled with the `bevy` feature. Did you mean to call `continue_with_world` instead?"
        );

        self.vm.continue_(|vm, instruction| {
            vm.run_instruction(instruction, |function, parameters| {
                function.call(parameters)
            })
        })
    }

    #[cfg(feature = "bevy")]
    /// The Bevy version of [`Dialogue::continue_`].
    /// Starts, or continues, execution of the current program.
    ///
    /// Calling this method returns a batch of [`DialogueEvent`]s that should be handled by the caller before calling [`Dialogue::continue_`] again.
    /// Some events can be ignored, however this method will error if the following events are not properly handled:
    /// - [`DialogueEvent::Options`] indicates that the program is waiting for the user to select an option.
    ///   The user's selection must be passed to [`Dialogue::set_selected_option`] before calling [`Dialogue::continue_with_world`] again.
    /// - [`DialogueEvent::DialogueComplete`] means that the program reached its end.
    ///   When this occurs, [`Dialogue::set_node`] must be called before [`Dialogue::continue_with_world`] is called again.
    ///
    /// See the documentation of [`DialogueEvent`] for more information on how to handle each event.
    ///
    /// ## Implementation Notes
    ///
    /// All handlers in the original were converted to [`DialogueEvent`]s because registration of complex callbacks is very unidiomatic in Rust.
    /// Specifically, we cannot guarantee [`Send`] and [`Sync`] properly without a lot of [`std::sync::RwLock`] boilerplate. The original implementation
    /// also allows unsound parallel mutation of [`Dialogue`]'s state, which would result in a deadlock in our case.
    pub fn continue_with_world(&mut self, world: &mut World) -> Result<Vec<DialogueEvent>> {
        self.vm.continue_(move |vm, instruction| {
            vm.run_instruction(instruction, |function, parameters| {
                function.call_with_world(parameters, world)
            })
        })
    }

    /// Returns true if the [`Dialogue`] is in a state where [`Dialogue::continue_`] can be called.
    pub fn can_continue(&self) -> bool {
        self.vm.assert_can_continue().is_ok()
    }

    fn extend_variable_storage_from(&mut self, program: &Program) {
        let initial: HashMap<String, YarnValue> = program
            .initial_values
            .iter()
            .map(|(k, v)| (k.clone(), v.clone().into()))
            .collect();

        // Extend the VariableStorage with the initial values from the program
        if let Err(e) = self.variable_storage_mut().extend(initial) {
            error!("Failed to populate VariableStorage with initial values: {e}");
        }
    }

    /// Sets or replaces the [`Dialogue`]'s current [`Program`]. The program is replaced, all current state is reset.
    pub fn replace_program(&mut self, program: Program) -> &mut Self {
        self.vm.program.replace(program.clone());
        self.vm.reset_state();
        self.extend_variable_storage_from(&program);
        self
    }

    /// Merges the currently set [`Program`] with the given one. If there is no program set, the given one is set.
    pub fn add_program(&mut self, program: Program) -> &mut Self {
        if let Some(existing_program) = self.vm.program.as_mut() {
            *existing_program =
                Program::combine(vec![existing_program.clone(), program.clone()]).unwrap();
        } else {
            self.vm.program.replace(program.clone());
            self.vm.reset_state();
        }
        self.extend_variable_storage_from(&program);

        self
    }

    /// Prepares the [`Dialogue`] that the user intends to start running a node.
    ///
    /// After this method is called, you call [`Dialogue::continue_`] to start executing it.
    ///
    /// If [`Dialogue::line_hints_enabled`] has been set, the next [`Dialogue::continue_`] call will return a [`DialogueEvent::LineHints`],
    /// as the Dialogue determines which lines may be delivered during the `node_name` node's execution.
    ///
    /// ## Errors
    ///
    /// Returns an error if no node with the value of `node_name` has been loaded.
    pub fn set_node(&mut self, node_name: impl Into<String>) -> Result<&mut Self> {
        self.vm.set_node(node_name)?;
        Ok(self)
    }

    /// Attempts to pop the line hints that were generated by the last [`Dialogue::set_node`] call.
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
            .map(|_| format!("{LINE_ID_PREFIX}{node_name}").into())
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

    /// Returns the headers for the node `node_name`.
    ///
    /// The headers are all the key-value pairs defined in the node's source code
    /// including the `tags` and `title` headers.
    ///
    /// Returns [`None`] if the node is not present in the program.
    #[must_use]
    pub fn get_headers_for_node(&self, node_name: &str) -> Option<HashMap<String, String>> {
        self.get_node_logging_errors(node_name).map(|node| {
            node.headers
                .iter()
                .map(|header| (header.key.clone(), header.value.clone()))
                .collect()
        })
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
    /// If [`Dialogue::continue_`] has never been called, this value will be [`None`].
    #[must_use]
    pub fn current_node(&self) -> Option<String> {
        self.vm.current_node()
    }

    /// Analyses the currently loaded Yarn program with the given [`Context`]. Call [`Context::finish_analysis`] afterwards to get the results.
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
    /// After the Dialogue emitted a [`DialogueEvent::Options`] in [`Dialogue::continue_`], this method must be called before [`Dialogue::continue_`] is called.
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

    /// Signals to the [`Dialogue`] that the user has selected a specified [`DialogueOption`].
    ///
    /// This makes dialogue replay more robust than [`self.set_selected_option`] when adding new options.
    ///
    /// The ID number that should be passed as the parameter to this method should be the id
    /// of the [`line`] field in the [`DialogueOption`] that represents the user's selection.
    ///
    /// ## Panics
    /// - If the Dialogue is not expecting an option to be selected.
    /// - If the line ID is not found in the vector of [`DialogueOption`] provided by [`DialogueEvent::Options`].
    ///
    /// ## See Also
    /// - [`Dialogue::continue_`]
    pub fn set_selected_option_by_line_id(&mut self, selected_line_id: LineId) -> Result<OptionId> {
        self.vm.set_selected_option_by_line_id(selected_line_id)
    }

    /// Gets a value indicating whether the Dialogue is currently executing Yarn instructions.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.vm.is_active()
    }

    /// Returns `true` if the last call to [`Dialogue::continue_`] returned [`DialogueEvent::Options`] and the dialogue is therefore
    /// waiting for the user to select an option via [`Dialogue::set_selected_option`]. If this is `true`, calling [`Dialogue::continue_`] will error
    pub fn is_waiting_for_option_selection(&self) -> bool {
        self.vm.is_waiting_for_option_selection()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_send_sync() {
        let variable_storage = Box::new(MemoryVariableStorage::new());
        let text_provider = Box::new(StringTableTextProvider::new());
        let dialogue = Dialogue::new(variable_storage, text_provider);
        accept_send_sync(dialogue);
    }

    fn accept_send_sync(_: impl Send + Sync) {}
}
