//! Compilation backend: the external `ysc` executable.
//!
//! Provides a `plugin` that turns loaded [`YarnFile`](crate::prelude::YarnFile)s into a
//! [`YarnProject`](crate::prelude::YarnProject) and handles hot reloading.
mod ysc;
pub use ysc::YscSettings;
pub(crate) use ysc::*;
