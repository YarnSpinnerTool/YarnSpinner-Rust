mod analyser;
mod dialogue;
mod dialogue_option;
mod events;
mod line;
pub mod markup;
pub mod pluralization;
mod string_newtype;
mod variable_storage;
mod virtual_machine;

pub(crate) use string_newtype::string_newtype;

pub use dialogue::Result;

pub mod prelude {
    pub(crate) use crate::virtual_machine::*;
    pub use crate::{
        analyser::*,
        dialogue::{Dialogue, DialogueError},
        dialogue_option::*,
        events::*,
        line::*,
        pluralization::*,
        variable_storage::*,
    };
}
