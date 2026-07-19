//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/VirtualMachine.cs>
//!
//! ## Implementation Notes
//! The `Operand` extensions and the `Operator` enum were moved into upstream crates to make them not depend on the runtime.

pub(crate) use self::{execution_state::*, state::*};
use crate::Result;
use crate::markup::{LineParser, ParsedMarkup};
use crate::prelude::*;
use core::fmt::Debug;
use std::collections::VecDeque;
use std::error::Error;
use log::*;

mod execution_state;
mod state;

#[derive(Debug, Clone)]
pub(crate) struct VirtualMachine {
    pub(crate) library: Library,
    pub(crate) program: Option<Program>,
    pub(crate) variable_storage: Box<dyn VariableStorage>,
    pub(crate) line_hints_enabled: bool,
    states: VecDeque<State>,
    execution_state: ExecutionState,
    batched_events: Vec<DialogueEvent>,
    line_parser: LineParser,
    text_provider: Box<dyn TextProvider>,
    language_code: Option<Language>,
}

impl VirtualMachine {

    fn current_state(&self) -> Option<&State> { self.states.back() }

    fn current_state_mut(&mut self) -> Option<&mut State> { self.states.back_mut() }

    fn current_node(&self) -> Option<&Node> {
        let node_name = self.current_state()?.node_name.clone();
        self.get_node_from_name(&node_name).ok()
    }

    fn pop<T>(&mut self) -> T
    where
        T: TryFrom<InternalValue>,
        <T as TryFrom<InternalValue>>::Error: Debug {
        self.current_state_mut().unwrap_or_bug().pop()
    }

    fn pop_value(&mut self) -> InternalValue {
        self.current_state_mut().unwrap_or_bug().pop_value()
    }

    fn push(&mut self, value: impl Into<InternalValue>) {
        self.current_state_mut().unwrap_or_bug().push(value);
    }

    fn longjmp<T: TryInto<usize>>(&mut self, destination: T) where T::Error: Error {
        let state= self.current_state_mut().unwrap_or_bug();
        let destination = destination.try_into().unwrap();
        assert_ne!(state.program_counter, destination);
        log::debug!("longjmp {:#08x}", destination);
        state.program_counter = destination;
    }

    fn step(&mut self) {
        self.current_state_mut().unwrap_or_bug().program_counter += 1;
    }

