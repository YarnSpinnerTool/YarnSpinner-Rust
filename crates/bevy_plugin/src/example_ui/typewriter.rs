use bevy::prelude::*;
use bevy::utils::Instant;

pub(crate) fn typewriter_plugin(_app: &mut App) {}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub(crate) struct TypeWrittenText {
    pub(crate) character_name: Option<String>,
    pub(crate) line: String,
    pub(crate) last_before_options: bool,
    pub(crate) start: Instant,
}
