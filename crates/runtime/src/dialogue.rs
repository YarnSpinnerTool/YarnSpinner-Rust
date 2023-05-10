use crate::prelude::*;
pub use handler_safe_dialogue::*;
pub(crate) use shared_state::*;
use std::fmt::Debug;
use std::ops::Deref;
use yarn_slinger_core::prelude::*;

mod handler_safe_dialogue;
mod shared_state;

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
    shared_state: SharedState,
    handler_safe_dialogue: HandlerSafeDialogue,
}

impl Default for Dialogue {
    fn default() -> Self {
        let shared_state = SharedState::default();
        let handler_safe_dialogue = HandlerSafeDialogue::from_shared_state(shared_state.clone());

        let mut vm = VirtualMachine::with_shared_state(shared_state.clone());
        let storage_one = shared_state.variable_storage_shared();
        let storage_two = storage_one.clone();
        vm.library
            .register_function("visited", move |node: String| -> bool {
                is_node_visited(storage_one.read().unwrap().deref().as_ref(), &node)
            })
            .register_function("visited_count", move |node: String| -> f32 {
                get_node_visit_count(storage_two.read().unwrap().deref().as_ref(), &node)
            });
        Self {
            vm,
            shared_state,
            handler_safe_dialogue,
        }
    }
}

impl SharedStateHolder for Dialogue {
    fn shared_state(&self) -> &SharedState {
        &self.shared_state
    }
}

// Builder API
impl Dialogue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_variable_storage<T: VariableStorage + 'static + Send + Sync>(
        &mut self,
        variable_storage: impl VariableStorage + 'static + Send + Sync,
    ) -> &mut Self {
        *self.variable_storage_mut() = Box::new(variable_storage);
        self
    }

    pub fn set_log_debug_message(
        &mut self,
        logger: impl Fn(String, &HandlerSafeDialogue) + Clone + 'static + Send + Sync,
    ) -> &mut Self {
        self.handler_safe_dialogue.log_debug_message = Box::new(logger.clone());
        self.vm.set_log_debug_message(logger);
        self
    }

    pub fn set_log_error_message(
        &mut self,
        logger: impl Fn(String, &HandlerSafeDialogue) + Clone + 'static + Send + Sync,
    ) -> &mut Self {
        self.handler_safe_dialogue.log_error_message = Box::new(logger.clone());
        self.vm.set_log_error_message(logger);
        self
    }

    pub fn set_line_handler(
        &mut self,
        line_handler: impl FnMut(Line, &mut HandlerSafeDialogue) + 'static + Send + Sync,
    ) -> &mut Self {
        self.vm.line_handler = Box::new(line_handler);
        self
    }

    /// The [`OptionsHandler`] that is called when a set of options are ready to be shown to the user.
    ///
    /// The Options Handler delivers a [`Vec`] of [`DialogueOption`] to the game.
    /// Before [`Dialogue::continue_`] can be called to resume execution,
    /// [`Dialogue::set_selected_option`] must be called to indicate which
    /// [`DialogueOption`] was selected by the user. If [`Dialogue::set_selected_option`] is not called, a panic occurs.
    pub fn set_options_handler(
        &mut self,
        options_handler: impl FnMut(Vec<DialogueOption>, &mut HandlerSafeDialogue)
            + 'static
            + Send
            + Sync,
    ) -> &mut Self {
        self.vm.options_handler = Box::new(options_handler);
        self
    }

    /// The [`CommandHandler`] that is called when a command is to be delivered to the game.
    pub fn set_command_handler(
        &mut self,
        command_handler: impl FnMut(Command, &mut HandlerSafeDialogue) + 'static + Send + Sync,
    ) -> &mut Self {
        self.vm.command_handler = Box::new(command_handler);
        self
    }

    /// The [`NodeCompleteHandler`] that is called when a node is complete.
    pub fn set_node_complete_handler(
        &mut self,
        node_complete_handler: impl FnMut(String, &mut HandlerSafeDialogue) + 'static + Send + Sync,
    ) -> &mut Self {
        self.vm.node_complete_handler = Box::new(node_complete_handler);
        self
    }

    /// The [`NodeStartHandler`] that is called when a node is started.
    pub fn set_node_start_handler(
        &mut self,
        node_start_handler: impl FnMut(String, &mut HandlerSafeDialogue) + 'static + Send + Sync,
    ) -> &mut Self {
        self.vm.node_start_handler = Some(Box::new(node_start_handler));
        self
    }

    /// The [`DialogueCompleteHandler`] that is called when the Dialogue reaches its end.
    pub fn set_dialogue_complete_handler(
        &mut self,
        dialogue_complete_handler: impl FnMut(&mut HandlerSafeDialogue) + 'static + Send + Sync,
    ) -> &mut Self {
        self.vm.dialogue_complete_handler = Some(Box::new(dialogue_complete_handler));
        self
    }

    /// The [`PrepareForLinesHandler`] that is called when the dialogue anticipates delivering some lines.
    pub fn set_prepare_for_lines_handler(
        &mut self,
        prepare_for_lines_handler: impl FnMut(Vec<LineId>, &mut HandlerSafeDialogue)
            + 'static
            + Send
            + Sync,
    ) -> &mut Self {
        self.vm.prepare_for_lines_handler = Some(Box::new(prepare_for_lines_handler));
        self
    }

    pub fn set_language_code(&mut self, language_code: impl Into<String>) -> &mut Self {
        self.language_code_mut().replace(language_code.into());
        self
    }
}

