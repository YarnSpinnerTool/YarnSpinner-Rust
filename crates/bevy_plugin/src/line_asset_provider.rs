use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::HashSet;
use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn line_asset_provider_plugin(_app: &mut App) {}

pub trait LineAssetProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn LineAssetProvider>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn set_localizations(&mut self, localizations: Localizations);
    fn set_language(&mut self, language: Option<Language>);
    fn get_language(&self) -> Option<Language>;
    fn lines_available(&self) -> bool;
    fn accept_line_hints(&mut self, line_ids: &[LineId]);
    fn get_asset(&self, line: &UnderlyingYarnLine) -> Option<HandleUntyped>;
}

impl Clone for Box<dyn LineAssetProvider> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[derive(Clone)]
pub struct AudioAssetProvider {
    language: Arc<RwLock<Option<Language>>>,
    localizations: Arc<RwLock<Option<Localizations>>>,
    asset_server: Arc<RwLock<AssetServer>>,
    handles: Arc<RwLock<HashSet<HandleUntyped>>>,
    line_ids: Arc<RwLock<HashSet<LineId>>>,
}

impl Debug for AudioAssetProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioAssetProvider")
            .field("language", &self.language)
            .field("localizations", &self.localizations)
            .field("asset_server", &())
            .finish()
    }
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

    fn set_localizations(&mut self, localizations: Localizations) {
        self.localizations.write().unwrap().replace(localizations);
    }

    fn set_language(&mut self, language: Option<Language>) {
        *self.language.write().unwrap() = language;
        self.reload_assets();
    }
    fn get_language(&self) -> Option<Language> {
        self.language.read().unwrap().clone()
    }

    fn lines_available(&self) -> bool {
        if self.language.read().unwrap().is_none()
            || self.localizations.read().unwrap().is_none()
            || self.line_ids.read().unwrap().is_empty()
            || self.handles.read().unwrap().is_empty()
        {
            return false;
        };
        let asset_server = self.asset_server.read().unwrap();
        self.handles
            .read()
            .unwrap()
            .iter()
            .all(|handle| asset_server.get_load_state(handle) == LoadState::Loading)
    }

    fn accept_line_hints(&mut self, line_ids: &[LineId]) {
        {
            let mut current_line_ids = self.line_ids.write().unwrap();
            current_line_ids.clear();
            current_line_ids.extend(line_ids.iter().cloned());
        }
        self.reload_assets();
    }

    fn get_asset(&self, line: &UnderlyingYarnLine) -> Option<HandleUntyped> {
        let localizations = self.localizations.read().unwrap();
        let language = self.language.read().unwrap();
        if let Some(language) = language.as_ref() {
            if let Some(localizations) = localizations.as_ref() {
                if let Some(localization) = localizations.translation(language) {
                    let dir = localization.assets_sub_folder.as_path();
                    let file_name = format!("{}.ogg", line.id.0.trim_start_matches("line:"));
                    let path = dir.join(file_name);
                    let asset_server = self.asset_server.read().unwrap();

                    if asset_server.asset_io().is_file(&path) {
                        let handle = asset_server.load_untyped(path);
                        return Some(handle);
                    } else {
                        warn!(
                            "Audio file \"{path}\" for line \"{line_id}\" does not exist",
                            path = path.display(),
                            line_id = line.id.0
                        );
                    }
                } else {
                    error!("Tried to get audio asset for \"{language}\", which is a language that is not supported by localizations");
                }
            }
        }
        None
    }
}

impl AudioAssetProvider {
    fn reload_assets(&mut self) {
        let localizations = self.localizations.read().unwrap();
        let language = self.language.read().unwrap();
        if let Some(language) = language.as_ref() {
            if let Some(localizations) = localizations.as_ref() {
                if let Some(localization) = localizations.translation(language) {
                    let dir = localization.assets_sub_folder.as_path();
                    let mut handles = self.handles.write().unwrap();
                    handles.clear();
                    let asset_server = self.asset_server.read().unwrap();
                    for line_id in self.line_ids.read().unwrap().iter() {
                        let file_name = format!("{}.ogg", line_id.0.trim_start_matches("line:"));
                        let path = dir.join(file_name);
                        if asset_server.asset_io().is_file(&path) {
                            let handle = asset_server.load_untyped(path);
                            handles.insert(handle);
                        } else {
                            warn!(
                                "Audio file \"{path}\" for line \"{line_id}\" does not exist",
                                path = path.display(),
                                line_id = line_id.0
                            );
                        }
                    }
                } else {
                    error!("Tried to get audio asset for \"{language}\", which is a language that is not supported by localizations");
                }
            }
        }
    }
}
