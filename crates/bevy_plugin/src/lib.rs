mod localization;
mod plugin;
mod utils;
mod yarn_file_loader;

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.
    pub(crate) use crate::utils::*;
    pub use crate::{localization::*, plugin::*};
    pub(crate) use anyhow::{Context, Error, Result};
    pub use yarn_slinger::prelude::YarnFile;
    pub(crate) use yarn_slinger::prelude::*;
    pub(crate) type SystemResult = Result<()>;
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use seldom_fn_plugin::FnPluginExt;
}
