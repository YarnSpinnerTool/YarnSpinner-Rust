use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Debug;
use thiserror::Error;

mod runtime_interaction;

pub(crate) fn dialogue_plugin(app: &mut App) {
    app.fn_plugin(runtime_interaction::runtime_interaction_plugin);
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Called a function that depends on a dialogue being initialized on a newly created one. Please wait one tick before calling this method so that the runtime has time to initialize itself.")]
    UninitializedDialogueError,
    #[error(transparent)]
    YarnRuntimeDialogueError(#[from] DialogueError),
}

#[derive(Debug, Default, Component)]
pub struct DialogueRunner {
    pub(crate) dialogue: Option<Dialogue>,
    pub(crate) variable_storage_override: Option<Box<dyn VariableStorage>>,
    pub(crate) text_provider_override: Option<Box<dyn TextProvider>>,
    pub(crate) line_asset_provider_override: Option<Option<Box<dyn LineAssetProvider>>>,
    pub(crate) library_buffer: Option<YarnFnLibrary>,
    pub(crate) continue_: bool,
}

impl DialogueRunner {
    pub fn new() -> Self {
        Self {
            dialogue: None,
            variable_storage_override: None,
            text_provider_override: None,
            line_asset_provider_override: None,
            library_buffer: None,
            continue_: false,
        }
    }

    pub fn override_variable_storage(mut self, storage: Box<dyn VariableStorage>) -> Self {
        self.variable_storage_override = Some(storage);
        self
    }

    pub fn override_text_provider(mut self, provider: Box<dyn TextProvider>) -> Self {
        self.text_provider_override = Some(provider);
        self
    }

    pub fn override_asset_provider(
        mut self,
        provider: impl Into<Option<Box<dyn LineAssetProvider>>>,
    ) -> Self {
        self.line_asset_provider_override = Some(provider.into());
        self
    }

    pub fn with_library(mut self, library: YarnFnLibrary) -> Self {
        self.extend_library(library);
        self
    }

    pub fn extend_library(&mut self, library: YarnFnLibrary) -> &mut Self {
        let own_library = self
            .dialogue
            .as_mut()
            .map(|dialogue| dialogue.library_mut())
            .unwrap_or_else(|| {
                self.library_buffer
                    .get_or_insert_with(YarnFnLibrary::standard_library)
            });
        own_library.extend(library);
        self
    }

    pub fn continue_(&mut self) {
        self.continue_ = true;
    }

    pub fn select_option(&mut self, option: OptionId) -> Result<()> {
        if let Some(dialogue) = &mut self.dialogue {
            dialogue.set_selected_option(option).map_err(Error::from)?;
        } else {
            return Err(Error::UninitializedDialogueError);
        }
        Ok(())
    }
}
