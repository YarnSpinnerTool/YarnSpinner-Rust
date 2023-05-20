use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn dialogue_plugin(app: &mut App) {}

#[derive(Debug, Component)]
pub struct DialogueRunner {
    yarn_files: Vec<Handle<YarnFile>>,
    compilation: Option<Compilation>,
    variable_storage: Option<Box<dyn VariableStorage>>,
    text_provider: Option<Box<dyn TextProvider>>,
}

impl DialogueRunner {
    pub fn new(yarn_files: Vec<Handle<YarnFile>>) -> Self {
        todo!()
    }

    pub fn override_global_variable_storage(mut self, storage: Box<dyn VariableStorage>) -> Self {
        todo!()
    }

    pub fn override_global_text_provider(mut self, provider: Box<dyn TextProvider>) -> Self {
        todo!()
    }

    pub fn override_global_asset_provider(mut self, provider: Box<dyn AssetProvider>) -> Self {
        todo!()
    }
}

pub trait AssetProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn AssetProvider + Send + Sync>;
    fn get_asset(&self, line: &YarnLine) -> Option<HandleUntyped>;
}

impl Clone for Box<dyn AssetProvider + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[derive(Debug, Clone)]
pub struct AudioAssetProvider {
    pub audio_language: Arc<RwLock<Language>>,
}
