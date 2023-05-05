//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
//!
//! ## Implementation notes
//! Introduced `LineId` newtype for better type safety

use crate::string_newtype;

/// A line of dialogue, sent from the [`Dialogue`] to the game.
///
/// When the game receives a [`Line`], it should do the following things to prepare the line for presentation to the user.
/// - Use the value in the [`Line::ID`] field to look up the appropriate user-facing text in the string table.
/// - Use [`Dialogue::expand_substitutions`] to replace all substitutions in the user-facing text.
/// - Use [`Dialogue::parse_markup`] to parse all markup in the line.
///
/// You do not create instances of this struct yourself. They are created by the [`Dialogue`] during program execution.
///
/// ## See also
/// [`Dialogue::line_handler`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    /// The ID of the line in the string table.
    pub id: LineId,
    /// The values that should be inserted into the user-facing text before delivery.
    pub substitutions: Vec<String>,
}

impl Line {
    pub fn new(id: LineId) -> Self {
        Self {
            id,
            substitutions: Default::default(),
        }
    }
}

string_newtype! {
    pub struct LineId(pub String);
}
