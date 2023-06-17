//! The runtime components of Yarn Slinger. These mostly follow the same structure as the original Yarn Spinner runtime.
//!
//! You probably don't want to use this crate directly.
//! - If you're a game developer, you'll want to use a crate that is already designed for your game engine of choice,
//! such as [`bevy_yarn_slinger`](https://crates.io/crates/bevy_yarn_slinger) for the [Bevy engine](https://bevyengine.org/).
//! - If you wish to write an adapter crate for an engine yourself, use the [`yarn_slinger`](https://crates.io/crates/yarn_slinger) crate.

#![warn(missing_docs, missing_debug_implementations)]
mod analyser;
mod command;
mod dialogue;
mod dialogue_option;
mod events;
mod language;
mod line;
pub mod markup;
mod pluralization;
mod text_provider;
mod variable_storage;
mod virtual_machine;

pub use dialogue::Result;

pub mod prelude {
    //! Everything you need to get starting using the Yarn Slinger runtime.
    pub use crate::{
        analyser::*,
        command::*,
        dialogue::{Dialogue, DialogueError},
        dialogue_option::*,
        events::*,
        language::*,
        line::*,
        markup::MarkupParseError,
        text_provider::*,
        variable_storage::*,
    };
    pub(crate) use crate::{pluralization::*, virtual_machine::*};
    pub(crate) use yarn_slinger_core::prelude::*;
}
