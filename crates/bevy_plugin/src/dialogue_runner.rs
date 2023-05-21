use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Debug;

pub(crate) fn dialogue_plugin(_app: &mut App) {}

#[derive(Debug, Default, Component)]
pub struct DialogueRunner {
    pub(crate) dialogue: Option<Dialogue>,
    pub(crate) variable_storage_override: Option<Box<dyn VariableStorage>>,
    pub(crate) text_provider_override: Option<Box<dyn TextProvider>>,
    pub(crate) line_asset_provider_override: Option<Option<Box<dyn LineAssetProvider>>>,
}

impl DialogueRunner {
    pub fn new() -> Self {
        Self {
            dialogue: None,
            variable_storage_override: None,
            text_provider_override: None,
            line_asset_provider_override: None,
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
}
