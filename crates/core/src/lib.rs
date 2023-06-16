//! The core components of Yarn Slinger, used for both the compiler and the runtime. These mostly follow the same structure as in the original Yarn Spinner.
//!
//! You probably don't want to use this crate directly. 
//! - If you're a game developer, you'll want to use a crate that is already designed for your game engine of choice,
//! such as [`bevy_yarn_slinger`](https://crates.io/crates/bevy_yarn_slinger) for the [Bevy engine](https://bevyengine.org/). 
//! - If you wish to write an adapter crate for an engine yourself, use the [`yarn_slinger`](https://crates.io/crates/yarn_slinger) crate.

#![warn(missing_docs, missing_debug_implementations)]
mod feature_gates;
mod generated;
mod internal_value;
mod library;
mod line_id;
mod operator;
mod position;
pub mod types;
mod yarn_fn;
mod yarn_value;

pub mod prelude {
    //! Types and functions used all throughout the runtime and compiler.
    pub use crate::{
        feature_gates::*,
        generated::{
            instruction::OpCode, operand::Value as OperandValue, Header, Instruction,
            InvalidOpCodeError, Node, Operand, Program,
        },
        internal_value::*,
        library::*,
        line_id::*,
        operator::*,
        position::*,
        types::Type,
        yarn_fn::*,
        yarn_value::*,
    };
}
