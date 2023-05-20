use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn line_provider_plugin(_app: &mut App) {}

#[derive(Debug, Clone, Resource)]
pub(crate) struct TextLineProvider(pub(crate) Box<dyn TextProvider>);

pub trait LineAssetProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn LineAssetProvider>;
    fn set_language(&mut self, language: Language);
    fn get_asset(&self, line: &YarnLine) -> Option<HandleUntyped>;
}

impl Clone for Box<dyn LineAssetProvider> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[derive(Debug, Clone)]
pub struct AudioAssetProvider {
    pub audio_language: Arc<RwLock<Language>>,
}

impl LineAssetProvider for AudioAssetProvider {
    fn clone_shallow(&self) -> Box<dyn LineAssetProvider> {
        Box::new(self.clone())
    }

    fn set_language(&mut self, language: Language) {
        *self.audio_language.write().unwrap() = language;
    }

    fn get_asset(&self, line: &YarnLine) -> Option<HandleUntyped> {
        todo!();
    }
}
