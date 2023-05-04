mod dialogue;
mod dialogue_option;
mod handlers;
mod line;
mod variable_storage;

pub mod prelude {
    pub use crate::{dialogue::*, dialogue_option::*, handlers::*, line::*, variable_storage::*};
}
