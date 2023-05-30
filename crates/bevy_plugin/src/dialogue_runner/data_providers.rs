use crate::prelude::*;
use bevy::prelude::*;
use std::any::TypeId;

pub(crate) fn dialogue_runner_data_providers_plugin(_app: &mut App) {}

pub struct DialogueRunnerDataProviders<'a>(pub(crate) &'a DialogueRunner);

pub struct DialogueRunnerDataProvidersMut<'a>(pub(crate) &'a mut DialogueRunner);

impl DialogueRunnerDataProviders<'_> {
    #[must_use]
    pub fn text_provider(&self) -> &dyn TextProvider {
        self.0.text_provider.as_ref()
    }

    #[must_use]
    pub fn asset_providers(&self) -> impl Iterator<Item = &dyn AssetProvider> {
        self.0
            .asset_providers
            .values()
            .map(|provider| provider.as_ref())
    }

    #[must_use]
    pub fn asset_provider<T: 'static>(&self) -> Option<&T> {
        self.0
            .asset_providers
            .get(&TypeId::of::<T>())
            .and_then(|provider| provider.as_any().downcast_ref())
    }

    #[must_use]
    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.0.dialogue.variable_storage()
    }

    #[must_use]
    pub fn are_texts_available(&self) -> bool {
        self.text_provider().are_lines_available()
    }

    #[must_use]
    pub fn are_assets_available(&self) -> bool {
        self.asset_providers()
            .all(|provider| provider.are_assets_available())
    }

    #[must_use]
    pub fn are_lines_available(&self) -> bool {
        self.are_texts_available() && self.are_assets_available()
    }
}

impl DialogueRunnerDataProvidersMut<'_> {
    #[must_use]
    pub fn text_provider(&self) -> &dyn TextProvider {
        self.0.text_provider.as_ref()
    }

    #[must_use]
    pub fn asset_providers(&self) -> impl Iterator<Item = &dyn AssetProvider> {
        self.0
            .asset_providers
            .values()
            .map(|provider| provider.as_ref())
    }

    #[must_use]
    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.0.dialogue.variable_storage()
    }

    #[must_use]
    pub fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.0.text_provider.as_mut()
    }

    #[must_use]
    pub fn asset_providers_mut(&mut self) -> impl Iterator<Item = &mut dyn AssetProvider> {
        self.0
            .asset_providers
            .values_mut()
            // Source: <https://stackoverflow.com/a/55866511/5903309>
            .map(|x| &mut **x as &mut dyn AssetProvider)
    }

    #[must_use]
    pub fn asset_provider<T: 'static>(&self) -> Option<&T> {
        self.0
            .asset_providers
            .get(&TypeId::of::<T>())
            .and_then(|provider| provider.as_any().downcast_ref())
    }

    #[must_use]
    pub fn asset_provider_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0
            .asset_providers
            .get_mut(&TypeId::of::<T>())
            .and_then(|provider| provider.as_any_mut().downcast_mut())
    }

    #[must_use]
    pub fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.0.dialogue.variable_storage_mut()
    }

    #[must_use]
    pub fn are_texts_available(&self) -> bool {
        self.text_provider().are_lines_available()
    }

    #[must_use]
    pub fn are_assets_available(&self) -> bool {
        self.asset_providers()
            .all(|provider| provider.are_assets_available())
    }

    #[must_use]
    pub fn are_lines_available(&self) -> bool {
        self.are_texts_available() && self.are_assets_available()
    }
}