// VM proxy
impl Dialogue {
    pub const DEFAULT_START_NODE_NAME: &'static str = "Start";

    /// Gets the [`Library`] that this Dialogue uses to locate functions.
    ///
    /// When the Dialogue is constructed, the Library is initialized with
    /// the built-in operators like `+`, `-`, and so on.
    pub fn library(&self) -> &Library {
        &self.vm.library
    }

    /// See [`Dialogue::library`].
    pub fn library_mut(&mut self) -> &mut Library {
        &mut self.vm.library
    }

    /// The object that provides access to storing and retrieving the values of variables.
    /// Be aware that accessing this object will block [`Dialogue::continue_`] and vice versa, so try to not cause a deadlock.
    pub fn variable_storage(&self) -> SharedMemoryVariableStore {
        SharedMemoryVariableStore(self.variable_storage_shared())
    }

    pub fn replace_program(&mut self, program: Program) -> &mut Self {
        self.vm.program_mut().replace(program);
        self.vm.reset_state();
        self
    }

    pub fn add_program(&mut self, program: Program) -> &mut Self {
        {
            let mut existing_program = self.program_mut();
            if let Some(existing_program) = existing_program.as_mut() {
                *existing_program =
                    Program::combine(vec![existing_program.clone(), program]).unwrap();
            } else {
                *existing_program = Some(program);
                drop(existing_program);
                self.vm.reset_state();
            }
        }
        self
    }

    /// Prepares the [`Dialogue`] that the user intends to start running a node.
    ///
    /// After this method is called, you call [`Dialogue::continue_`] to start executing it.
    ///
    /// If [`Dialogue::prepare_for_lines_handler`] has been set, it may be called when this method is invoked,
    /// as the Dialogue determines which lines may be delivered during the `start_node` node's execution.
    ///
    /// ## Panics
    ///
    /// Panics if no node named `start_node` has been loaded.
    pub fn set_node(&mut self, start_node: &str) -> &mut Self {
        self.vm.set_node(start_node);
        self
    }

    pub fn set_node_to_start(&mut self) -> &mut Self {
        self.set_node(Self::DEFAULT_START_NODE_NAME);
        self
    }

    /// Starts, or continues, execution of the current program.
    ///
    /// This method repeatedly executes instructions until one of the following conditions is encountered:
    /// - The [`LineHandler`] or [`CommandHandler`] is called. After calling either of these handlers, the Dialogue will wait until [`Dialogue::continue_`] is called.
    /// - The [`OptionsHandler`] is called. When this occurs, the Dialogue is waiting for the user to specify which of the options has been selected,
    /// and [`Dialogue::set_selected_option`] must be called before [`Dialogue::continue_`] is called.
    /// - The program reaches its end. When this occurs, [`Dialogue::set_node`] must be called before [`Dialogue::continue_`] is called again.
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
    ///
    /// ## Implementation Notes
    ///
    /// The original states that the [`LineHandler`] and [`CommandHandler`] may call [`Dialogue::continue_`]. Because of the borrow checker,
    /// this is action is very unidiomatic and impossible to do without introducing a lot of interior mutability all along the API.
    /// For this reason, we disallow mutating the [`Dialogue`] within any handler.
    pub fn continue_(&mut self) -> Option<DialogueEvent> {
        // Cannot 'continue' an already running VM.
        self.vm.continue_()
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
}

// HandlerSafeDialogue proxy
impl Dialogue {
    /// Gets the names of the nodes in the currently loaded Program, if there is one.
    pub fn node_names(&self) -> Option<Vec<String>> {
        self.handler_safe_dialogue.node_names()
    }

