mod dialogue;
mod dialogue_option;
mod handlers;
mod line;
mod variable_storage;
mod virtual_machine;

pub mod prelude {
    pub(crate) use crate::virtual_machine::*;
    pub use crate::{dialogue::*, dialogue_option::*, handlers::*, line::*, variable_storage::*};
}