    pub(crate) fn current_node_name(&self) -> Option<String> {
        match self.current_node() {
            Some(n) => Some(n.name.clone()),
            None => None
        }
    }

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
            states: Default::default(),
            execution_state: Default::default(),
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
        self.states.clear();
    }

    fn push_state(&mut self, node_name: &str) -> &mut State {
        let mut state = State::default();
        state.node_name = node_name.to_owned();
        self.states.push_back_mut(state)
    }

    fn pop_state(&mut self) -> Result<State> {
        if let Some(state) = self.states.pop_back() {
            Ok(state)
        } else {
            Err(DialogueError::ReturnStackEmpty)
        }
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

    pub(crate) fn jump_to_node(&mut self, node_name: impl Into<String>) -> Result<()> {
        let node_name = node_name.into();

        if !self.states.is_empty() {
            self.leave_node()?;
        }
        self.reset_state();
        self.enter_node(node_name)
    }

    pub(crate) fn enter_node(&mut self, node_name: impl Into<String>) -> Result<()> {
        let node_name = node_name.into();

        debug!("Loading node \"{node_name}\"");
        self.get_node_from_name(&node_name)?; // must exist

        self.push_state(&node_name);

        self.batched_events
            .push(DialogueEvent::NodeStart(node_name));

        if self.line_hints_enabled {
            self.send_line_hints();
        }

        Ok(())
    }

    pub(crate) fn leave_node(&mut self) -> Result<()> {
        let state = self.pop_state()?;

        let node = self.get_node_from_name(&state.node_name)?;

        self.batched_events
            .push(DialogueEvent::NodeComplete(node.name.clone()));

        Ok(())
    }

    fn send_line_hints(&mut self) {
        // Create a list; we will never have more lines and options
        // than total instructions, so that's a decent capacity for
        // the list
        // [sic] TODO: maybe this list could be reused to save on allocations?

        let string_ids: Vec<_> = self
            .current_node()
            .as_ref()
            .unwrap()
            .instructions
            .iter()
            // Loop over every instruction and find the ones that run a
            // line or add an option; these are the two instructions
            // that will signal a line can appear to the player
            .filter_map(|instruction| {
                let opcode = instruction.instruction_type.as_ref().unwrap_or_bug();

                match opcode {
                    InstructionType::RunLine(line) => Some(line.line_id.clone().into()),
                    InstructionType::AddOption(opt) => Some(opt.line_id.clone().into()),
                    _ => None
                }
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

    fn current_instruction(&self) -> Option<Instruction> {
        let state = self.current_state()?;
        let current_node = self.current_node()?;
        Some(current_node.instructions[state.program_counter].clone())
    }

    /// Resumes execution.
    pub(crate) fn continue_(
        &mut self,
        mut instruction_fn: impl FnMut(&mut Self, &Instruction) -> crate::Result<()>,
    ) -> crate::Result<Vec<DialogueEvent>> {
        self.assert_can_continue()?;
        self.set_execution_state(ExecutionState::Running);

        while self.execution_state == ExecutionState::Running {
            let current_instruction = self.current_instruction().unwrap_or_bug();
            instruction_fn(self, &current_instruction)?;
            // ## Implementation note
            // The original increments the program counter here, but that leads to intentional underflow on [`OpCode::RunNode`],
            // so we do the incrementation in [`VirtualMachine::run_instruction`] instead.

            let Some(state) = self.current_state() else {
                break
            };

            let node = self.get_node_from_name(&state.node_name)?;

            if state.program_counter >= node.instructions.len() {
                self.leave_node()?;
                self.set_execution_state(ExecutionState::Stopped);
                self.batched_events.push(DialogueEvent::DialogueComplete);
                debug!("Run complete.");
            }
        }
        Ok(core::mem::take(&mut self.batched_events))
    }

    pub(crate) fn parse_markup(&mut self, line: &str) -> crate::markup::Result<ParsedMarkup> {
        self.line_parser.parse_markup(line)
    }

    /// Runs a series of tests to see if the [`VirtualMachine`] is in a state where [`VirtualMachine::r#continue`] can be called. Panics if it can't.
    pub(crate) fn assert_can_continue(&self) -> crate::Result<()> {
        if self.current_node().is_none() || self.current_node_name().is_none() {
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

        let state = self.current_state_mut().unwrap_or_bug();

        if selected_option_id.0 >= state.current_options.len() {
            return Err(DialogueError::InvalidOptionIdError {
                selected_option_id,
                max_id: state.current_options.len().saturating_sub(1),
            });
        }

        // We now know what number option was selected; push the
        // corresponding node name to the stack.
        let destination = state.current_options[selected_option_id.0].destination.unwrap_or_bug();
        state.push(destination);

        // We no longer need the accumulated list of options; clear it
        // so that it's ready for the next one
        state.current_options.clear();

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

        let state = self.current_state_mut().unwrap_or_bug();

        if let Some(selected_option) = state
            .current_options
            .iter()
            .find(|o| o.line.id == selected_line_id)
        {
            let selected_option_id = selected_option.id;
            self.set_selected_option(selected_option_id)
                .map(|_| selected_option_id)
        } else {
            let line_ids = state
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

    /// ## Implementation note
    ///
    /// Increments the program counter here instead of in `continue_` for cleaner code
    pub(crate) fn run_instruction(
        &mut self,
        instruction: &Instruction,
        mut function_call_fn: impl FnMut(&dyn UntypedYarnFn, Vec<YarnValue>) -> YarnValue,
    ) -> crate::Result<()> {
        log::debug!("running(0x{:#08x}): {:?}", self.current_state().unwrap().program_counter, instruction);
        match instruction.instruction_type.as_ref().unwrap_or_bug() {
            InstructionType::JumpTo(jumpto) => {
                // Jumps to destination
                self.longjmp(jumpto.destination);
            }
            InstructionType::PeekAndJump(_) => {
                let state = self.current_state().unwrap_or_bug();
                // Jumps to a label whose name is on the stack.
                let jump_destination: InternalValue = state.peek();

                match jump_destination.r#type {
                    Type::Number => {
                        self.longjmp(jump_destination);
                    }
                    Type::String => {
                        panic!("invalid operand on stack: labels no longer supported")
                    },
                    _ => { panic!("invalid operand on stack") }
                }
            }
            InstructionType::RunLine(run_line) => {
                // Looks up a string from the string table and passes it to the client as a line

                let string_id: LineId = run_line.line_id.clone().into();

                let substitutions = self.pop_substitutions(run_line.substitution_count as usize);
                let line = self.prepare_line(string_id, &substitutions)?;

                self.batched_events.push(DialogueEvent::Line(line));

                // Implementation note:
                // In the original, this is only done if `execution_state` is still `DeliveringContent`,
                // because the line handler is allowed to call `continue_`. However, we disallow that because of
                // how this violates borrow checking. So, we'll always wait at this point instead until the user
                // called `continue_` themselves outside of the line handler.
                self.set_execution_state(ExecutionState::WaitingForContinue);

                self.step();
            }
            InstructionType::RunCommand(cmd) => {
                // Passes a string to the client as a custom command
                let command_text = self
                    .pop_substitutions(cmd.substitution_count as usize)
                    .into_iter()
                    .enumerate()
                    .fold(cmd.command_text.clone(), |command_text, (i, substitution)| {
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

                self.step();
            }
            InstructionType::AddOption(opt) => {
                // Add an option to the current state
                let string_id: LineId = opt.line_id.clone().into();
                let substitutions = self.pop_substitutions(opt.substitution_count as usize);
                let line = self.prepare_line(string_id, &substitutions)?;

                let state = self.current_state_mut().unwrap_or_bug();

                // Indicates whether the VM believes that the
                // option should be shown to the user, based on any
                // conditions that were attached to the option.
                let line_condition_passed = if opt.has_condition {
                    // The fourth operand is a bool that indicates
                    // whether this option had a condition or not.
                    // If it does, then a bool value will exist on
                    // the stack indicating whether the condition
                    // passed or not. We pass that information to
                    // the game.
                    state.pop()
                } else {
                    true
                };

                let index = state.current_options.len();

                // ## Implementation note:
                // The original calculates the ID in the `ShowOptions` opcode,
                // but this way is cleaner because it allows us to store a `DialogueOption` instead of a bunch of values in a big tuple.
                state.current_options.push(DialogueOption {
                    line,
                    id: OptionId(index),
                    destination: Some(opt.destination as usize),
                    is_available: line_condition_passed,
                });
                self.step();
            }
            InstructionType::ShowOptions(_) => {
                let state = self.current_state().unwrap_or_bug();

                // If we have no options to show, immediately stop.
                if state.current_options.is_empty() {
                    self.batched_events.push(DialogueEvent::DialogueComplete);
                    self.set_execution_state(ExecutionState::Stopped);
                    self.step();
                    return Ok(());
                }

                // We can't continue until our client tell us which option to pick
                self.set_execution_state(ExecutionState::WaitingOnOptionSelection);

                // Pass the options set to the client, as well as a
                // delegate for them to call when the user has made
                // a selection
                let state = self.current_state().unwrap_or_bug();
                let current_options = state.current_options.clone();
                self.batched_events
                    .push(DialogueEvent::Options(current_options));

                // Implementation note:
                // Not checking the execution state now since we have no line handler to call `continue_` from.
                self.step();
            }
            InstructionType::PushString(push) => {
                self.push(push.value.clone());
                self.step();
            }
            InstructionType::PushFloat(push) => {
                self.push(push.value);
                self.step();
            }
            InstructionType::PushBool(push) => {
                self.push(push.value);
                self.step();
            }
            InstructionType::JumpIfFalse(jump) => {
                let state = self.current_state().unwrap_or_bug();

                // Jumps to a named label if the value on the top of the stack evaluates to the boolean value 'false'.
                let is_top_value_true: bool = state.peek();
                if !is_top_value_true {
                    let goto = jump.destination;

                    self.longjmp(goto);
                } else {
                    self.step();
                }
            }
            InstructionType::Pop(_) => {
                // Pops a value from the stack.
                self.pop_value();
                self.step();
            }
            InstructionType::CallFunc(func) => {
                let state = self.current_state_mut().unwrap_or_bug();

                let actual_parameter_count: usize = state.pop();
                // Get the parameters, which were pushed in reverse
                let parameters = {
                    let mut parameters: Vec<_> = (0..actual_parameter_count)
                        .rev()
                        .map(|_| state.pop_value().raw_value)
                        .collect();
                    parameters.reverse();
                    parameters
                };

                // Call a function, whose parameters are expected to be on the stack. Pushes the function's return value, if it returns one.
                let function_name: String = func.function_name.clone();
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
                self.push(typed_return_value);
                self.step();
            }
            InstructionType::PushVariable(push) => {
                // Get the contents of a variable, push that onto the stack.
                let variable_name: String = push.variable_name.clone();
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

                self.push(loaded_value);
                self.step();
            }
            InstructionType::StoreVariable(store) => {
                let state = self.current_state().unwrap_or_bug();

                // Store the top value on the stack in a variable.
                let top_value = state.peek_value().clone();
                let variable_name: String = store.variable_name.clone();
                self.variable_storage.set(variable_name, top_value.into())?;

                self.step();
            }
            InstructionType::Stop(_) => {
                // Immediately stop execution, and report that fact.
                while !self.states.is_empty() {
                    self.leave_node()?;
                }

                self.batched_events.push(DialogueEvent::DialogueComplete);
                self.set_execution_state(ExecutionState::Stopped);
                debug!("Stopped");
            }
            InstructionType::Return(_) => {
                // Perform no action

                self.leave_node()?;
                self.step();
            }
            InstructionType::PeekAndRunNode(_) => {
                // Run a node

                let node_name: String = self.pop();

                // jump to a node with that name.
                self.batched_events
                    .push(DialogueEvent::NodeComplete(node_name.clone()));
                self.jump_to_node(&node_name)?;
            }
            InstructionType::RunNode(run) => {
                // Run a node

                let node_name: String = run.node_name.clone();

                // jump to a node with that name.
                self.batched_events
                    .push(DialogueEvent::NodeComplete(node_name.clone()));
                self.jump_to_node(&node_name)?;

                // No need to increment the program counter, since otherwise we'd skip the first instruction
            }
            InstructionType::PeekAndDetourToNode(_) => {
                // Detour to a node

                let node_name: String = self.pop();

                // jump to a node with that name.
                self.enter_node(node_name)?;

                // No need to increment the program counter, since otherwise we'd skip the first instruction
            }
            InstructionType::DetourToNode(detour) => {
                // Detour to a node

                let node_name: String = detour.node_name.clone();

                // jump to a node with that name.
                self.enter_node(&node_name)?;

                // No need to increment the program counter, since otherwise we'd skip the first instruction
            }
            InstructionType::AddSaliencyCandidate(_) => {
                todo!()
            }
            InstructionType::AddSaliencyCandidateFromNode(_) => {
                todo!()
            }
            InstructionType::SelectSaliencyCandidate(_) => {
                todo!()
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

    fn pop_substitutions(
        &mut self,
        count: usize,
    ) -> Vec<String> {
        let state = self.current_state_mut().unwrap_or_bug();
        let mut values: Vec<_> = (0..count)
            .rev()
            .map(|_| state.pop())
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
    if substitutions.is_empty() {
        // if we have no substitutions we want to just return the text as is
        return text.to_owned();
    }

    substitutions
        .iter()
        .enumerate()
        .fold(text.to_owned(), |text, (i, substitution)| {
            text.replace(&format!("{{{i}}}",), substitution)
        })
}
