//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs
//!
//! ## Implementation Notes
//! The `Operand` extensions and the `Operator` enum were moved into upstream crates to make them not depend on the runtime.

pub(crate) use self::{execution_state::*, state::*};
use crate::prelude::*;
use log::*;
use std::fmt::Debug;
use yarn_slinger_core::prelude::instruction::OpCode;
use yarn_slinger_core::prelude::*;

mod execution_state;
mod state;

#[derive(Debug)]
pub(crate) struct VirtualMachine {
    pub(crate) line_handler: LineHandler,
    pub(crate) options_handler: OptionsHandler,
    pub(crate) command_handler: CommandHandler,
    pub(crate) node_start_handler: Option<NodeStartHandler>,
    pub(crate) node_complete_handler: NodeCompleteHandler,
    pub(crate) dialogue_complete_handler: Option<DialogueCompleteHandler>,
    pub(crate) prepare_for_lines_handler: Option<PrepareForLinesHandler>,
    pub(crate) library: Library,
    handler_safe_dialogue: HandlerSafeDialogue,
    shared: SharedState,
    log_debug_message: Logger,
    log_error_message: Logger,
}

impl SharedStateHolder for VirtualMachine {
    fn shared_state(&self) -> &SharedState {
        &self.shared
    }
}

impl VirtualMachine {
    pub(crate) fn with_shared_state(shared_state: SharedState) -> Self {
        let dialogue_data = HandlerSafeDialogue::from_shared_state(shared_state.clone());
        fn default_line_handler(line: Line, _dialogue: &mut HandlerSafeDialogue) {
            info!("Delivering line: {:?}\nTo handle this command on your own, register a handler via `Dialogue::with_line_handler`.", line);
        }
        fn default_options_handler(
            options: Vec<DialogueOption>,
            _dialogue: &mut HandlerSafeDialogue,
        ) {
            info!("Delivering options: {:?}\nTo handle this command on your own, register a handler via `Dialogue::with_options_handler`.", options);
        }
        fn default_command_handler(command: Command, _dialogue: &mut HandlerSafeDialogue) {
            info!("Executing command: {:?}\nTo handle this command on your own, register a handler via `Dialogue::with_command_handler`.", command);
        }
        fn default_node_complete_handler(node_name: String, _dialogue: &mut HandlerSafeDialogue) {
            info!("Completed node: {:?}\nTo handle this command on your own, register a handler via `Dialogue::with_node_complete_handler`.", node_name);
        }
        Self {
            log_debug_message: dialogue_data.log_debug_message.clone(),
            log_error_message: dialogue_data.log_error_message.clone(),
            line_handler: Box::new(default_line_handler),
            options_handler: Box::new(default_options_handler),
            command_handler: Box::new(default_command_handler),
            node_start_handler: Default::default(),
            node_complete_handler: Box::new(default_node_complete_handler),
            dialogue_complete_handler: Default::default(),
            prepare_for_lines_handler: Default::default(),
            handler_safe_dialogue: dialogue_data,
            shared: shared_state,
            library: Library::standard_library(),
        }
    }

    pub(crate) fn reset_state(&mut self) {
        *self.state_mut() = State::default();
    }

    pub(crate) fn set_execution_state(&mut self, execution_state: ExecutionState) -> &mut Self {
        *self.execution_state_mut() = execution_state;
        if execution_state == ExecutionState::Stopped {
            self.reset_state()
        }
        self
    }

    /// # Implementation Notes
    /// The original does not reset the state upon calling this. I suspect that's a bug.
    pub(crate) fn stop(&mut self) -> &mut Self {
        self.set_execution_state(ExecutionState::Stopped)
    }

