#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![warn(missing_docs, missing_debug_implementations)]
#![doc = include_str!("../../../readme.md")]
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
    //! Default implementations for Yarn Slinger traits.
    #[cfg(feature = "audio_assets")]
    pub use crate::line_provider::AudioAssetProvider;
    pub use crate::line_provider::{
        file_extensions, FileExtensionAssetProvider, StringsFileTextProvider,
    };
    pub use yarn_slinger::runtime::{MemoryVariableStore, StringTableTextProvider};
}

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.

    #[cfg(feature = "audio_assets")]
    pub use crate::default_impl::AudioAssetProvider;
    pub use crate::{
        commands::{YarnCommand, YarnCommandRegistrations},
        default_impl::FileExtensionAssetProvider,
        dialogue_runner::{
            DialogueCompleteEvent, DialogueOption, DialogueRunner, DialogueRunnerBuilder,
            DialogueStartEvent, ExecuteCommandEvent, LineHintsEvent, LocalizedLine,
            NodeCompleteEvent, NodeStartEvent, PresentLineEvent, PresentOptionsEvent, StartNode,
        },
        line_provider::{AssetProvider, LineAssets, TextProvider},
        localization::{FileGenerationMode, Localization, Localizations},
        plugin::{YarnFileSource, YarnSlingerPlugin, YarnSlingerSystemSet},
        project::YarnProject,
        yarn_file_asset::YarnFile,
    };
    pub(crate) use crate::{localization::StringsFile, utils::*};
    pub(crate) use anyhow::{Context, Error, Result};
    pub(crate) use yarn_slinger::prelude::*;
    pub use yarn_slinger::prelude::{
        IntoYarnValueFromNonYarnValue, Language, LineId, MarkupAttribute, MarkupValue, OptionId,
        VariableStorage, YarnFn, YarnFnLibrary,
    };
    pub(crate) type SystemResult = Result<()>;
    pub(crate) use seldom_fn_plugin::FnPluginExt;
    pub(crate) use serde::{Deserialize, Serialize};
}

pub use crate::commands::UntypedYarnCommand;
pub use crate::dialogue_runner::{InnerDialogue, InnerDialogueMut};
pub use yarn_slinger::core::{yarn_fn_type, UntypedYarnFn};
pub use yarn_slinger::prelude::{
    Compilation, StringInfo, TextProvider as UnderlyingTextProvider, YarnAnalysisContext,
    YarnCommand as UnderlyingYarnCommand, YarnLine as UnderlyingYarnLine,
};

pub mod deferred_loading {
    //! Contains types needed for the deferred loading functionality, which is used when the list of yarn files is not immediately available at startup.
    pub use crate::plugin::DeferredYarnSlingerPlugin;
    pub use crate::project::LoadYarnProjectEvent;
}
