mod localization;
mod plugin;
mod utils;
mod yarn_file_asset;

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.
    pub(crate) use crate::utils::*;
    pub use crate::{localization::*, plugin::YarnSlingerPlugin, yarn_file_asset::YarnFile};
    pub(crate) use anyhow::{Context, Error, Result};
    pub use yarn_slinger::prelude::LineId;
    pub(crate) use yarn_slinger::prelude::*;
    pub(crate) type SystemResult = Result<()>;
    pub(crate) use seldom_fn_plugin::FnPluginExt;
    pub(crate) use serde::{Deserialize, Serialize};
}

pub mod filesystem_events {
    pub use crate::localization::CreateMissingStringsFilesEvent;
}