    pub(crate) fn set_node(&mut self, node_name: &str) {
        self.log_debug_message.call(
            format!("Running node \"{node_name}\""),
            &self.handler_safe_dialogue,
        );

        *self.current_node_mut() = {
            let program = self.program();
            let program = program.as_ref().unwrap_or_else(|| {
                panic!("Cannot load node \"{node_name}\": No nodes have been loaded.")
            });
            assert!(
                !program.nodes.is_empty(),
                "Cannot load node \"{node_name}\": No nodes have been loaded.",
            );
            program.nodes.get(node_name).cloned()
        };
        self.reset_state();
        {
            *self.current_node_name_mut() = Some(node_name.to_owned());
        }
        {
            let mut guard = self.current_node_mut();
            let current_node = guard
                .as_mut()
                .unwrap_or_else(|| panic!("No node named \"{node_name}\" has been loaded."));

            current_node.name = node_name.to_owned();
        }

        if let Some(node_start_handler) = &mut self.node_start_handler {
            node_start_handler.call(node_name.to_owned(), &mut self.handler_safe_dialogue);
        }

        // Do we have a way to let the client know that certain lines
        // might be run?
        if self.prepare_for_lines_handler.is_none() {
            return;
        };

        // If we have a prepare-for-lines handler, figure out what
        // lines we anticipate running

        // Create a list; we will never have more lines and options
        // than total instructions, so that's a decent capacity for
        // the list
        // [sic] TODO: maybe this list could be reused to save on allocations?

        let string_ids = self
            .current_node()
            .as_ref()
            .unwrap()
            .instructions
            .iter()
            // Loop over every instruction and find the ones that run a
            // line or add an option; these are the two instructions
            // that will signal a line can appear to the player
            .filter_map(|instruction| {
                let opcode: OpCode = instruction.opcode.try_into().unwrap();
                [OpCode::RunLine, OpCode::AddOption]
                    .contains(&opcode)
                    .then(|| {
                        // Both RunLine and AddOption have the string ID
                        // they want to show as their first operand, so
                        // store that
                        let id: String = instruction.operands[0].clone().try_into().unwrap();
                        LineId(id)
                    })
            })
            .collect();
        let prepare_for_lines_handler = self.prepare_for_lines_handler.as_mut().unwrap();
        prepare_for_lines_handler.call(string_ids, &mut self.handler_safe_dialogue);
    }

    /// Resumes execution.
    pub(crate) fn continue_(&mut self) {
        self.assert_can_continue();
        if *self.execution_state() == ExecutionState::DeliveringContent {
            // We were delivering a line, option set, or command, and
            // the client has called Continue() on us. We're still
            // inside the stack frame of the client callback, so to
            // avoid recursion, we'll note that our state has changed
            // back to Running; when we've left the callback, we'll
            // continue executing instructions.
            *self.execution_state_mut() = ExecutionState::Running;
            return;
        }

        *self.execution_state_mut() = ExecutionState::Running;

        // Execute instructions until something forces us to stop
        while *self.execution_state() == ExecutionState::Running {
            let current_node = self.current_node().clone().unwrap();
            let current_instruction = &current_node.instructions[self.state().program_counter];
            self.run_instruction(current_instruction);
            // ## Implementation note
            // The original increments the program counter here, but that leads to intentional underflow on [`OpCode::RunNode`],
            // so we do the incrementation in [`VirtualMachine::run_instruction`] instead.

            if self.state().program_counter < current_node.instructions.len() {
                continue;
            }

            self.node_complete_handler
                .call(current_node.name.clone(), &mut self.handler_safe_dialogue);
            *self.execution_state_mut() = ExecutionState::Stopped;
            if let Some(dialogue_complete_handler) = &mut self.dialogue_complete_handler {
                dialogue_complete_handler.call(&mut self.handler_safe_dialogue);
            }
            self.log_debug_message
                .call("Run complete.".to_owned(), &self.handler_safe_dialogue);
        }
    }

    /// Runs a series of tests to see if the [`VirtualMachine`] is in a state where [`VirtualMachine::continue_`] can be called. Panics if it can't.
    fn assert_can_continue(&self) {
        assert!(
            self.current_node().is_some(),
            "Cannot continue running dialogue. No node has been selected."
        );
        assert_ne!(
            ExecutionState::WaitingOnOptionSelection,
            *self.execution_state(),
            "Cannot continue running dialogue. Still waiting on option selection."
        );
        // ## Implementation note:
        // The other checks the original did are not needed because our relevant handlers cannot be `None` per our API.
    }

    pub(crate) fn unload_programs(&mut self) {
        *self.program_mut() = None
    }

