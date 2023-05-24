use crate::prelude::*;
use crate::yarn_slinger::BorrowedLine;
use bevy::prelude::*;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn line_asset_provider_plugin(_app: &mut App) {}

pub trait LineAssetProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn LineAssetProvider>;
    fn set_language(&mut self, language: Option<Language>);
    fn get_language(&self) -> Option<Language>;
    fn get_asset<'a, 'b, 'c>(
        &'a self,
        line: BorrowedLine<'b>,
        asset_server: &'c AssetServer,
    ) -> Option<HandleUntyped>;
}

impl Clone for Box<dyn LineAssetProvider> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[derive(Debug, Clone)]
pub struct AudioAssetProvider {
    pub audio_language: Arc<RwLock<Option<Language>>>,
}

impl LineAssetProvider for AudioAssetProvider {
    fn clone_shallow(&self) -> Box<dyn LineAssetProvider> {
        Box::new(self.clone())
    }
    fn set_language(&mut self, language: Option<Language>) {
        *self.audio_language.write().unwrap() = language;
    }
    fn get_language(&self) -> Option<Language> {
        self.audio_language.read().unwrap().clone()
    }
    fn get_asset<'a, 'b, 'c>(
        &'a self,
        _line: BorrowedLine<'b>,
        _asset_server: &'c AssetServer,
    ) -> Option<HandleUntyped> {
        todo!();
    }
}
