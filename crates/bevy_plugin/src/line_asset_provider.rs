use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::prelude::*;
use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn line_asset_provider_plugin(_app: &mut App) {}

pub trait LineAssetProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn LineAssetProvider>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn set_language(&mut self, language: Option<Language>);
    fn get_language(&self) -> Option<Language>;
    fn lines_available(&self) -> bool;
    fn accept_line_hints(&self, line_ids: &[LineId]);
    fn get_asset(
        &self,
        line: &UnderlyingYarnLine,
        asset_server: &AssetServer,
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_language(&mut self, language: Option<Language>) {
        *self.audio_language.write().unwrap() = language;
    }
    fn get_language(&self) -> Option<Language> {
        self.audio_language.read().unwrap().clone()
    }

    fn lines_available(&self) -> bool {
        todo!()
    }

    fn accept_line_hints(&self, _line_ids: &[LineId]) {}
    fn get_asset(
        &self,
        _line: &UnderlyingYarnLine,
        _asset_server: &AssetServer,
    ) -> Option<HandleUntyped> {
        todo!();
    }
}
