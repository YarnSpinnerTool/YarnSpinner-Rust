use std::path::Path;

mod compiler;
mod output;
pub mod parser;

pub mod prelude {
    pub use crate::compiler::*;
    pub use crate::output::*;
}
