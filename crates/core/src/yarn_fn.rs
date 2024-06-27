//! Not part of the original implementation, but needed due to Rust's type system.
//! Inspired by how Bevy stores [`FnSystem`](https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/struct.FnSystem.html)s.
//! This is all here just to emulate the `Dictionary<string, Delegate>` used in Yarn Spinner's `Library` class.

mod function_registry;
mod function_wrapping;
mod optionality;
mod parameter_wrapping;

pub(crate) use function_registry::*;
pub use {function_wrapping::*, parameter_wrapping::*};
