//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
//!
//! ## Implementation notes
//!
//! - `OptionSet` was replaced by a simple `Vec<DialogueOption>`
//! - Additional newtypes were introduced for strings.

use crate::prelude::*;
#[macro_use]
mod impl_handler_macro;
use crate::string_newtype;

string_newtype! {
    /// A command, sent from the [`Dialogue`] to the game.
    ///
    /// You do not create instances of this struct yourself. They are created by the [`Dialogue`] during program execution.
    ///
    /// ## See also
    /// [`CommandHandler`]
    pub struct Command(pub String);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DialogueEvent {
    Line(Line),
    Options(Vec<DialogueOption>),
    Command(Command),
    NodeComplete(String),
    NodeStart(String),
    PrepareForLines(Vec<LineId>),
    DialogueComplete,
}

impl_handler! {
    /// Represents a method that receives diagnostic messages and error information from a [`Dialogue`].
    ///
    /// The text that this delegate receives may be output to a console, or sent to a log.
    ///
    /// ## Params
    /// - The text that should be logged.
    pub struct Logger(pub LoggerFn: Fn(String) + Clone);

    /// Represents the method that is called when the [`Dialogue`] delivers a [`Line`].
    ///
    /// ## See also
    /// - [`OptionsHandler`]
    /// - [`CommandHandler`]
    /// - [`NodeStartHandler`]
    /// - [`NodeCompleteHandler`]
    /// - [`DialogueCompleteHandler`]
    pub struct LineHandler(pub LineHandlerFn: FnMut(Line));

    /// Represents the method that is called when the [`Dialogue`] delivers an [`OptionSet`].
    ///
    /// ## See also
    /// - [`LineHandler`]
    /// - [`CommandHandler`]
    /// - [`NodeStartHandler`]
    /// - [`NodeCompleteHandler`]
    /// - [`DialogueCompleteHandler`]
    pub struct OptionsHandler(pub OptionsHandlerFn: FnMut(Vec<DialogueOption>));

    /// Represents the method that is called when the [`Dialogue`] delivers a [`Command`].
    ///
    /// ## See also
    /// - [`LineHandler`]
    /// - [`OptionsHandler`]
    /// - [`NodeStartHandler`]
    /// - [`NodeCompleteHandler`]
    /// - [`DialogueCompleteHandler`]
    pub struct CommandHandler(pub CommandHandlerFn: FnMut(Command));

    /// Represents the method that is called when the [`Dialogue`] reaches the end of a node.
    /// The param is the node name.
    ///
    /// This method may be called multiple times over the course of code execution. A node being complete does not necessarily represent the end of the conversation.
    ///
    /// ## See also
    /// - [`LineHandler`]
    /// - [`OptionsHandler`]
    /// - [`CommandHandler`]
    /// - [`NodeStartHandler`]
    /// - [`DialogueCompleteHandler`]
    pub struct NodeCompleteHandler(pub NodeCompleteHandlerFn: FnMut(String));

    /// Represents the method that is called when the [`Dialogue`] begins executing a node.
    /// The param is the node name.
    ///
    /// ## See also
    /// - [`LineHandler`]
    /// - [`OptionsHandler`]
    /// - [`CommandHandler`]
    /// - [`NodeCompleteHandler`]
    /// - [`DialogueCompleteHandler`]
    pub struct NodeStartHandler(pub NodeStartHandlerFn: FnMut(String));

    /// Represents the method that is called when the dialogue has reached its end, and no more code remains to be run.
    ///
    /// ## See also
    /// - [`LineHandler`]
    /// - [`OptionsHandler`]
    /// - [`CommandHandler`]
    /// - [`NodeStartHandler`]
    /// - [`NodeCompleteHandler`]
    pub struct DialogueCompleteHandler(pub DialogueCompleteHandlerFn: FnMut());

    /// Represents the method that is called when the dialogue anticipates that it will deliver lines.
    ///
    /// This method should begin preparing to run the lines. For example, if a game delivers dialogue via voice-over,
    /// the appropriate audio files should be loaded.
    ///
    /// This method serves to provide a hint to the game that a line _may_ be run.
    /// Not every line indicated in the provided `LineId`s may end up actually running.
    ///
    /// This method may be called any number of times during a dialogue session.
    pub struct PrepareForLinesHandler(pub PrepareForLinesHandlerFn: FnMut(Vec<LineId>));
}
