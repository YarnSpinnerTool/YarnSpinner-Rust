use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet, Uuid};
use std::any::Any;
use std::fmt::Debug;

pub(crate) fn file_extension_asset_provider_plugin(_app: &mut App) {}

#[derive(Clone, Default)]
pub struct FileExtensionAssetProvider {
    language: Option<Language>,
    localizations: Option<Localizations>,
    asset_server: Option<AssetServer>,
    handles: HashSet<HandleUntyped>,
    line_ids: HashSet<LineId>,
    file_extensions: HashMap<Uuid, Vec<String>>,
}

#[macro_export]
macro_rules! file_extensions {
    ($($type:ty: $ext:expr),* $(,)?) => {
        {
            bevy::utils::HashMap::from([
                $(
                    (<$type as bevy::reflect::TypeUuid>::TYPE_UUID, $ext),
                )*
            ])
        }
    };
}
pub use file_extensions;

impl FileExtensionAssetProvider {
    pub fn new() -> Self {
        default()
    }

    pub fn with_file_extensions<T, U, V>(mut self, file_extensions: T) -> Self
    where
        T: IntoIterator<Item = (Uuid, U)>,
        U: IntoIterator<Item = V>,
        V: AsRef<str>,
    {
        self.file_extensions
            .extend(file_extensions.into_iter().map(|(type_id, extensions)| {
                (
                    type_id,
                    extensions
                        .into_iter()
                        .map(|s| s.as_ref().trim_start_matches('.').to_owned())
                        .collect::<Vec<_>>(),
                )
            }));
        self
    }
}

impl AssetProvider for FileExtensionAssetProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_language(&self) -> Option<Language> {
        self.language.clone()
    }

    fn set_language(&mut self, language: Option<Language>) {
        self.language = language;
    }

    fn set_localizations(&mut self, localizations: Localizations) {
        self.localizations.replace(localizations);
    }

    fn set_asset_server(&mut self, asset_server: AssetServer) {
        self.asset_server.replace(asset_server);
    }

    fn are_assets_available(&self) -> bool {
        if self.language.is_none()
            || self.localizations.is_none()
            || self.line_ids.is_empty()
            || self.handles.is_empty()
        {
            return false;
        };
        let Some(asset_server) = self.asset_server.as_ref() else {
            return false;
        };
        self.handles
            .iter()
            .all(|handle| asset_server.get_load_state(handle) == LoadState::Loaded)
    }

    fn accept_line_hints(&mut self, line_ids: &[LineId]) {
        {
            self.line_ids.clear();
            self.line_ids.extend(line_ids.iter().cloned());
        }
        self.reload_assets();
    }

    fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets {
        if let Some(language) = self.language.as_ref() {
            if let Some(localizations) = self.localizations.as_ref() {
                if let Some(localization) = localizations.supported_localization(language) {
                    let dir = localization.assets_sub_folder.as_path();
                    let file_name_without_extension = line.id.0.trim_start_matches("line:");
                    let Some(asset_server) = self.asset_server.as_ref() else {
                            return default();
                        };
                    let assets = self
                        .file_extensions
                        .iter()
                        .filter_map(|(type_id, exts)| {
                            exts.iter().find_map(|ext| {
                                let file_name = format!("{}.{}", file_name_without_extension, ext);
                                let path = dir.join(file_name);

                                if asset_server.asset_io().is_file(&path) {
                                    Some((*type_id, asset_server.load_untyped(path)))
                                } else {
                                    None
                                }
                            })
                        })
                        .collect::<HashSet<_>>();
                    return LineAssets::with_assets(assets);
                } else {
                    panic!("Tried to find an asset for \"{language}\", which is a language that is not supported by localizations");
                }
            }
        }
        default()
    }
}

impl FileExtensionAssetProvider {
    fn reload_assets(&mut self) {
        if let Some(language) = self.language.as_ref() {
            if let Some(localizations) = self.localizations.as_ref() {
                if let Some(localization) = localizations.supported_localization(language) {
                    let dir = localization.assets_sub_folder.as_path();
                    self.handles.clear();
                    let Some(asset_server) = self.asset_server.as_ref() else {
                        return;
                    };
                    for line_id in self.line_ids.iter() {
                        let file_name = format!("{}.ogg", line_id.0.trim_start_matches("line:"));
                        let path = dir.join(file_name);
                        if asset_server.asset_io().is_file(&path) {
                            let handle = asset_server.load_untyped(path);
                            self.handles.insert(handle);
                        } else {
                            debug!(
                                "Asset file \"{path}\" for line \"{line_id}\" does not exist",
                                path = path.display(),
                                line_id = line_id.0
                            );
                        }
                    }
                } else {
                    panic!("Tried to find an asset for \"{language}\", which is a language that is not supported by localizations");
                }
            }
        }
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
