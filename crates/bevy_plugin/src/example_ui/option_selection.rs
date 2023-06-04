use bevy::prelude::*;

pub(crate) fn option_selection_plugin(_app: &mut App) {}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub(crate) struct OptionSelection {}
