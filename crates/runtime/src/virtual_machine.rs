//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs>
//!
//! ## Implementation Notes
//! The `Operand` extensions and the `Operator` enum were moved into upstream crates to make them not depend on the runtime.

pub(crate) use self::{execution_state::*, state::*};
use crate::Result;
use crate::markup::{LineParser, ParsedMarkup};
use crate::prelude::*;
use core::fmt::Debug;
use log::*;

mod execution_state;
mod state;

#[derive(Debug, Clone)]
pub(crate) struct VirtualMachine {
    pub(crate) library: Library,
    pub(crate) program: Option<Program>,
    pub(crate) variable_storage: Box<dyn VariableStorage>,
    pub(crate) line_hints_enabled: bool,
    current_node_name: Option<String>,
    state: State,
    execution_state: ExecutionState,
    current_node: Option<Node>,
    batched_events: Vec<DialogueEvent>,
    line_parser: LineParser,
    text_provider: Box<dyn TextProvider>,
    language_code: Option<Language>,
}

impl VirtualMachine {
    pub(crate) fn new(
        library: Library,
        variable_storage: Box<dyn VariableStorage>,
        line_parser: LineParser,
        text_provider: Box<dyn TextProvider>,
    ) -> Self {
        Self {
            library,
            variable_storage,
            line_parser,
            text_provider,
            language_code: Default::default(),
            program: Default::default(),
            current_node_name: Default::default(),
            state: Default::default(),
            execution_state: Default::default(),
            current_node: Default::default(),
            batched_events: Default::default(),
            line_hints_enabled: Default::default(),
        }
    }

    pub(crate) fn text_provider(&self) -> &dyn TextProvider {
        self.text_provider.as_ref()
    }

