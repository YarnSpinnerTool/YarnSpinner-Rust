//! This example implements a minimal Yarn dialogue runner, which outputs to the terminal using ratatui.
//! Not all features are covered here (e.g. localization), but it should give you an idea of how the
//! yarnspinner crate can be used without a pre-made engine integration.
//!
//! The implementation is annotated with various comments explaining what each API does - start at
//! TuiDialogueRunner::new, and follow along from there!

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use anyhow::{Context, anyhow};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Stylize};
use yarnspinner::compiler::Compiler;
use yarnspinner::core::{IntoYarnValueFromNonYarnValue, LineId, YarnFn, YarnValue};
use yarnspinner::runtime::{
    Dialogue, DialogueEvent, Line, MemoryVariableStorage, StringTableTextProvider,
};

use self::terminal::Terminal;
use self::widgets::{ContinueView, LineView, OptionsView, OptionsViewState};

pub mod terminal;
pub mod widgets;

enum Status {
    ReadyToContinue,
    WaitingForContinue,
    WaitingForOptions(OptionsViewState),
}

pub struct TuiDialogueRunner {
    dialogue: Dialogue,
    metadata: HashMap<LineId, Vec<String>>,
    status: Status,
    last_line: Option<Line>,
    background_color: Color,
}

impl TuiDialogueRunner {
    pub fn new(
        source_path: impl AsRef<Path>,
        start_node: &str,
    ) -> anyhow::Result<TuiDialogueRunner> {
        // Before we can run our dialogue, we need to compile it.
        //
        // In a real game, you might want to consider doing this as part of your asset
        // pipeline, rather than at runtime..
        let compilation = Compiler::new().read_file(source_path).compile()?;

        // One of the outputs of compiling is a string table, containing all of the
        // text (and associated metadata) for our dialogue.
        let mut base_language_string_table = HashMap::<LineId, String>::default();
        let mut metadata = HashMap::<LineId, Vec<String>>::default();

        for (k, v) in compilation.string_table {
            base_language_string_table.insert(k.clone(), v.text);
            metadata.insert(k, v.metadata);
        }

        // In order to create a Dialogue object, we need two things.
        //
        // First, an implementation of TextProvider, which is used to fetch the text
        // that gets displayed in your game.
        //
        // The yarnspinner crate provides a simple implementation of this called
        // StringTableTextProvider, which supports storing the base language
        // (whatever you wrote the original text in), and one active localization.
        // For this example, we'll just stick to the base language.
        let mut text_provider = StringTableTextProvider::new();
        text_provider.extend_base_language(base_language_string_table);

        // Second, an implementation of VariableStorage, which is where any variables
        // defined in your dialogue will get stored.
        //
        // The yarnspinner crate provides a simple implementation of this called
        // MemoryVariableStorage, which (as the name suggests!) stores the variables
        // in memory.
        let variable_storage = MemoryVariableStorage::new();

        // Now we can create the Dialogue! Note that both parameters need to be boxed.
        let mut dialogue = Dialogue::new(Box::new(variable_storage), Box::new(text_provider));

        // Finally, we need to actually give the Dialogue (which is empty to begin with) our
        // compiled program, and tell it which node to start running from.
        //
        // To see how we actually drive this at runtime, scroll down to `fn update`!
        dialogue.add_program(compilation.program.context("no program compiled")?);
        dialogue.set_node(start_node)?;

        Ok(TuiDialogueRunner {
            dialogue,
            metadata,
            status: Status::ReadyToContinue,
            last_line: None,
            background_color: Color::Black,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        terminal::set_panic_hook();
        let mut terminal = terminal::init()?;

        let result = self.run_inner(&mut terminal);

        // We should always *try* to clean up the terminal before we exit, even if an
        // error was thrown.
        let _ = terminal::restore();

        result
    }

    fn run_inner(&mut self, terminal: &mut Terminal) -> anyhow::Result<()> {
        loop {
            let should_exit = self.update()?;

            if should_exit {
                return Ok(());
            }

            self.draw(terminal)?;
        }
    }

    fn update(&mut self) -> anyhow::Result<bool> {
        // This function is where we actually make use of the Dialogue to decide what
        // gets shown to the player. We're using a TUI, but the approach would be
        // similar using a game engine too.
        //
        // First, let's check for any input from the player.
        if let Some(key) = terminal::poll_input()? {
            match key {
                // If they press Q, we'll exit the game loop.
                KeyCode::Char('q') => return Ok(true),

                // If they press Enter, we should check if the UI is currently waiting for
                // them to continue or select an option. If so, we can signal to the
                // dialogue that we're ready to continue running.
                KeyCode::Enter => match &mut self.status {
                    Status::WaitingForContinue => {
                        self.status = Status::ReadyToContinue;
                    }

                    Status::WaitingForOptions(state) => {
                        if let Some(id) = state.selected() {
                            // If the dialogue is waiting for an option to be selected, we
                            // must set the selected option before trying to continue
                            // running, or we'll get an error. We'll see where this
                            // ID comes from later!
                            self.dialogue.set_selected_option(id)?;

                            self.status = Status::ReadyToContinue;
                        }
                    }

                    _ => {}
                },

                // If they press Up or Down, and the UI is currently waiting for an option
                // to be selected, we should move the cursor.
                KeyCode::Up => {
                    if let Status::WaitingForOptions(state) = &mut self.status {
                        state.move_cursor_up();
                    }
                }

                KeyCode::Down => {
                    if let Status::WaitingForOptions(state) = &mut self.status {
                        state.move_cursor_down();
                    }
                }

                _ => {}
            }
        }

        // Now that we're done handling the player's input, we can check if we're ready to
        // continue running the dialogue.
        if let Status::ReadyToContinue = &self.status {
            // If so, call `Dialogue::continue_` to get a list of dialogue events to handle.
            let events = self.dialogue.continue_()?;

            for event in events {
                match event {
                    // A 'Line' event will be triggered whenever a new line of dialogue is ready
                    // to be presented to the player.
                    DialogueEvent::Line(line) => {
                        // If this is the last line before displaying a list of options,
                        // we don't need to wait for the player to hit 'continue'.
                        let last_line_before_options = self
                            .metadata
                            .get(&line.id)
                            .map(|m| m.iter().any(|x| x == "lastline"))
                            .unwrap_or(false);

                        if !last_line_before_options {
                            self.status = Status::WaitingForContinue;
                        }

                        // Now we can store the new line, for our 'draw' method to render.
                        self.last_line = Some(line);
                    }

                    // An 'Options' event will be triggered whenever a list of options needs
                    // to be presented to the player.
                    //
                    // As mentioned previously, we cannot run `continue_` again until an option
                    // has been selected.
                    DialogueEvent::Options(options) => {
                        self.status = Status::WaitingForOptions(OptionsViewState::new(options));
                    }

                    // A 'Command' event will be triggered whenever a custom command is encountered
                    // in the dialogue.
                    //
                    // The command is provided to you in the form of a name, and a list of `YarnValue`
                    // parameters. It's up to you to handle parsing/validating these in whatever
                    // way is appropriate for your game.
                    DialogueEvent::Command(command) => match command.name.as_str() {
                        "set_background" => match command.parameters.first() {
                            Some(YarnValue::String(s)) => {
                                self.background_color = Color::from_str(s)?;
                            }

                            _ => {
                                return Err(anyhow!(
                                    "invalid parameters: {:?}",
                                    command.parameters
                                ));
                            }
                        },

                        _ => {
                            return Err(anyhow!("unknown command: {}", command.name));
                        }
                    },

                    // A 'DialogueComplete' event will be triggered whenever we reach the end of a node
                    // without jumping somewhere else. To continue running, you must call `set_node` on
                    // the dialogue (as we did in the constructor).
                    DialogueEvent::DialogueComplete => {
                        return Ok(true);
                    }

                    // There are several other events that can be triggered (such as NodeStart, or
                    // LineHints), which may be useful depending on what you're trying to build.
                    // See the docs for more info on these!
                    _ => {}
                }
            }
        }

        // And with that, we're done until the next update!
        //
        // From here, you may want to look at `get_variable` and 'set_variable' to learn how
        // to access your dialogue's state, or 'add_function' to learn how to add custom
        // functions to Yarn.
        Ok(false)
    }

    fn draw(&mut self, terminal: &mut Terminal) -> anyhow::Result<()> {
        terminal.draw(|f| {
            let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(5)]);
            let [output_area, options_area] = layout.areas(f.size());

            if let Some(line) = &self.last_line {
                f.render_widget(LineView::new(line).bg(self.background_color), output_area);
            }

            match &mut self.status {
                Status::WaitingForContinue => f.render_widget(
                    ContinueView::default().bg(self.background_color),
                    options_area,
                ),

                Status::WaitingForOptions(state) => f.render_stateful_widget(
                    OptionsView::default().bg(self.background_color),
                    options_area,
                    state,
                ),

                _ => {}
            }
        })?;

        Ok(())
    }

    pub fn get_variable(&self, name: &str) -> anyhow::Result<YarnValue> {
        let value = self.dialogue.variable_storage().get(name)?;

        Ok(value)
    }

    pub fn set_variable(
        &mut self,
        name: impl Into<String>,
        value: YarnValue,
    ) -> anyhow::Result<()> {
        self.dialogue
            .variable_storage_mut()
            .set(name.into(), value)?;

        Ok(())
    }

    pub fn add_function<Marker, F>(&mut self, name: impl Into<Cow<'static, str>>, function: F)
    where
        Marker: 'static,
        F: YarnFn<Marker> + 'static + Clone,
        F::Out: IntoYarnValueFromNonYarnValue + 'static + Clone,
    {
        self.dialogue.library_mut().add_function(name, function);
    }
}
