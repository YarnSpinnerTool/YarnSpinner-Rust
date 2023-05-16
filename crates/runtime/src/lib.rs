mod analyser;
mod command;
mod dialogue;
mod dialogue_option;
mod events;
mod line;
pub mod markup;
pub mod pluralization;
mod text_provider;
mod variable_storage;
mod virtual_machine;

pub use dialogue::Result;

pub mod prelude {
    pub(crate) use crate::virtual_machine::*;
    pub use crate::{
        analyser::*,
        command::*,
        dialogue::{Dialogue, DialogueError},
        dialogue_option::*,
        events::*,
        line::*,
        markup::MarkupParseError,
        pluralization::*,
        text_provider::*,
        variable_storage::*,
    };
    pub(crate) use yarn_slinger_core::prelude::*;
}
