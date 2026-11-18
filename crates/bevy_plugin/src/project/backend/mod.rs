//! Compilation backends. Exactly one is active, chosen by the `compiler` feature:
//! - `compiler`: the bespoke Rust compiler (deprecated).
//! - otherwise: the external `ysc` executable.
//!
//! Each provides a `plugin` that turns loaded [`YarnFile`](crate::prelude::YarnFile)s into a
//! [`YarnProject`](crate::prelude::YarnProject) and handles hot reloading.
#[cfg(feature = "compiler")]
mod compiler;
#[cfg(feature = "compiler")]
pub(crate) use compiler::*;
#[cfg(not(feature = "compiler"))]
mod ysc;
#[cfg(not(feature = "compiler"))]
pub use ysc::YscSettings;
#[cfg(not(feature = "compiler"))]
pub(crate) use ysc::*;
