mod analyser;
mod command;
mod dialogue;
mod dialogue_option;
mod events;
mod feature_gates;
mod line;
pub mod markup;
pub mod pluralization;
mod text_provider;
mod variable_storage;
mod virtual_machine;

pub use dialogue::Result;

pub mod prelude {
    pub use crate::{
        analyser::*,
        command::*,
        dialogue::{Dialogue, DialogueError},
        dialogue_option::*,
        events::*,
        line::*,
        pluralization::*,
        text_provider::*,
        variable_storage::*,
    };
    pub(crate) use crate::{feature_gates::*, virtual_machine::*};
}
