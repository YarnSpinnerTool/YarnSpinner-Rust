//! # Bevy Yarn Slinger
//!
//! This is the Bevy integration for Yarn Slinger, the friendly dialogue creation tool for Rust.
//! It allows you to easily create dialogue systems in your game.
//!
//! ## Usage
//!
//! The three main types you will interact with are:
//! - [`YarnSlingerPlugin`]: The plugin registering all systems and types.
//! - [`YarnProject`]: A [`Resource`](bevy::prelude::Resource) for the compiled Yarn project, which is created for you when [`YarnSlingerPlugin`] is added.
//! - [`DialogueRunner`]: The [`Component`](bevy::prelude::Component) running through the Yarn files and sending events for things you should draw on the screen.
//! Can be created from a [`YarnProject`].
//!
//! ## Dialogue Views
//!
//! The dialogue runner itself does not draw anything to the screen, it only tells you what content to present.
//! Any plugin that handles the actual drawing is called a *dialogue view*. We provide an [example dialogue view](https://crates.io/crates/bevy_yarn_slinger_example_dialogue_view)
//! that you can use to explore the features of Yarn Slinger and get started quickly.
//!
//! Specifically, a dialogue view is required to do the following things
//! - Handle the [`PresentLineEvent`](crate::events::PresentLineEvent) and draw the line to the screen.
//! - Handle the [`PresentOptionsEvent`](crate::events::PresentOptionsEvent) and draw the options to the screen.
//! - Call [`DialogueRunner::continue_in_next_update`](crate::prelude::DialogueRunner::continue_in_next_update) when the user wishes to continue the dialogue.
//! - Pass a user's option selection to the right dialogue runner via [`DialogueRunner::select_option`](crate::prelude::DialogueRunner::select_option).
//!
//! See the documentation for the [`events`] module for additional optional events that may be handled
//!
//! Note that while [`DialogueRunner`]s are setup in such a way that you can have multiple instances running in parallel (such as for split-screen co-op),
//! a general-purpose dialogue view is not required to support this use-case, as every game that does this will have it's own way of wanting to deal with this.
//! In particular, the [example dialogue view](https://crates.io/crates/bevy_yarn_slinger_example_dialogue_view) only supports a single [`DialogueRunner`].
//!
//! ## Demo
//!
//! You can play a the [Yarn Slinger Demo](https://janhohenheim.itch.io/yarn-slinger-demo) in your browser to see the aforementioned example dialogue view in action.
//! Additionally, there are [many examples](https://github.com/yarn-slinger/yarn-slinger/tree/main/examples/bevy_yarn_slinger/src/bin) that you can check out.
//!
//! ## Example
//!
//! The main workflow is as follows:
//! - Register the [`YarnSlingerPlugin`]
//! - When the [`YarnProject`] [`Resource`](bevy::prelude::Resource) is added, spawn a [`DialogueRunner`] from it.
//! The latter can nicely be done with `my_system.run_if(resource_added::<YarnProject>())`.
//!
//! The following example is adapted from the [hello world example](https://github.com/yarn-slinger/yarn-slinger/blob/main/examples/bevy_yarn_slinger/src/bin/hello_world.rs).
//!
//! ```yarn
//! // assets/dialogue/hello_world.yarn
//! title: Start
//! ---
//! Hello world!
//! ===
//! ```
//!
//! ```no_run
//! // src/main.rs
//! use bevy::prelude::*;
//! use bevy_yarn_slinger::prelude::*;
//! // Use the example dialogue view to see the dialogue in action. Requires the `bevy_yarn_slinger_example_dialogue_view` crate.
//! // use bevy_yarn_slinger_example_dialogue_view::prelude::*;
//!
//! fn main() {
//!     let mut app = App::new();
//!     app.add_plugins(DefaultPlugins)
//!         // Register the Yarn Slinger plugin using its default settings, which will look for Yarn files in the "dialogue" folder
//!         // If this app should support Wasm or Android, we cannot load files without specifying them, so use the following instead.
//!         // .add_plugin(YarnSlingerPlugin::with_yarn_source(YarnFileSource::file("dialogue/hello_world.yarn")))
//!         .add_plugin(YarnSlingerPlugin::new())
//!         // Initialize the bundled example UI. Requires the `bevy_yarn_slinger_example_dialogue_view` crate.
//!         // .add_plugin(ExampleYarnSlingerDialogueViewPlugin::new())
//!         .add_systems((
//!             setup_camera.on_startup(),
//!             // Spawn dialogue runner once the Yarn project has finished compiling
//!             spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
//!         ))
//!         .run();
//! }
//!
//! fn setup_camera(mut commands: Commands) {
//!     commands.spawn(Camera2dBundle::default());
//! }
//!
//! fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
//!     // Create a dialogue runner from the project
//!     let mut dialogue_runner = project.create_dialogue_runner();
//!     // Immediately show the dialogue to the player by starting at the "Start" node
//!     dialogue_runner.start_node("Start");
//!     commands.spawn(dialogue_runner);
//! }
//! ```
//!
//! [`DialogueRunner`]: crate::prelude::DialogueRunner
//! [`YarnProject`]: crate::prelude::YarnProject
//! [`YarnSlingerPlugin`]: crate::prelude::YarnSlingerPlugin

