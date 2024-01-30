//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
//!
//! ## Implementation notes
//!
//! - `OptionSet` was replaced by a simple `Vec<DialogueOption>`
//! - Additional newtypes were introduced for strings.

use crate::prelude::*;
use yarnspinner_core::prelude::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
/// An event encountered while running [`Dialogue::continue_`]. A caller is expected to handle these events and act accordingly.
///
/// ## Implementation note
///
/// Corresponds to Yarn Spinner's `<EventName>Handler`s.
pub enum DialogueEvent {
    /// A [`Line`] should be presented to the user.
    Line(Line),
    /// A list of [`DialogueOption`]s should be presented to the user, who in turns must select one of them.
    /// The selected option must be communicated to the [`Dialogue`] via [`Dialogue::set_selected_option`] before calling [`Dialogue::continue_`] again.
    Options(Vec<DialogueOption>),
    /// A [`Command`] should be executed.
    ///
    /// It is not specified whether the command should be finished executing before calling [`Dialogue::continue_`] again or it is run in parallel.
    /// A library wrapping Yarn Spinner for a game engine should specify this.
    Command(Command),
    /// The node with the given name was completed.
    NodeComplete(String),
    /// The node with the given name was entered.
    NodeStart(String),
    /// Only emitted if `Dialogue::should_send_line_hints` is enabled.
    ///
    /// A hint that the contained line IDs might be encountered while progressing the dialogue.
    /// These are not guaranteed to run, but give a caller the chance to pre-load resources for them if they want.
    ///
    /// ## Implementation note
    ///
    /// Corresponds to Yarn Spinner's `PrepareForLinesHandler`
    LineHints(Vec<LineId>),
    /// The dialogue was completed. Set it to a new node via [`Dialogue::set_node`] before calling [`Dialogue::continue_`] again.
    DialogueComplete,
}
