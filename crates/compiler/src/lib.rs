use std::path::Path;

mod compiler;
//mod intermediate;
mod output;
mod parser;

pub mod prelude {
    pub use crate::parser::generated::*;
}
