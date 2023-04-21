mod compiler;
mod error_listener;
mod input_manager;
mod output;
mod parser;

pub mod prelude {
    pub(crate) use crate::input_manager::*;
    pub use crate::{compiler::*, error_listener::*, output::*, parser::*};
}
