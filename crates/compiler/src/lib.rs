mod compiler;
mod error_listener;
mod output;
mod parser;

pub mod prelude {
    pub use crate::parser::generated::*;
}
