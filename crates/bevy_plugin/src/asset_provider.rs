use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::asset::{Asset, HandleId, LoadState};
use bevy::prelude::*;
use bevy::utils::HashSet;
use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn asset_provider_plugin(_app: &mut App) {}

pub trait AssetProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn AssetProvider>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn set_asset_server(&mut self, asset_server: AssetServer);
    fn set_localizations(&mut self, localizations: Localizations);
    fn set_language(&mut self, language: Option<Language>);
    fn get_language(&self) -> Option<Language>;
    fn assets_available(&self) -> bool;
    fn accept_line_hints(&mut self, line_ids: &[LineId]);
    fn get_assets(&self, line: &UnderlyingYarnLine) -> Assets;
}

impl Clone for Box<dyn AssetProvider> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[derive(Clone, Default)]
pub struct FileExtensionAssetProvider {
    language: Arc<RwLock<Option<Language>>>,
    localizations: Arc<RwLock<Option<Localizations>>>,
    asset_server: Arc<RwLock<Option<AssetServer>>>,
    handles: Arc<RwLock<HashSet<HandleUntyped>>>,
    line_ids: Arc<RwLock<HashSet<LineId>>>,
    file_extensions: Arc<RwLock<Vec<String>>>,
}

impl FileExtensionAssetProvider {
    pub fn with_file_extensions(file_extensions: Vec<impl AsRef<str>>) -> Self {
        let file_extensions = file_extensions
            .into_iter()
            .map(|s| s.as_ref().trim_start_matches(".").to_owned())
            .collect();
        Self {
            file_extensions: Arc::new(RwLock::new(file_extensions)),
            ..default()
        }
    }
}

impl<T, U> From<T> for FileExtensionAssetProvider
where
    T: IntoIterator<Item = U>,
    U: AsRef<str>,
{
    fn from(file_extensions: T) -> Self {
        Self::with_file_extensions(file_extensions.into_iter().collect())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Assets(HashSet<HandleUntyped>);
impl Assets {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn get_handle<T>(&self) -> Option<Handle<T>>
    where
        T: Asset,
    {
        self.0.iter().find_map(|handle| {
            if let HandleId::Id(type_uuid, _) = handle.id() {
                (T::TYPE_UUID == type_uuid).then(|| handle.clone().typed())
            } else {
                None
            }
        })
    }
}

impl From<HashSet<HandleUntyped>> for Assets {
    fn from(h: HashSet<HandleUntyped>) -> Self {
        Self(h)
    }
}

impl IntoIterator for Assets {
    type Item = <HashSet<HandleUntyped> as IntoIterator>::Item;
    type IntoIter = <HashSet<HandleUntyped> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<HandleUntyped> for Assets {
    fn extend<T: IntoIterator<Item = HandleUntyped>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl Debug for FileExtensionAssetProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioAssetProvider")
            .field("language", &self.language)
            .field("localizations", &self.localizations)
            .field("asset_server", &())
            .field("handles", &self.handles)
            .field("line_ids", &self.line_ids)
            .finish()
    }
}

impl AssetProvider for FileExtensionAssetProvider {
    fn clone_shallow(&self) -> Box<dyn AssetProvider> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn set_asset_server(&mut self, asset_server: AssetServer) {
        self.asset_server.write().unwrap().replace(asset_server);
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

    fn assets_available(&self) -> bool {
        if self.language.read().unwrap().is_none()
            || self.localizations.read().unwrap().is_none()
            || self.line_ids.read().unwrap().is_empty()
            || self.handles.read().unwrap().is_empty()
        {
            return false;
        };
        let asset_server = self.asset_server.read().unwrap();
        let Some(asset_server) = asset_server.as_ref() else {
            return false;
        };
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

    fn get_assets(&self, line: &UnderlyingYarnLine) -> Assets {
        let localizations = self.localizations.read().unwrap();
        let language = self.language.read().unwrap();
        if let Some(language) = language.as_ref() {
            if let Some(localizations) = localizations.as_ref() {
                if let Some(localization) = localizations.translation(language) {
                    let dir = localization.assets_sub_folder.as_path();
                    let file_name_without_extension = line.id.0.trim_start_matches("line:");
                    let asset_server = self.asset_server.read().unwrap();
                    let Some(asset_server) = asset_server.as_ref() else {
                            return default();
                        };
                    return self
                        .file_extensions
                        .read()
                        .unwrap()
                        .iter()
                        .filter_map(|ext| {
                            let file_name = format!("{}.{}", file_name_without_extension, ext);
                            let path = dir.join(file_name);

                            (asset_server.asset_io().is_file(&path))
                                .then(|| asset_server.load_untyped(path))
                        })
                        .collect::<HashSet<_>>()
                        .into();
                } else {
                    error!("Tried to get audio asset for \"{language}\", which is a language that is not supported by localizations");
                }
            }
        }
        default()
    }
}

impl FileExtensionAssetProvider {
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
                    let Some(asset_server) = asset_server.as_ref() else {
                        return;
                    };
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
