use crate::prelude::*;
use log::*;
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
    log_debug_message: Logger,

    /// Invoked when the Dialogue needs to report an error.
    log_error_message: Logger,

    /// The [`Dialogue`]'s locale, as an IETF BCP 47 code.
    ///
    /// This code is used to determine how the `plural` and `ordinal`
    /// markers determine the plural class of numbers.
    ///
    /// For example, the code "en-US" represents the English language as
    /// used in the United States.
    pub language_code: Option<String>,

    /// The node that execution will start from.
    program: Option<Program>,

    vm: VirtualMachine,
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
            log_debug_message: Logger(Box::new(|msg| debug!("{msg}"))),
            log_error_message: Logger(Box::new(|msg| error!("{msg}"))),
            language_code: Default::default(),
            program: Default::default(),
            vm: Default::default(),
        }
    }
}

impl Dialogue {
    const DEFAULT_START_NODE_NAME: &'static str = "Start";

    /// Initializes a new instance of the [`Dialogue`] class.
    pub fn with_variable_storage(self, variable_storage: impl VariableStorage + 'static) -> Self {
        Self {
            variable_storage: Box::new(variable_storage),
            ..self
        }
    }

    pub fn with_log_debug_message(self, logger: impl Fn(String) + Clone + 'static) -> Self {
        Self {
            log_debug_message: Logger(Box::new(logger)),
            ..self
        }
    }

    pub fn with_log_error_message(self, logger: impl Fn(String) + Clone + 'static) -> Self {
        Self {
            log_error_message: Logger(Box::new(logger)),
            ..self
        }
    }

    pub fn with_line_handler(self, line_handler: impl Fn(Line) + Clone + 'static) -> Self {
        todo!()
    }

    pub fn with_options_handler(
        self,
        options_handler: impl Fn(Vec<DialogueOption>) + Clone + 'static,
    ) -> Self {
        todo!()
    }

    pub fn with_command_handler(self, command_handler: impl Fn(Command) + Clone + 'static) -> Self {
        todo!()
    }

    pub fn with_node_complete_handler(
        self,
        node_complete_handler: impl Fn(NodeName) + Clone + 'static,
    ) -> Self {
        todo!()
    }

    pub fn with_node_start_handler(
        self,
        node_start_handler: impl Fn(NodeName) + Clone + 'static,
    ) -> Self {
        todo!()
    }

    pub fn with_dialogue_complete_handler(
        self,
        dialogue_complete_handler: impl Fn() + Clone + 'static,
    ) -> Self {
        todo!()
    }

    pub fn with_prepare_for_lines_handler(
        self,
        prepare_for_lines_handler: impl Fn(Vec<LineId>) + Clone + 'static,
    ) -> Self {
        todo!()
    }

    pub fn with_language_code(self, language_code: impl Into<String>) -> Self {
        Self {
            language_code: Some(language_code.into()),
            ..self
        }
    }

    pub fn log_debug_message(&self) -> &Logger {
        &self.log_debug_message
    }

    pub fn log_error_message(&self) -> &Logger {
        &self.log_error_message
    }

    /// The [`OptionsHandler`] that is called when a set of options are ready to be shown to the user.
    ///
    /// The Options Handler delivers a [`Vec`] of [`DialogueOption`] to the game.
    /// Before [`Dialogue::continue`] can be called to resume execution,
    /// [`Dialogue::set_selected_option`] must be called to indicate which
    /// [`DialogueOption`] was selected by the user. If [`Dialogue::set_selected_option`] is not called, a panic occurs.
    pub fn options_handler(&self) -> &OptionsHandler {
        todo!()
    }

    pub fn line_handler(&self) -> &LineHandler {
        todo!()
    }

    /// The [`CommandHandler`] that is called when a command is to be delivered to the game.
    pub fn command_handler(&self) -> &CommandHandler {
        todo!()
    }

    /// The [`NodeCompleteHandler`] that is called when a node is complete.
    pub fn node_complete_handler(&self) -> &NodeCompleteHandler {
        todo!()
    }

    /// The [`NodeStartHandler`] that is called when a node is started.
    pub fn node_start_handler(&self) -> &NodeStartHandler {
        todo!()
    }

    /// The [`DialogueCompleteHandler`] that is called when the Dialogue reaches its end.
    pub fn dialogue_complete_handler(&self) -> &DialogueCompleteHandler {
        todo!()
    }

    /// The [`PrepareForLinesHandler`] that is called when the dialogue anticipates delivering some lines.
    pub fn prepare_for_lines_handler(&self) -> &PrepareForLinesHandler {
        todo!()
    }

    /// Gets a value indicating whether the Dialogue is currently executing Yarn instructions.
    pub fn is_active(&self) -> bool {
        todo!()
    }

    pub(crate) fn program(&self) -> Option<&Program> {
        self.program.as_ref()
    }

    pub(crate) fn with_new_program(mut self, program: Program) -> Self {
        self.set_program(program);
        self
    }

    pub(crate) fn with_added_program(mut self, program: Program) -> Self {
        self.add_program(program);
        self
    }

    pub(crate) fn set_program(&mut self, program: Program) -> &mut Self {
        self.program = Some(program.clone());
        self.vm.program = program;
        self.vm.reset_state();
        self
    }

    pub fn add_program(&mut self, program: Program) -> &mut Self {
        if let Some(existing_program) = &mut self.program {
            *existing_program = Program::combine(vec![existing_program.clone(), program]).unwrap();
        } else {
            self.set_program(program);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_handler() {
        let dialogue = Dialogue::default()
            .with_log_debug_message(|_| {})
            .with_options_handler(|_| {});
        let _cloned = dialogue.clone();
    }
}
