#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs, missing_debug_implementations)]

use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;
pub use updating::SpeakerChangeEvent;

pub mod prelude {
    pub use crate::{
        ExampleYarnSlingerUiPlugin, ExampleYarnSlingerUiSystemSet, SpeakerChangeEvent,
    };
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ExampleYarnSlingerUiPlugin;

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct ExampleYarnSlingerUiSystemSet;

impl ExampleYarnSlingerUiPlugin {
    pub fn new() -> Self {
        Self::default()
    }
}

mod assets;
mod option_selection;
mod setup;
mod typewriter;
mod updating;

impl Plugin for ExampleYarnSlingerUiPlugin {
    fn build(&self, app: &mut App) {
        app.fn_plugin(assets::ui_assets_plugin)
            .fn_plugin(setup::ui_setup_plugin)
            .fn_plugin(updating::ui_updating_plugin)
            .fn_plugin(typewriter::typewriter_plugin)
            .fn_plugin(option_selection::option_selection_plugin);
    }
}
