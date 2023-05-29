#![allow(clippy::too_many_arguments)]

mod commands;
mod dialogue_runner;
mod line_provider;
mod localization;
mod plugin;
mod project;
mod utils;
mod yarn_file_asset;
pub use anyhow::{Error, Result};

pub mod default_impl {
    pub use crate::line_provider::{FileExtensionAssetProvider, StringsFileTextProvider};
    pub use yarn_slinger::runtime::{MemoryVariableStore, StringTableTextProvider};
}

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.

    pub use crate::{
        commands::{YarnCommand, YarnCommandRegistrations},
        default_impl::FileExtensionAssetProvider,
        dialogue_runner::{
            DialogueCompleteEvent, DialogueOption, DialogueRunner, DialogueRunnerBuilder,
            ExecuteCommandEvent, LineHintsEvent, LocalizedLine, NodeCompleteEvent, NodeStartEvent,
            PresentLineEvent, PresentOptionsEvent,
        },
        line_provider::{AssetProvider, LineAssets, TextProvider},
        localization::{FileGenerationMode, Localization, Localizations},
        plugin::{YarnFileSource, YarnSlingerPlugin},
        project::YarnProject,
        yarn_file_asset::YarnFile,
    };
    pub(crate) use crate::{localization::StringsFile, utils::*};
    pub(crate) use anyhow::{Context, Error, Result};
    pub(crate) use yarn_slinger::prelude::*;
    pub use yarn_slinger::prelude::{
        Language, LineId, MarkupAttribute, MarkupValue, OptionId, VariableStorage, YarnFn,
        YarnFnLibrary,
    };
    pub(crate) type SystemResult = Result<()>;
    pub(crate) use seldom_fn_plugin::FnPluginExt;
    pub(crate) use serde::{Deserialize, Serialize};
}

pub use yarn_slinger::core::yarn_fn_type;
pub use yarn_slinger::prelude::{
    Compilation, StringInfo, TextProvider as UnderlyingTextProvider, YarnAnalysisContext,
    YarnCommand as UnderlyingYarnCommand, YarnLine as UnderlyingYarnLine,
};

pub mod deferred_loading {
    pub use crate::plugin::DeferredYarnSlingerPlugin;
    pub use crate::project::LoadYarnProjectEvent;
}
