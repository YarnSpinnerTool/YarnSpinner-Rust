//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs
//!
//! ## Implementation Notes
//! The `Operand` extensions and the `Operator` enum were moved into upstream crates to make them not depend on the runtime.

pub(crate) use self::execution_state::*;
use self::state::*;
use crate::prelude::*;
use log::*;
use yarn_slinger_core::prelude::*;

mod execution_state;
mod state;

#[derive(Debug, Clone)]
pub(crate) struct VirtualMachine {
    pub(crate) program: Option<Program>,
    pub(crate) line_handler: LineHandler,

    pub(crate) log_debug_message: Logger,
    pub(crate) log_error_message: Logger,
    pub(crate) options_handler: OptionsHandler,
    pub(crate) command_handler: CommandHandler,
    pub(crate) node_start_handler: NodeStartHandler,
    pub(crate) node_complete_handler: NodeCompleteHandler,
    pub(crate) dialogue_complete_handler: DialogueCompleteHandler,
    pub(crate) prepare_for_lines_handler: PrepareForLinesHandler,
    state: State,
    execution_state: ExecutionState,
    current_node: Option<Node>,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            log_debug_message: Logger(Box::new(|msg: String| debug!("{}", msg))),
            log_error_message: Logger(Box::new(|msg: String| error!("{}", msg))),
            line_handler: LineHandler(Box::new(|line| {
                info!("Delivering line: {:?}", line);
            })),
            options_handler: OptionsHandler(Box::new(|options| {
                info!("Delivering options: {:?}", options);
            })),
            command_handler: CommandHandler(Box::new(|command| {
                info!("Executing command: {:?}", command);
            })),
            node_start_handler: NodeStartHandler(Box::new(|node_name| {
                info!("Starting node: {:?}", node_name);
            })),
            node_complete_handler: NodeCompleteHandler(Box::new(|node_name| {
                info!("Completed node: {:?}", node_name);
            })),
            dialogue_complete_handler: DialogueCompleteHandler(Box::new(|| {
                info!("Dialogue complete");
            })),
            prepare_for_lines_handler: PrepareForLinesHandler(Box::new(|line_ids| {
                info!("Preparing for lines: {:?}", line_ids);
            })),
            program: Default::default(),
            state: Default::default(),
            execution_state: Default::default(),
            current_node: Default::default(),
        }
    }
}

impl VirtualMachine {
    pub(crate) fn reset_state(&mut self) {
        self.state = State::default();
    }

    pub(crate) fn execution_state(&self) -> ExecutionState {
        self.execution_state
    }

    pub(crate) fn set_execution_state(&mut self, execution_state: ExecutionState) -> &mut Self {
        self.execution_state = execution_state;
        if self.execution_state == ExecutionState::Stopped {
            self.reset_state()
        }
        self
    }

    /// # Implementation Notes
    /// The original does not reset the state upon calling this. I suspect that's a bug.
    pub(crate) fn stop(&mut self) -> &mut Self {
        self.set_execution_state(ExecutionState::Stopped)
    }

    pub(crate) fn set_node(&self, _node_name: &str) {
        todo!()
    }

    pub(crate) fn set_selected_option(&mut self, selected_option_id: OptionId) {
        assert_ne!(ExecutionState::WaitingOnOptionSelection, self.execution_state, "SetSelectedOption was called, but Dialogue wasn't waiting for a selection. \
                This method should only be called after the Dialogue is waiting for the user to select an option.");

        assert!(
            selected_option_id.0 < self.state.current_options.len(),
            "{selected_option_id:?} is not a valid option ID (expected a number between 0 and {}.",
            self.state.current_options.len() - 1
        );

        // We now know what number option was selected; push the
        // corresponding node name to the stack.
        let destination_node = self.state.current_options[selected_option_id.0]
            .destination_node
            .clone();
        self.state.push(destination_node);

        // We no longer need the accumulated list of options; clear it
        // so that it's ready for the next one
        self.state.current_options.clear();

        // We're no longer in the WaitingForOptions state; we are now waiting for our game to let us continue
        self.execution_state = ExecutionState::WaitingForContinue;
    }

    /// Resumes execution.
    pub(crate) fn continue_(&mut self) {
        self.assert_can_continue();
        if self.execution_state == ExecutionState::DeliveringContent {
            // We were delivering a line, option set, or command, and
            // the client has called Continue() on us. We're still
            // inside the stack frame of the client callback, so to
            // avoid recursion, we'll note that our state has changed
            // back to Running; when we've left the callback, we'll
            // continue executing instructions.
            self.execution_state = ExecutionState::Running;
            return;
        }

        self.execution_state = ExecutionState::Running;

        // Execute instructions until something forces us to stop
        while self.execution_state == ExecutionState::Running {
            let current_node = self.current_node.as_ref().unwrap();
            let current_instruction = &current_node.instructions[self.state.program_counter];
            self.run_instruction(current_instruction);

            self.state.program_counter += 1;

            if self.state.program_counter < current_node.instructions.len() {
                continue;
            }

            self.node_complete_handler.call(current_node.name.clone());
            self.execution_state = ExecutionState::Stopped;
            self.dialogue_complete_handler.call();
            self.log_debug_message.call("Run complete.".to_owned());
        }
    }

    /// Runs a series of tests to see if the [`VirtualMachine`] is in a state where [`VirtualMachine::continue_`] can be called. Panics if it can't
    fn assert_can_continue(&self) {
        assert!(
            self.current_node.is_some(),
            "Cannot continue running dialogue. No node has been selected."
        );
        assert_eq!(
            ExecutionState::WaitingOnOptionSelection,
            self.execution_state,
            "Cannot continue running dialogue. Still waiting on option selection."
        );
    }

    pub(crate) fn current_node_name(&self) -> Option<&str> {
        self.state.current_node_name.as_deref()
    }

    pub(crate) fn unload_programs(&mut self) {
        self.program = None
    }

    fn run_instruction(&self, _current_instruction: &Instruction) {
        todo!()
    }
}
