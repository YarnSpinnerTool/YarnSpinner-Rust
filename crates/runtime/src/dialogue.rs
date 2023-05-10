use crate::prelude::*;
use log::error;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
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
    /// The [`Dialogue`]'s locale, as an IETF BCP 47 code.
    ///
    /// This code is used to determine how the `plural` and `ordinal`
    /// markers determine the plural class of numbers.
    ///
    /// For example, the code "en-US" represents the English language as
    /// used in the United States.
    pub language_code: Option<String>,
}

impl Default for Dialogue {
    fn default() -> Self {
        let variable_storage: Arc<RwLock<Box<dyn VariableStorage + Send + Sync>>> =
            Arc::new(RwLock::new(Box::new(MemoryVariableStore::new())));

        let storage_one = variable_storage.clone();
        let storage_two = storage_one.clone();

        let library = Library::standard_library()
            .with_function("visited", move |node: String| -> bool {
                is_node_visited(storage_one.read().unwrap().deref().as_ref(), &node)
            })
            .with_function("visited_count", move |node: String| -> f32 {
                get_node_visit_count(storage_two.read().unwrap().deref().as_ref(), &node)
            });
        Self {
            vm: VirtualMachine::new(library, variable_storage),
            language_code: None,
        }
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
        *self.vm.variable_storage.write().unwrap() = Box::new(variable_storage);
        self
    }

    pub fn set_language_code(&mut self, language_code: impl Into<String>) -> &mut Self {
        self.language_code.replace(language_code.into());
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
        SharedMemoryVariableStore(self.vm.variable_storage.clone())
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
        self.vm
            .program
            .as_ref()
            .map(|program| program.nodes.keys().cloned().collect())
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
        self.get_node_logging_errors(node_name)
            .map(|_| format!("line:{node_name}"))
    }

    /// Returns the tags for the node `node_name`.
    ///
    /// The tags for a node are defined by setting the `tags` header in
    /// the node's source code. This header must be a space-separated list
    ///
    /// Returns [`None`] if the node is not present in the program.
    pub fn get_tags_for_node(&self, node_name: &str) -> Option<Vec<String>> {
        self.get_node_logging_errors(node_name)
            .map(|node| node.tags)
    }

    /// Gets a value indicating whether a specified node exists in the Program.
    pub fn node_exists(&self, node_name: &str) -> bool {
        // Not calling `get_node_logging_errors` because this method does not write errors when there are no nodes.
        if let Some(program) = self.vm.program.as_ref() {
            program.nodes.contains_key(node_name)
        } else {
            error!("Tried to call NodeExists, but no program has been loaded");
            false
        }
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
        substitutions
            .into_iter()
            .enumerate()
            .fold(text.to_owned(), |text, (i, substitution)| {
                text.replace(&format!("{{{i}}}",), substitution)
            })
    }

    /// Gets the name of the node that this Dialogue is currently executing.
    ///
    /// If [`Dialogue::continue_`] has never been called, this value
    /// will be [`None`].
    pub fn current_node(&self) -> Option<String> {
        self.vm.current_node_name.clone()
    }

    pub fn analyse(&self) -> ! {
        todo!()
    }

    pub fn parse_markup(&self, line: &str) -> String {
        // ## Implementation notes
        // It would be more ergonomic to not expose this and call it automatically.
        // We should probs remove this from the API.
        // Pass the MarkupResult directly into the LineHandler
        // todo!()
        line.to_owned()
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
    pub fn set_selected_option(&mut self, selected_option_id: OptionId) {
        self.vm.set_selected_option(selected_option_id);
    }

    /// Gets a value indicating whether the Dialogue is currently executing Yarn instructions.
    pub fn is_active(&self) -> bool {
        self.vm.is_active()
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
