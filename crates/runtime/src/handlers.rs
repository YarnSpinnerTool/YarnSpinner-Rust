//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
//!
//! ## Implementation notes
//!
//! - `OptionSet` was replaced by a simple `Vec<DialogueOption>`
//! - Additional newtypes were introduced for strings.

use crate::prelude::*;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

/// A command, sent from the [`Dialogue`] to the game.
///
/// You do not create instances of this struct yourself. They are created by the [`Dialogue`] during program execution.
///
/// ## See also
/// [`CommandHandler`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command(pub String);
impl Deref for Command {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Command {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeName(pub String);
impl Deref for NodeName {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NodeName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Represents a method that receives diagnostic messages and error information from a [`Dialogue`].
///
/// The text that this delegate receives may be output to a console, or sent to a log.
///
/// ## Params
/// - The text that should be logged.

#[derive(Debug, Clone)]
pub struct Logger(pub Box<dyn LoggerTrait>);

impl Clone for Box<dyn LoggerTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Debug for dyn LoggerTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logger")
    }
}

pub trait LoggerTrait {
    fn call(&self, message: String);
    fn clone_box(&self) -> Box<dyn LoggerTrait>;
}
impl<T> LoggerTrait for T
where
    T: Fn(String),
{
    fn call(&self, message: String) {
        self(message)
    }

    fn clone_box(&self) -> Box<dyn LoggerTrait> {
        Box::new(self.clone())
    }
}

/// Represents the method that is called when the [`Dialogue`] delivers a [`Line`].
///
/// ## See also
/// - [`OptionsHandler`]
/// - [`CommandHandler`]
/// - [`NodeStartHandler`]
/// - [`NodeCompleteHandler`]
/// - [`DialogueCompleteHandler`]
pub type LineHandler = dyn Fn(Line);

/// Represents the method that is called when the [`Dialogue`] delivers an [`OptionSet`].
///
/// ## See also
/// - [`LineHandler`]
/// - [`CommandHandler`]
/// - [`NodeStartHandler`]
/// - [`NodeCompleteHandler`]
/// - [`DialogueCompleteHandler`]
pub type OptionsHandler = dyn Fn(Vec<DialogueOption>);

/// Represents the method that is called when the [`Dialogue`] delivers a [`Command`].
///
/// ## See also
/// - [`LineHandler`]
/// - [`OptionsHandler`]
/// - [`NodeStartHandler`]
/// - [`NodeCompleteHandler`]
/// - [`DialogueCompleteHandler`]
pub type CommandHandler = dyn Fn(Command);

/// Represents the method that is called when the [`Dialogue`] reaches the end of a node.
///
/// This method may be called multiple times over the course of code execution. A node being complete does not necessarily represent the end of the conversation.
///
/// ## See also
/// - [`LineHandler`]
/// - [`OptionsHandler`]
/// - [`CommandHandler`]
/// - [`NodeStartHandler`]
/// - [`DialogueCompleteHandler`]
pub type NodeCompleteHandler = dyn Fn(NodeName);

/// Represents the method that is called when the [`Dialogue`] begins executing a node.
///
/// ## See also
/// - [`LineHandler`]
/// - [`OptionsHandler`]
/// - [`CommandHandler`]
/// - [`NodeCompleteHandler`]
/// - [`DialogueCompleteHandler`]
pub type NodeStartHandler = dyn Fn(NodeName);

/// Represents the method that is called when the dialogue has reached its end, and no more code remains to be run.
///
/// ## See also
/// - [`LineHandler`]
/// - [`OptionsHandler`]
/// - [`CommandHandler`]
/// - [`NodeStartHandler`]
/// - [`NodeCompleteHandler`]
pub type DialogueCompleteHandler = dyn Fn();

/// Represents the method that is called when the dialogue anticipates that it will deliver lines.
///
/// This method should begin preparing to run the lines. For example, if a game delivers dialogue via voice-over,
/// the appropriate audio files should be loaded.
///
/// This method serves to provide a hint to the game that a line _may_ be run.
/// Not every line indicated in the provided `LineId`s may end up actually running.
///
/// This method may be called any number of times during a dialogue session.
pub type PrepareForLinesHandler = dyn Fn(Vec<LineId>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_assign_handlers() {
        let logger = Logger(Box::new(|message| println!("{}", message)));
        let clone = logger.clone();
    }
}
