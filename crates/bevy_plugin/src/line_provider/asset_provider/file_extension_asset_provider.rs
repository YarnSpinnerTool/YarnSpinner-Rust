use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::HashSet;
use std::fmt::Debug;

pub(crate) fn file_extension_asset_provider_plugin(_app: &mut App) {}

#[derive(Clone, Default)]
pub struct FileExtensionAssetProvider {
    language: Option<Language>,
    localizations: Option<Localizations>,
    asset_server: Option<AssetServer>,
    handles: HashSet<HandleUntyped>,
    line_ids: HashSet<LineId>,
    file_extensions: Vec<String>,
}

impl FileExtensionAssetProvider {
    pub fn with_file_extensions(file_extensions: Vec<impl AsRef<str>>) -> Self {
        let file_extensions = file_extensions
            .into_iter()
            .map(|s| s.as_ref().trim_start_matches('.').to_owned())
            .collect();
        Self {
            file_extensions,
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

impl AssetProvider for FileExtensionAssetProvider {
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
            .all(|handle| asset_server.get_load_state(handle) == LoadState::Loading)
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
                    return self
                        .file_extensions
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
