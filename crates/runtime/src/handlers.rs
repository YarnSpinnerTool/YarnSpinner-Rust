//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
//!
//! ## Implementation notes
//!
//! - `OptionSet` was replaced by a simple `Vec<DialogueOption>`
//! - Additional newtypes were introduced for strings.

use crate::prelude::*;
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
