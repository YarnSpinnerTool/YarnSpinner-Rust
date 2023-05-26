#![allow(clippy::too_many_arguments)]

mod dialogue_runner;
mod line_provider;
mod localization;
mod plugin;
mod project;
mod utils;
mod yarn_file_asset;
pub use anyhow::{Error, Result};

pub mod default_impl {
    pub use crate::line_provider::{
        FileExtensionAssetProvider, SharedTextProvider, StringsFileTextProvider,
    };
    pub use yarn_slinger::runtime::{MemoryVariableStore, StringTableTextProvider};
}

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.
    pub use crate::{
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
        Language, LineId, MarkupAttribute, MarkupValue, OptionId, VariableStorage, YarnCommand,
        YarnFn, YarnFnLibrary,
    };
    pub(crate) type SystemResult = Result<()>;
    pub(crate) use seldom_fn_plugin::FnPluginExt;
    pub(crate) use serde::{Deserialize, Serialize};
}

pub use crate::line_provider::GenericAsset;

pub use yarn_slinger::prelude::{
    Compilation, StringInfo, TextProvider as UnderlyingTextProvider, YarnAnalysisContext,
    YarnLine as UnderlyingYarnLine,
};