    /// Returns the string ID that contains the original, uncompiled source
    /// text for a node.
    ///
    /// A node's source text will only be present in the string table if its
    /// `tags` header contains `rawText`.
    ///
    /// Because the [`Dialogue`] API is designed to be unaware
    /// of the contents of the string table, this method does not test to
    /// see if the string table contains an entry with the line ID. You will
    /// need to test for that yourself.
    pub fn get_string_id_for_node(&self, node_name: &str) -> Option<String> {
        self.handler_safe_dialogue.get_string_id_for_node(node_name)
    }

    /// Returns the tags for the node `node_name`.
    ///
    /// The tags for a node are defined by setting the `tags` header in
    /// the node's source code. This header must be a space-separated list
    ///
    /// Returns [`None`] if the node is not present in the program.
    pub fn get_tags_for_node(&self, node_name: &str) -> Option<Vec<String>> {
        self.handler_safe_dialogue.get_tags_for_node(node_name)
    }

    /// Gets a value indicating whether a specified node exists in the Program.
    pub fn node_exists(&self, node_name: &str) -> bool {
        self.handler_safe_dialogue.node_exists(node_name)
    }

    /// Replaces all substitution markers in a text with the given
    /// substitution list.
    ///
    /// This method replaces substitution markers - for example, `{0}`
    /// - with the corresponding entry in `substitutions`.
    /// If `test` contains a substitution marker whose
    /// index is not present in `substitutions`, it is
    /// ignored.
    pub fn expand_substitutions<'a>(
        text: &str,
        substitutions: impl IntoIterator<Item = &'a str>,
    ) -> String {
        HandlerSafeDialogue::expand_substitutions(text, substitutions)
    }

    /// Gets the name of the node that this Dialogue is currently executing.
    ///
    /// If [`Dialogue::continue_`] has never been called, this value
    /// will be [`None`].
    pub fn current_node(&self) -> Option<String> {
        self.handler_safe_dialogue.current_node()
    }

    /// The [`Dialogue`]'s locale, as an IETF BCP 47 code.
    ///
    /// This code is used to determine how the `plural` and `ordinal`
    /// markers determine the plural class of numbers.
    ///
    /// For example, the code "en-US" represents the English language as
    /// used in the United States.
    pub fn language_code(&self) -> Option<String> {
        self.handler_safe_dialogue.language_code()
    }
    pub fn analyse(&self) -> ! {
        self.handler_safe_dialogue.analyse()
    }
    pub fn parse_markup(&self, line: &str) -> String {
        self.handler_safe_dialogue.parse_markup(line)
    }

    /// Signals to the [`Dialogue`] that the user has selected a specified [`DialogueOption`].
    ///
    /// After the Dialogue delivers an [`OptionSet`], this method must be called before [`Dialogue::continue_`] is called.
    ///
    /// The ID number that should be passed as the parameter to this method should be the [`DialogueOption::Id`]
    /// field in the [`DialogueOption`] that represents the user's selection.
    ///
    /// ## Panics
    /// - If the Dialogue is not expecting an option to be selected.
    /// - If the option ID is not found in the current [`OptionSet`].
    ///
    /// ## See Also
    /// - [`Dialogue::continue_`]
    /// - [`OptionsHandler`]
    /// - [`OptionSet`]
    pub fn set_selected_option(&mut self, selected_option_id: OptionId) -> &mut Self {
        self.handler_safe_dialogue
            .set_selected_option(selected_option_id);
        self
    }
    /// Gets a value indicating whether the Dialogue is currently executing Yarn instructions.
    pub fn is_active(&self) -> bool {
        self.handler_safe_dialogue.is_active()
    }
}

impl AsRef<HandlerSafeDialogue> for Dialogue {
    fn as_ref(&self) -> &HandlerSafeDialogue {
        &self.handler_safe_dialogue
    }
}

impl AsMut<HandlerSafeDialogue> for Dialogue {
    fn as_mut(&mut self) -> &mut HandlerSafeDialogue {
        &mut self.handler_safe_dialogue
    }
}

fn is_node_visited(variable_storage: &dyn VariableStorage, node_name: &str) -> bool {
    if let Some(YarnValue::Number(count)) = variable_storage.get(node_name) {
        count > 0.0
    } else {
        false
    }
}

fn get_node_visit_count(variable_storage: &dyn VariableStorage, node_name: &str) -> f32 {
    if let Some(YarnValue::Number(count)) = variable_storage.get(node_name) {
        count
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_handler() {
        let _dialogue = Dialogue::default()
            .set_log_debug_message(|_, _| {})
            .set_options_handler(|_, _| {});
    }

    #[test]
    fn is_send_sync() {
        let dialogue = Dialogue::default();
        accept_send_sync(dialogue);
    }

    fn accept_send_sync(_: impl Send + Sync) {}
}