#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs, missing_debug_implementations)]

mod commands;
mod development_file_generation;
mod dialogue_runner;
mod line_provider;
mod localization;
mod plugin;
mod project;
mod utils;
mod yarn_file_asset;
pub use anyhow::{Error, Result};

pub mod default_impl {
    //! Default implementations for Yarn Slinger traits.
    #[cfg(feature = "audio_assets")]
    pub use crate::line_provider::AudioAssetProvider;
    pub use crate::line_provider::{
        file_extensions, FileExtensionAssetProvider, StringsFileTextProvider,
    };
    pub use yarn_slinger::runtime::{MemoryVariableStore, StringTableTextProvider};
}

pub mod events {
    //! Events that are sent by the [`DialogueRunner`](crate::prelude::DialogueRunner). A dialogue view is expected to at least handle [`PresentLineEvent`] and [`PresentOptionsEvent`].
    pub use crate::dialogue_runner::{
        DialogueCompleteEvent, DialogueStartEvent, ExecuteCommandEvent, LineHintsEvent,
        NodeCompleteEvent, NodeStartEvent, PresentLineEvent, PresentOptionsEvent,
    };
}

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.

    #[cfg(feature = "audio_assets")]
    pub use crate::default_impl::AudioAssetProvider;
    pub use crate::{
        commands::{YarnCommand, YarnCommands},
        default_impl::FileExtensionAssetProvider,
        development_file_generation::DevelopmentFileGeneration,
        dialogue_runner::{DialogueOption, DialogueRunner, DialogueRunnerBuilder, LocalizedLine},
        line_provider::{AssetProvider, LineAssets, TextProvider},
        localization::{Localization, Localizations},
        plugin::{YarnFileSource, YarnSlingerPlugin, YarnSlingerSystemSet},
        project::YarnProject,
        yarn_file_asset::YarnFile,
    };
    pub(crate) use crate::{localization::StringsFile, utils::*};
    pub(crate) use anyhow::{Context, Error, Result};
    pub(crate) use yarn_slinger::prelude::*;
    pub use yarn_slinger::prelude::{
        IntoYarnValueFromNonYarnValue, Language, LineId, MarkupAttribute, MarkupValue, OptionId,
        VariableStorage, YarnFn, YarnLibrary,
    };
    pub(crate) type SystemResult = Result<()>;
    pub(crate) use seldom_fn_plugin::FnPluginExt;
    pub(crate) use serde::{Deserialize, Serialize};
}

pub use crate::commands::{TaskFinishedIndicator, UntypedYarnCommand};
pub use crate::dialogue_runner::{InnerDialogue, InnerDialogueMut};
pub use yarn_slinger::core::{yarn_fn_type, UntypedYarnFn};
pub use yarn_slinger::prelude::{
    Compilation, StringInfo, TextProvider as UnderlyingTextProvider, YarnAnalysisContext,
    YarnCommand as UnderlyingYarnCommand, YarnLine as UnderlyingYarnLine,
};

pub mod deferred_loading {
    //! Contains types needed for the deferred loading functionality, which is used when the list of yarn files is not immediately available at startup.
    pub use crate::plugin::DeferredYarnSlingerPlugin;
    pub use crate::project::LoadYarnProjectEvent;
}
