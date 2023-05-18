mod assets;
mod config;
mod plugin;
mod utils;

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.
    pub(crate) use crate::utils::*;
    pub use crate::{config::*, plugin::*};
    pub(crate) use anyhow::{Context, Error, Result};
    pub use yarn_slinger::prelude::YarnFile;
    pub(crate) use yarn_slinger::prelude::*;
    pub(crate) type SystemResult = Result<()>;
}
