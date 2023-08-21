//! A simple example dialogue view for Yarn Slinger.
//! A dialogue view is a plugin that handles presenting lines and options to the user and advances the dialogue on user input.
//! This one shows text in a dialogue box inspired by Legend of Zelda: Breath of the Wild.
//!
//! ## Demo
//!
//! The [Yarn Slinger Demo](https://janhohenheim.itch.io/yarn-slinger-demo) uses this dialogue view, so you can play that in the browser if you
//! want to see it in action. Additionally, all [Bevy Yarn Slinger examples](https://github.com/yarn-slinger/yarn-slinger/tree/main/examples/bevy_yarn_slinger/src/bin) use
//! this dialogue view as well.
//!
//! ## Usage
//!
//! It's enough to simply register [`ExampleYarnSlingerDialogueViewPlugin`] alongside [`YarnSlingerPlugin`](bevy_yarn_slinger::prelude::YarnSlingerPlugin):
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_yarn_slinger::*;
//! use bevy_yarn_slinger::prelude::YarnSlingerPlugin;
//! use bevy_yarn_slinger_example_dialogue_view::prelude::*;
//!
//! App::new()
//!    .add_plugins(DefaultPlugins)
//!    .add_plugins(YarnSlingerPlugin::new())
//!    .add_plugins(ExampleYarnSlingerDialogueViewPlugin::new());
//! ```
//!
//! This crate also exposes the [`SpeakerChangeEvent`] which you can use to animate characters while they are speaking,
//! as the text is written out over a few seconds.
//!
//! ## Inputs
//!
//! - Advance the dialogue: press the space bar, left click or tap the screen after the text is done typing.
//! - Type out the text faster: Same as above, but hold press before the text is done typing.
//! - Select an option: press the number key corresponding to the option you want to select or click/tap the option.
//!
//! ## Limitations
//!
//! This dialogue view expects only a single instance of [`DialogueRunner`](bevy_yarn_slinger::prelude::DialogueRunner) to be running.
//! Its behavior is otherwise undefined.

#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs, missing_debug_implementations)]

use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;
pub use updating::SpeakerChangeEvent;

pub mod prelude {
    //! Everything you need to get starting using this example Yarn Slinger dialogue view.
    pub use crate::{
        ExampleYarnSlingerDialogueViewPlugin, ExampleYarnSlingerDialogueViewSystemSet,
        SpeakerChangeEvent,
    };
}

/// The plugin registering all systems of the dialogue view.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ExampleYarnSlingerDialogueViewPlugin;

/// The [`SystemSet`] containing all systems added by the [`ExampleYarnSlingerDialogueViewPlugin`].
/// Is run after the [`YarnSlingerSystemSet`](bevy_yarn_slinger::prelude::YarnSlingerSystemSet).
#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct ExampleYarnSlingerDialogueViewSystemSet;

impl ExampleYarnSlingerDialogueViewPlugin {
    /// Creates a new example dialogue view
    pub fn new() -> Self {
        Self::default()
    }
}

mod assets;
mod option_selection;
mod setup;
mod typewriter;
mod updating;

impl Plugin for ExampleYarnSlingerDialogueViewPlugin {
    fn build(&self, app: &mut App) {
        app.fn_plugin(assets::ui_assets_plugin)
            .fn_plugin(setup::ui_setup_plugin)
            .fn_plugin(updating::ui_updating_plugin)
            .fn_plugin(typewriter::typewriter_plugin)
            .fn_plugin(option_selection::option_selection_plugin);
    }
}
