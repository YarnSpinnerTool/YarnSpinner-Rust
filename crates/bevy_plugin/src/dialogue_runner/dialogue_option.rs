use crate::line_provider::LineAssets;
use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn dialogue_option_plugin(_app: &mut App) {}

/// An option that can be presented to the user during a dialogue.
/// Given to you by a [`PresentOptions`](crate::events::PresentOptions) event.
#[derive(Debug, Clone, PartialEq)]
pub struct DialogueOption {
    /// The [`LocalizedLine`] that should be presented to the user for this option.
    pub line: LocalizedLine,

    /// The identifying number for this option.
    ///
    /// When the user selects this option, this value should be used as the parameter for [`DialogueRunner::select_option`].
    pub id: OptionId,

    /// The name of the node that will be run if this option is selected.
    ///
    /// The value of this property not be valid if this is a shortcut option.
    pub destination_node: String,

    /// Gets a value indicating whether the player should be permitted to select this option.
    ///
    /// If this value is `false`, this option had a line condition on it that failed.
    /// The option will still be delivered to the game, but, depending on the needs of the game,
    /// the game may decide to not allow the player to select it, or not offer it to the player at all.
    ///
    /// This is intended for situations where games wish to show options that the player _could_ have taken,
    /// if some other condition had been met (e.g. having enough "charisma" points).
    pub is_available: bool,
}

impl DialogueOption {
    pub(crate) fn from_yarn_dialogue_option(
        yarn_dialogue_option: yarnspinner::prelude::DialogueOption,
        assets: LineAssets,
        metadata: Vec<String>,
    ) -> Self {
        Self {
            line: LocalizedLine::from_yarn_line(yarn_dialogue_option.line, assets, metadata),
            id: yarn_dialogue_option.id,
            destination_node: yarn_dialogue_option.destination_node,
            is_available: yarn_dialogue_option.is_available,
        }
    }
}
