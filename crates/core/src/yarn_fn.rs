//! Not part of the original implementation, but needed due to Rust's type system.
//! Inspired by how Bevy stores [`FnSystem`](https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/struct.FnSystem.html)s.
//! This is all here just to emulate the `Dictionary<string, Delegate>` used in Yarn Spinner's `Library` class.

mod function_regristry;
mod function_wrapping;

pub use {function_regristry::*, function_wrapping::*};
