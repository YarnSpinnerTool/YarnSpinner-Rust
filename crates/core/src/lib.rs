//! The core components of Yarn Spinner, used for both the compiler and the runtime. These mostly follow the same structure as in the original Yarn Spinner.
//!
//! You probably don't want to use this crate directly.
//! - If you're a game developer, you'll want to use a crate that is already designed for your game engine of choice,
//!   such as [`bevy_yarnspinner`](https://crates.io/crates/bevy_yarnspinner) for the [Bevy engine](https://bevyengine.org/).
//! - If you wish to write an adapter crate for an engine yourself, use the [`yarnspinner`](https://crates.io/crates/yarnspinner) crate.

#![warn(missing_docs, missing_debug_implementations)]
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

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
    #[cfg(any(feature = "bevy", feature = "serde"))]
    pub use crate::feature_gates::*;

    // Re-export alloc types for internal use only
    pub(crate) use crate::{
        alloc::borrow::ToOwned,
        alloc::boxed::Box,
        alloc::format,
        alloc::string::{String, ToString},
        alloc::vec,
        alloc::vec::Vec,
    };

    pub use crate::{
        generated::{
            Header, Instruction, InvalidOpCodeError, Node, Operand, Program, instruction::OpCode,
            operand::Value as OperandValue,
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