    pub(crate) fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.text_provider.as_mut()
    }

    pub(crate) fn variable_storage(&self) -> &dyn VariableStorage {
        self.variable_storage.as_ref()
    }

    pub(crate) fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.variable_storage.as_mut()
    }

    pub(crate) fn set_language_code(&mut self, language_code: impl Into<Option<Language>>) {
        let language_code = language_code.into();
        self.language_code.clone_from(&language_code);
        self.line_parser.set_language_code(language_code.clone());
        self.text_provider.set_language(language_code);
    }

    pub(crate) fn reset_state(&mut self) {
        self.state = State::default();
        self.current_node_name = None;
    }

    pub(crate) fn set_execution_state(&mut self, execution_state: ExecutionState) -> &mut Self {
        self.execution_state = execution_state;
        if execution_state == ExecutionState::Stopped {
            self.reset_state()
        }
        self
    }

    /// # Implementation Notes
    /// The original does not reset the state upon calling this. I suspect that's a bug.
    pub(crate) fn stop(&mut self) -> Vec<DialogueEvent> {
        self.set_execution_state(ExecutionState::Stopped);
        self.batched_events.push(DialogueEvent::DialogueComplete);
        core::mem::take(&mut self.batched_events)
    }

    pub(crate) fn set_node(&mut self, node_name: impl Into<String>) -> Result<()> {
        let node_name = node_name.into();
        debug!("Loading node \"{node_name}\"");
        let current_node = self.get_node_from_name(&node_name)?;
        self.current_node = Some(current_node.clone());

        self.reset_state();

        self.current_node_name = Some(node_name.clone());

        self.batched_events
            .push(DialogueEvent::NodeStart(node_name));

        if self.line_hints_enabled {
            self.send_line_hints();
        }
        Ok(())
    }

    fn send_line_hints(&mut self) {
        // Create a list; we will never have more lines and options
        // than total instructions, so that's a decent capacity for
        // the list
        // [sic] TODO: maybe this list could be reused to save on allocations?

        let string_ids: Vec<_> = self
            .current_node
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
        self.text_provider.accept_line_hints(&string_ids);
        self.batched_events
            .push(DialogueEvent::LineHints(string_ids));
    }

    pub(crate) fn pop_line_hints(&mut self) -> Option<Vec<LineId>> {
        match self.batched_events.pop() {
            Some(DialogueEvent::LineHints(string_ids)) => Some(string_ids),
            Some(event) => {
                self.batched_events.push(event);
                None
            }
            None => None,
        }
    }

    fn get_node_from_name(&self, node_name: &str) -> Result<&Node> {
        let program = self
            .program
            .as_ref()
            .ok_or_else(|| DialogueError::NoProgramLoaded)?;
        assert!(
            !program.nodes.is_empty(),
            "Cannot load node \"{node_name}\": No nodes have been loaded.",
        );

        program
            .nodes
            .get(node_name)
            .ok_or_else(|| DialogueError::InvalidNode {
                node_name: node_name.to_owned(),
            })
    }

    /// Resumes execution.
    pub(crate) fn continue_(
        &mut self,
        mut instruction_fn: impl FnMut(&mut Self, &Instruction) -> crate::Result<()>,
    ) -> crate::Result<Vec<DialogueEvent>> {
        self.assert_can_continue()?;
        self.set_execution_state(ExecutionState::Running);

        while self.execution_state == ExecutionState::Running {
            let current_node = self.current_node.clone().unwrap();
            let current_instruction = &current_node.instructions[self.state.program_counter];
            instruction_fn(self, current_instruction)?;
            // ## Implementation note
            // The original increments the program counter here, but that leads to intentional underflow on [`OpCode::RunNode`],
            // so we do the incrementation in [`VirtualMachine::run_instruction`] instead.

            if self.state.program_counter < current_node.instructions.len() {
                continue;
            }

            self.batched_events
                .push(DialogueEvent::NodeComplete(current_node.name.clone()));
            self.set_execution_state(ExecutionState::Stopped);
            self.batched_events.push(DialogueEvent::DialogueComplete);
            debug!("Run complete.");
        }
        Ok(core::mem::take(&mut self.batched_events))
    }

    pub(crate) fn parse_markup(&mut self, line: &str) -> crate::markup::Result<ParsedMarkup> {
        self.line_parser.parse_markup(line)
    }

    /// Runs a series of tests to see if the [`VirtualMachine`] is in a state where [`VirtualMachine::r#continue`] can be called. Panics if it can't.
    pub(crate) fn assert_can_continue(&self) -> crate::Result<()> {
        if self.current_node.is_none() || self.current_node_name.is_none() {
            Err(DialogueError::NoNodeSelectedOnContinue)
        } else if self.execution_state == ExecutionState::WaitingOnOptionSelection {
            Err(DialogueError::ContinueOnOptionSelectionError)
        } else {
            // ## Implementation note:
            // The other checks the original did are not needed because our relevant handlers cannot be `None` per our API.
            Ok(())
        }
    }

    pub(crate) fn unload_programs(&mut self) {
        self.program = None
    }

    pub(crate) fn set_selected_option(&mut self, selected_option_id: OptionId) -> Result<()> {
        if self.execution_state != ExecutionState::WaitingOnOptionSelection {
            return Err(DialogueError::UnexpectedOptionSelectionError);
        }
        if selected_option_id.0 >= self.state.current_options.len() {
            return Err(DialogueError::InvalidOptionIdError {
                selected_option_id,
                max_id: self.state.current_options.len().saturating_sub(1),
            });
        }

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
        self.set_execution_state(ExecutionState::WaitingForContinue);
        Ok(())
    }

    pub(crate) fn set_selected_option_by_line_id(
        &mut self,
        selected_line_id: LineId,
    ) -> Result<OptionId> {
        if self.execution_state != ExecutionState::WaitingOnOptionSelection {
            return Err(DialogueError::UnexpectedOptionSelectionError);
        }
        if let Some(selected_option) = self
            .state
            .current_options
            .iter()
            .find(|o| o.line.id == selected_line_id)
        {
            let selected_option_id = selected_option.id;
            self.set_selected_option(selected_option_id)
                .map(|_| selected_option_id)
        } else {
            let line_ids = self
                .state
                .current_options
                .iter()
                .map(|o| o.line.id.clone())
                .collect();
            Err(DialogueError::InvalidLineIdError {
                selected_line_id,
                line_ids,
            })
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        self.execution_state != ExecutionState::Stopped
    }

    pub(crate) fn is_waiting_for_option_selection(&self) -> bool {
        self.execution_state == ExecutionState::WaitingOnOptionSelection
    }

    pub(crate) fn current_node(&self) -> Option<String> {
        self.current_node_name.clone()
    }

    /// ## Implementation note
    ///
    /// Increments the program counter here instead of in `continue_` for cleaner code
    pub(crate) fn run_instruction(
        &mut self,
        instruction: &Instruction,
        mut function_call_fn: impl FnMut(&dyn UntypedYarnFn, Vec<YarnValue>) -> YarnValue,
    ) -> crate::Result<()> {
        let opcode: OpCode = instruction.opcode.try_into().unwrap();
        match opcode {
            OpCode::JumpTo => {
                // Jumps to a named label
                let label_name: String = instruction.read_operand(0);
                self.state.program_counter = self.find_instruction_point_for_label(&label_name);
            }
            OpCode::Jump => {
                // Jumps to a label whose name is on the stack.
                let jump_destination: String = self.state.peek();
                self.state.program_counter =
                    self.find_instruction_point_for_label(&jump_destination);
            }
            OpCode::RunLine => {
                // Looks up a string from the string table and passes it to the client as a line

                let string_id: String = instruction.read_operand(0);
                let string_id: LineId = string_id.into();

                // The second operand, if provided (compilers prior
                // to v1.1 don't include it), indicates the number
                // of expressions in the line. We need to pop these
                // values off the stack and deliver them to the
                // line handler.
                assert_up_to_date_compiler(instruction.operands.len() >= 2);

                let substitutions = self.pop_substitutions_with_count_at_operand(instruction, 1);
                let line = self.prepare_line(string_id, &substitutions)?;

                self.batched_events.push(DialogueEvent::Line(line));

                // Implementation note:
                // In the original, this is only done if `execution_state` is still `DeliveringContent`,
                // because the line handler is allowed to call `continue_`. However, we disallow that because of
                // how this violates borrow checking. So, we'll always wait at this point instead until the user
                // called `continue_` themselves outside of the line handler.
                self.set_execution_state(ExecutionState::WaitingForContinue);
                self.state.program_counter += 1;
            }
            OpCode::RunCommand => {
                // Passes a string to the client as a custom command
                let command_text: String = instruction.read_operand(0);
                assert_up_to_date_compiler(instruction.operands.len() >= 2);
                let command_text = self
                    .pop_substitutions_with_count_at_operand(instruction, 1)
                    .into_iter()
                    .enumerate()
                    .fold(command_text, |command_text, (i, substitution)| {
                        command_text.replace(&format!("{{{i}}}"), &substitution)
                    });
                let command = Command::parse(command_text);

                self.batched_events.push(DialogueEvent::Command(command));

                // Implementation note:
                // In the original, this is only done if `execution_state` is still `DeliveringContent`,
                // because the line handler is allowed to call `continue_`. However, we disallow that because of
                // how this violates borrow checking. So, we'll always wait at this point instead until the user
                // called `continue_` themselves outside of the line handler.
                self.set_execution_state(ExecutionState::WaitingForContinue);
                self.state.program_counter += 1;
            }
            OpCode::AddOption => {
                // Add an option to the current state
                let string_id: String = instruction.read_operand(0);
                let string_id: LineId = string_id.into();
                assert_up_to_date_compiler(instruction.operands.len() >= 4);
                let substitutions = self.pop_substitutions_with_count_at_operand(instruction, 2);
                let line = self.prepare_line(string_id, &substitutions)?;

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
                    self.state.pop()
                } else {
                    true
                };

                let index = self.state.current_options.len();
                let node_name = instruction.read_operand(1);
                // ## Implementation note:
                // The original calculates the ID in the `ShowOptions` opcode,
                // but this way is cleaner because it allows us to store a `DialogueOption` instead of a bunch of values in a big tuple.
                self.state.current_options.push(DialogueOption {
                    line,
                    id: OptionId(index),
                    destination_node: node_name,
                    is_available: line_condition_passed,
                });
                self.state.program_counter += 1;
            }
            OpCode::ShowOptions => {
                // If we have no options to show, immediately stop.
                if self.state.current_options.is_empty() {
                    self.batched_events.push(DialogueEvent::DialogueComplete);
                    self.set_execution_state(ExecutionState::Stopped);
                    self.state.program_counter += 1;
                    return Ok(());
                }

                // We can't continue until our client tell us which option to pick
                self.set_execution_state(ExecutionState::WaitingOnOptionSelection);

                // Pass the options set to the client, as well as a
                // delegate for them to call when the user has made
                // a selection
                let current_options = self.state.current_options.clone();
                self.batched_events
                    .push(DialogueEvent::Options(current_options));

                // Implementation note:
                // Not checking the execution state now since we have no line handler to call `continue_` from.
                self.state.program_counter += 1;
            }
            OpCode::PushString => {
                // Pushes a string value onto the stack. The operand is an index into the string table, so that's looked up first.
                let string_table_index: String = instruction.read_operand(0);
                self.state.push(string_table_index);
                self.state.program_counter += 1;
            }
            OpCode::PushFloat => {
                // Pushes a floating point onto the stack.
                let float: f32 = instruction.read_operand(0);
                self.state.push(float);
                self.state.program_counter += 1;
            }
            OpCode::PushBool => {
                // Pushes a boolean value onto the stack.
                let boolean: bool = instruction.read_operand(0);
                self.state.push(boolean);
                self.state.program_counter += 1;
            }

            OpCode::PushNull => {
                panic!(
                    "PushNull is no longer valid op code, because null is no longer a valid value from Yarn Spinner 2.0 onwards. To fix this error, re-compile the original source code."
                );
            }
            OpCode::JumpIfFalse => {
                // Jumps to a named label if the value on the top of the stack evaluates to the boolean value 'false'.
                let is_top_value_true: bool = self.state.peek();
                if !is_top_value_true {
                    let label_name: String = instruction.read_operand(0);
                    let instruction_point = self.find_instruction_point_for_label(&label_name);
                    self.state.program_counter = instruction_point;
                } else {
                    self.state.program_counter += 1;
                }
            }
            OpCode::Pop => {
                // Pops a value from the stack.
                self.state.pop_value();
                self.state.program_counter += 1;
            }
            OpCode::CallFunc => {
                let actual_parameter_count: usize = self.state.pop();
                // Get the parameters, which were pushed in reverse
                let parameters = {
                    let mut parameters: Vec<_> = (0..actual_parameter_count)
                        .rev()
                        .map(|_| self.state.pop_value().raw_value)
                        .collect();
                    parameters.reverse();
                    parameters
                };

                // Call a function, whose parameters are expected to be on the stack. Pushes the function's return value, if it returns one.
                let function_name: String = instruction.read_operand(0);
                let function =
                    self.library
                        .get(&function_name)
                        .ok_or(DialogueError::FunctionNotFound {
                            function_name: function_name.to_string(),
                            library: self.library.clone(),
                        })?;

                // Expect the compiler to have placed the number of parameters
                // actually passed at the top of the stack.
                let expected_parameter_count = function.parameter_types().len();

                assert_eq!(
                    expected_parameter_count, actual_parameter_count,
                    "Function {function_name} expected {expected_parameter_count} parameters, but received {actual_parameter_count}",
                );

                // Invoke the function
                let return_value = function_call_fn(function, parameters);
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
                self.state.push(typed_return_value);
                self.state.program_counter += 1;
            }
            OpCode::PushVariable => {
                // Get the contents of a variable, push that onto the stack.
                let variable_name: String = instruction.read_operand(0);
                let loaded_value = self
                    .variable_storage
                    .get(&variable_name)
                    .or_else(|e| {
                        if let VariableStorageError::VariableNotFound { .. } = e {
                            // We don't have a value for this. The initial
                            // value may be found in the program. (If it's
                            // not, then the variable's value is undefined,
                            // which isn't allowed.)
                            let initial_value = self
                                .program
                                .as_ref()
                                .unwrap()
                                .initial_values
                                .get(&variable_name)
                                .unwrap_or_else(|| panic!("The loaded program does not contain an initial value for the variable {variable_name}"))
                                .clone();

                            // Store the initial value in the variable_storage
                            self.variable_storage.set(variable_name.clone(), initial_value.clone().into())?;

                            Ok(initial_value.into())
                        } else {
                            Err(e)
                        }
                    })?;
                self.state.push(loaded_value);
                self.state.program_counter += 1;
            }
            OpCode::StoreVariable => {
                // Store the top value on the stack in a variable.
                let top_value = self.state.peek_value().clone();
                let variable_name: String = instruction.read_operand(0);
                self.variable_storage.set(variable_name, top_value.into())?;
                self.state.program_counter += 1;
            }
            OpCode::Stop => {
                // Immediately stop execution, and report that fact.
                let current_node_name = self.current_node_name.clone().unwrap();
                self.batched_events
                    .push(DialogueEvent::NodeComplete(current_node_name));
                self.batched_events.push(DialogueEvent::DialogueComplete);
                self.set_execution_state(ExecutionState::Stopped);

                self.state.program_counter += 1;
            }
            OpCode::RunNode => {
                // Run a node

                // Pop a string from the stack, and jump to a node
                // with that name.
                let node_name: String = self.state.pop();
                self.batched_events
                    .push(DialogueEvent::NodeComplete(node_name.clone()));
                self.set_node(&node_name)?;

                // No need to increment the program counter, since otherwise we'd skip the first instruction
            }
        }
        Ok(())
    }

    fn prepare_line(&mut self, string_id: LineId, substitutions: &[String]) -> Result<Line> {
        let line_text = self.text_provider.get_text(&string_id).ok_or_else(|| {
            DialogueError::LineProviderError {
                id: string_id.clone(),
                language_code: self.language_code.clone(),
            }
        })?;
        let substituted_text = expand_substitutions(&line_text, substitutions);
        let markup = self
            .parse_markup(&substituted_text)
            .map_err(DialogueError::MarkupParseError)?;
        let line = Line {
            id: string_id,
            text: markup.text,
            attributes: markup.attributes,
        };
        Ok(line)
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
        self.current_node
            .as_ref()
            .unwrap()
            .labels
            .get(label_name)
            .copied()
            .unwrap_or_else(|| {
                panic!(
                    "Unknown label {label_name} in node {}",
                    self.current_node_name.as_ref().unwrap()
                )
            })
            .try_into()
            .unwrap()
    }

    fn pop_substitutions_with_count_at_operand(
        &mut self,
        instruction: &Instruction,
        index: usize,
    ) -> Vec<String> {
        let expression_count: usize = instruction.operands[index].clone().try_into().unwrap();
        let mut values: Vec<_> = (0..expression_count)
            .rev()
            .map(|_| self.state.pop())
            .collect();
        values.reverse();
        values
    }
}

fn assert_up_to_date_compiler(predicate: bool) {
    assert!(
        predicate,
        "The Yarn script provided was compiled using an older compiler. \
        Please recompile it using the latest version of either Yarn Spinner or Yarn Spinner."
    )
}

/// Replaces all substitution markers in a text with the given substitution list.
///
/// This method replaces substitution markers
/// (for example, `{0}`) with the corresponding entry in `substitutions`.
/// If `test` contains a substitution marker whose
/// index is not present in `substitutions`, it is
/// ignored.
#[must_use]
fn expand_substitutions(text: &str, substitutions: &[String]) -> String {
    substitutions
        .iter()
        .enumerate()
        .fold(text.to_owned(), |text, (i, substitution)| {
            text.replace(&format!("{{{i}}}",), substitution)
        })
}
