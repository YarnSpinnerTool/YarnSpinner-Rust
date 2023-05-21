#![allow(clippy::too_many_arguments)]

mod dialogue_runner;
mod line_provider;
mod localization;
mod plugin;
mod project;
mod utils;
mod yarn_file_asset;

pub mod default_impl {
    pub use yarn_slinger::runtime::{MemoryVariableStore, StringTableTextProvider};
}

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.
    pub use crate::{
        dialogue_runner::DialogueRunner,
        line_provider::{AudioAssetProvider, LineAssetProvider},
        localization::{CurrentLanguage, FileGenerationMode, Localization, Localizations},
        plugin::{YarnFileSource, YarnSlingerPlugin},
        project::{
            GlobalLineAssetProvider, GlobalTextProvider, GlobalVariableStorage, YarnFilesInProject,
        },
        yarn_file_asset::YarnFile,
    };
    pub(crate) use crate::{
        localization::{CurrentStringsFile, StringsFile},
        utils::*,
    };
    pub(crate) use anyhow::{Context, Error, Result};
    pub(crate) use yarn_slinger::prelude::*;
    pub use yarn_slinger::prelude::{Language, LineId, TextProvider, VariableStorage};
    pub(crate) type SystemResult = Result<()>;
    pub(crate) use seldom_fn_plugin::FnPluginExt;
    pub(crate) use serde::{Deserialize, Serialize};
}

pub mod filesystem_events {
    pub use crate::localization::{
        CreateMissingStringsFilesEvent, UpdateAllStringsFilesForYarnFileEvent,
    };
}