    pub(crate) fn set_log_debug_message(
        &mut self,
        logger: impl Fn(String, &HandlerSafeDialogue) + Clone + 'static + Send + Sync,
    ) -> &mut Self {
        self.log_debug_message = Box::new(logger);
        self.handler_safe_dialogue.log_debug_message = self.log_debug_message.clone();
        self
    }

    pub(crate) fn set_log_error_message(
        &mut self,
        logger: impl Fn(String, &HandlerSafeDialogue) + Clone + 'static + Send + Sync,
    ) -> &mut Self {
        self.log_error_message = Box::new(logger);
        self.handler_safe_dialogue.log_error_message = self.log_error_message.clone();
        self
    }

    /// ## Implementation note
    ///
    /// Increments the program counter here instead of in `continue_` for cleaner code
    fn run_instruction(&mut self, instruction: &Instruction) {
        let opcode: OpCode = instruction.opcode.try_into().unwrap();
        match opcode {
            OpCode::JumpTo => {
                // Jumps to a named label
                let label_name: String = instruction.read_operand(0);
                self.state_mut().program_counter =
                    self.find_instruction_point_for_label(&label_name);
            }
            OpCode::Jump => {
                // Jumps to a label whose name is on the stack.
                let jump_destination: String = self.state().peek();
                self.state_mut().program_counter =
                    self.find_instruction_point_for_label(&jump_destination);
            }
            OpCode::RunLine => {
                // Looks up a string from the string table and passes it to the client as a line

                let string_id: String = instruction.read_operand(0);

                // The second operand, if provided (compilers prior
                // to v1.1 don't include it), indicates the number
                // of expressions in the line. We need to pop these
                // values off the stack and deliver them to the
                // line handler.
                assert_up_to_date_compiler(instruction.operands.len() >= 2);

                let strings = self.pop_substitutions_with_count_at_index(instruction, 1);

                let line = Line {
                    id: string_id.into(),
                    substitutions: strings,
                };

                // Suspend execution, because we're about to deliver content
                *self.execution_state_mut() = ExecutionState::DeliveringContent;

                self.line_handler
                    .call(line, &mut self.handler_safe_dialogue);

                // Implementation note:
                // In the original, this is only done if `execution_state` is still `DeliveringContent`,
                // because the line handler is allowed to call `continue_`. However, we disallow that because of
                // how this violates borrow checking. So, we'll always wait at this point instead until the user
                // called `continue_` themselves outside of the line handler.
                *self.execution_state_mut() = ExecutionState::WaitingForContinue;
                self.state_mut().program_counter += 1;
            }
            OpCode::RunCommand => {
                // Passes a string to the client as a custom command
                let command_text: String = instruction.read_operand(0);
                assert_up_to_date_compiler(instruction.operands.len() >= 2);
                let command_text = self
                    .pop_substitutions_with_count_at_index(instruction, 1)
                    .into_iter()
                    .enumerate()
                    .fold(command_text, |command_text, (i, substitution)| {
                        command_text.replace(&format!("{{{i}}}"), &substitution)
                    });
                *self.execution_state_mut() = ExecutionState::DeliveringContent;
                let command = Command(command_text);

                self.command_handler
                    .call(command, &mut self.handler_safe_dialogue);

                // Implementation note:
                // In the original, this is only done if `execution_state` is still `DeliveringContent`,
                // because the line handler is allowed to call `continue_`. However, we disallow that because of
                // how this violates borrow checking. So, we'll always wait at this point instead until the user
                // called `continue_` themselves outside of the line handler.
                *self.execution_state_mut() = ExecutionState::WaitingForContinue;
                self.state_mut().program_counter += 1;
            }
            OpCode::AddOption => {
                // Add an option to the current state
                let string_id: String = instruction.read_operand(0);
                assert_up_to_date_compiler(instruction.operands.len() >= 4);
                let strings = self.pop_substitutions_with_count_at_index(instruction, 2);
                let line = Line {
                    id: string_id.into(),
                    substitutions: strings,
                };

                // Indicates whether the VM believes that the
                // option should be shown to the user, based on any
                // conditions that were attached to the option.
                let line_condition_passed = if instruction.read_operand(3) {
                    // The fourth operand is a bool that indicates
                    // whether this option had a condition or not.
                    // If it does, then a bool value will exist on
                    // the stack indicating whether the condition
                    // passed or not. We pass that information to
                    // the game.
                    self.state_mut().pop()
                } else {
                    true
                };

                let index = self.state().current_options.len();
                let node_name = instruction.read_operand(1);
                // ## Implementation note:
                // The original calculates the ID in the `ShowOptions` opcode,
                // but this way is cleaner because it allows us to store a `DialogueOption` instead of a bunch of values in a big tuple.
                self.state_mut().current_options.push(DialogueOption {
                    line,
                    id: OptionId(index),
                    destination_node: node_name,
                    is_available: line_condition_passed,
                });
                self.state_mut().program_counter += 1;
            }
            OpCode::ShowOptions => {
                // If we have no options to show, immediately stop.
                if self.state().current_options.is_empty() {
                    *self.execution_state_mut() = ExecutionState::Stopped;
                    if let Some(dialogue_complete_handler) = &mut self.dialogue_complete_handler {
                        dialogue_complete_handler.call(&mut self.handler_safe_dialogue);
                    }
                    self.state_mut().program_counter += 1;
                    return;
                }

                // We can't continue until our client tell us which option to pick
                *self.execution_state_mut() = ExecutionState::WaitingOnOptionSelection;

                // Pass the options set to the client, as well as a
                // delegate for them to call when the user has made
                // a selection
                let current_options = self.state().current_options.clone();
                self.options_handler
                    .call(current_options, &mut self.handler_safe_dialogue);

                if *self.execution_state() == ExecutionState::WaitingForContinue {
                    // we are no longer waiting on an option
                    // selection - the options handler must have
                    // called SetSelectedOption! Continue running
                    // immediately.
                    *self.execution_state_mut() = ExecutionState::Running;
                }
                self.state_mut().program_counter += 1;
            }
            OpCode::PushString => {
                //Pushes a string value onto the stack. The operand is an index into the string table, so that's looked up first.
                let string_table_index: String = instruction.read_operand(0);
                self.state_mut().push(string_table_index);
                self.state_mut().program_counter += 1;
            }
            OpCode::PushFloat => {
                // Pushes a floating point onto the stack.
                let float: f32 = instruction.read_operand(0);
                self.state_mut().push(float);
                self.state_mut().program_counter += 1;
            }
            OpCode::PushBool => {
                // Pushes a boolean value onto the stack.
                let boolean: bool = instruction.read_operand(0);
                self.state_mut().push(boolean);
                self.state_mut().program_counter += 1;
            }

            OpCode::PushNull => {
                panic!("PushNull is no longer valid op code, because null is no longer a valid value from Yarn Spinner 2.0 onwards. To fix this error, re-compile the original source code.");
            }
            OpCode::JumpIfFalse => {
                // Jumps to a named label if the value on the top of the stack evaluates to the boolean value 'false'.
                let is_top_value_true: bool = self.state_mut().peek();
                if !is_top_value_true {
                    let label_name: String = instruction.read_operand(0);
                    let instruction_point = self.find_instruction_point_for_label(&label_name);
                    self.state_mut().program_counter = instruction_point;
                } else {
                    self.state_mut().program_counter += 1;
                }
            }
            OpCode::Pop => {
                // Pops a value from the stack.
                self.state_mut().pop_value();
                self.state_mut().program_counter += 1;
            }
            OpCode::CallFunc => {
                let actual_parameter_count: usize = self.state_mut().pop();
                // Get the parameters, which were pushed in reverse
                let parameters = {
                    let mut parameters: Vec<_> = (0..actual_parameter_count)
                        .rev()
                        .map(|_| self.state_mut().pop_value().raw_value)
                        .collect();
                    parameters.reverse();
                    parameters
                };

                // Call a function, whose parameters are expected to be on the stack. Pushes the function's return value, if it returns one.
                let function_name: String = instruction.read_operand(0);
                let function = self.library.get(&function_name).unwrap_or_else(|| {
                    panic!(
                        "Function \"{}\" not found in library: {}",
                        function_name, self.library
                    )
                });

                // Expect the compiler to have placed the number of parameters
                // actually passed at the top of the stack.
                let expected_parameter_count = function.parameter_types().len();

                assert_eq!(
                    expected_parameter_count, actual_parameter_count,
                    "Function {function_name} expected {expected_parameter_count} parameters, but received {actual_parameter_count}",
                );

                // Invoke the function
                let return_value = function.call(parameters);
                let return_type = function
                    .return_type()
                    .try_into()
                    .unwrap_or_else(|e| panic!("Failed to get Yarn type for return type id of function {function_name}: {e:?}"));
                let typed_return_value = InternalValue {
                    raw_value: return_value,
                    r#type: return_type,
                };
                // ## Implementation note:
                // The original code first checks whether the return type is `void`. This is vestigial from the v1 compiler.
                // In current Yarn, every function MUST return a valid typed value, so we skip that check.
                self.state_mut().push(typed_return_value);
                self.state_mut().program_counter += 1;
            }
            OpCode::PushVariable => {
                // Get the contents of a variable, push that onto the stack.
                let variable_name: String = instruction.read_operand(0);
                let loaded_value = self
                    .variable_storage()
                    .get(&variable_name)
                    .unwrap_or_else(|| {
                        // We don't have a value for this. The initial
                        // value may be found in the program. (If it's
                        // not, then the variable's value is undefined,
                        // which isn't allowed.)

                        self
                            .program()
                            .as_ref()
                            .unwrap()
                            .initial_values
                            .get(&variable_name)
                            .unwrap_or_else(|| panic!("The loaded program does not contain an initial value for the variable {variable_name}"))
                            .clone()
                            .into()
                    });
                self.state_mut().push(loaded_value);
                self.state_mut().program_counter += 1;
            }
            OpCode::StoreVariable => {
                // Store the top value on the stack in a variable.
                let top_value = self.state().peek_value().clone();
                let variable_name: String = instruction.read_operand(0);
                self.variable_storage_mut()
                    .set(variable_name, top_value.into());
                self.state_mut().program_counter += 1;
            }
            OpCode::Stop => {
                // Immediately stop execution, and report that fact.
                let current_node_name = self.current_node_name().clone().unwrap();
                self.node_complete_handler
                    .call(current_node_name, &mut self.handler_safe_dialogue);
                if let Some(dialogue_complete_handler) = &mut self.dialogue_complete_handler {
                    dialogue_complete_handler.call(&mut self.handler_safe_dialogue);
                }
                *self.execution_state_mut() = ExecutionState::Stopped;
                self.state_mut().program_counter += 1;
            }
            OpCode::RunNode => {
                // Run a node

                // Pop a string from the stack, and jump to a node
                // with that name.
                let node_name: String = self.state_mut().pop();
                self.node_complete_handler
                    .call(node_name.clone(), &mut self.handler_safe_dialogue);
                self.set_node(&node_name);

                // No need to increment the program counter, since otherwise we'd skip the first instruction
            }
        }
    }

    /// Looks up the instruction number for a named label in the current node.
    ///
    /// # Panics
    ///
    /// Panics in the following cases:
    /// - The label is not found in the current node
    /// - The current node is unset
    /// - The found instruction point is negative
    fn find_instruction_point_for_label(&self, label_name: &str) -> usize {
        self.current_node()
            .as_ref()
            .unwrap()
            .labels
            .get(label_name)
            .copied()
            .unwrap_or_else(|| {
                panic!(
                    "Unknown label {} in node {}",
                    label_name,
                    self.current_node_name().as_ref().unwrap()
                )
            })
            .try_into()
            .unwrap()
    }

    fn pop_substitutions_with_count_at_index(
        &mut self,
        instruction: &Instruction,
        index: usize,
    ) -> Vec<String> {
        let expression_count: usize = instruction.operands[index].clone().try_into().unwrap();
        let mut values: Vec<_> = (0..expression_count)
            .rev()
            .map(|_| self.state_mut().pop())
            .collect();
        values.reverse();
        values
    }
}

fn assert_up_to_date_compiler(predicate: bool) {
    assert!(
        predicate,
        "The Yarn script provided was compiled using an older compiler. \
        Please recompile it using the latest version of either Yarn Slinger or Yarn Spinner."
    )
}
