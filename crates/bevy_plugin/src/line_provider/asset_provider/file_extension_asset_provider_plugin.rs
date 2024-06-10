use crate::fmt_utils::SkipDebug;
use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::asset::{LoadState, LoadedUntypedAsset};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use std::any::Any;
use std::fmt::Debug;
use std::path::PathBuf;

pub(crate) fn file_extension_asset_provider_plugin(_app: &mut App) {}

/// An [`AssetProvider`] that loads assets from disk based on their file extension and name.
/// If the file extension "png" is associated with the type [`Image`] via [`FileExtensionAssetProvider::with_file_extensions`], and this provider is
/// asked to look up the asset for a line with the ID "123", it will look for a file named
/// "123.png" in the line asset subdirectory specified by the [`Localization`] corresponding to the language set by [`DialogueRunner::set_asset_language`].
///
/// By default, the line asset subdirectory will be `"dialogue/<language>"`. So for the language "en-US" and the line ID "123", the provider will
/// specifically look for "assets/dialogue/en-US/123.png" when calling [`FileExtensionAssetProvider::get_assets`].
/// Because this requires knowledge of the current language, this provider will only fetch assets if you set up Yarn Spinner with [`Localizations`] using
/// [`YarnSpinnerPlugin::with_localizations`] or [`LoadYarnProjectEvent::with_localizations`](crate::deferred_loading::LoadYarnProjectEvent::with_localizations).
///
/// You can use this provider in a [`DialogueRunner`] by calling [`DialogueRunnerBuilder::add_asset_provider`] with an instance of this type.
///
/// If you want to load audio assets, the feature `audio_assets` will provide you with an [`AudioAssetProvider`] that is a wrapper around this type
/// configured in such a way.
#[derive(Clone, Default, Debug)]
pub struct FileExtensionAssetProvider {
    language: Option<Language>,
    localizations: Option<Localizations>,
    asset_server: SkipDebug<Option<AssetServer>>,
    loading_handles: HashMap<PathBuf, Handle<LoadedUntypedAsset>>,
    loaded_handles: HashMap<PathBuf, UntypedHandle>,
    line_ids: HashSet<LineId>,
    file_extensions: HashMap<&'static str, Vec<String>>,
}

/// A convenience macro for specifying file extensions used by [`FileExtensionAssetProvider::with_file_extensions`].
/// The syntax is as follows:
/// ```ignore
/// file_extensions! {
///     <AssetType1>: ["<ext1>", "<ext2>", ...],
///     <AssetType2>: ["<ext1>", "<ext2>", ...],
///     ...
/// }
/// ```
/// where `<AssetType>` is a type implementing [`Asset`](bevy::asset::Asset) and `<ext>` is a file extension without the leading dot.
/// See [`FileExtensionAssetProvider::with_file_extensions`] for an example.
#[macro_export]
macro_rules! file_extensions {
    ($($type:ty: $ext:expr),* $(,)?) => {
        {
            bevy::utils::HashMap::from([
                $(
                    (<$type as bevy::reflect::TypePath>::type_path(), $ext),
                )*
            ])
        }
    };
}
pub use file_extensions;

impl FileExtensionAssetProvider {
    /// Initializes a new [`FileExtensionAssetProvider`] with no file extensions.
    /// Call [`FileExtensionAssetProvider::with_file_extensions`] to add file extensions.
    pub fn new() -> Self {
        default()
    }

    /// Adds file extensions for the given type. For convenience, you can use the [`file_extensions`] macro to specify the extensions.
    ///
    /// ## Example
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_yarnspinner::file_extensions;
    /// use bevy_yarnspinner::prelude::*;
    ///
    /// let file_extension_provider = FileExtensionAssetProvider::new()
    ///     .with_file_extensions(file_extensions! {
    ///        Image: ["png", "jpg", "jpeg"],
    ///        AudioSource: ["mp3", "ogg", "wav"],
    ///     });
    /// ```
    pub fn with_file_extensions<T, U, V>(mut self, file_extensions: T) -> Self
    where
        T: IntoIterator<Item = (&'static str, U)>,
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
        self.reload_assets();
    }

    fn set_localizations(&mut self, localizations: Localizations) {
        self.localizations.replace(localizations);
    }

    fn set_asset_server(&mut self, asset_server: AssetServer) {
        self.asset_server.replace(asset_server);
    }

    fn update_asset_availability(
        &mut self,
        loaded_untyped_assets: &Assets<LoadedUntypedAsset>,
    ) -> bool {
        if self.language.is_none() || self.localizations.is_none() || self.line_ids.is_empty() {
            return false;
        };

        if self.loading_handles.is_empty() && self.loaded_handles.is_empty() {
            return true;
        }

        let Some(asset_server) = self.asset_server.as_ref() else {
            return false;
        };

        self.loading_handles.retain(|_path, handle| {
            asset_server.get_load_state(handle.id()) != Some(LoadState::Failed)
        });
        let newly_loaded: HashMap<_, _> = self
            .loading_handles
            .iter()
            .filter_map(|(path, handle)| {
                loaded_untyped_assets
                    .get(handle)
                    .map(|loaded| (path.clone(), loaded.handle.clone()))
            })
            .collect();
        self.loading_handles
            .retain(|path, _| !newly_loaded.contains_key(path));
        self.loaded_handles.extend(newly_loaded);
        if !self.loading_handles.is_empty() {
            false
        } else {
            self.loaded_handles
                .iter()
                .all(|(_path, handle)| asset_server.is_loaded_with_dependencies(handle.id()))
        }
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
                    let assets = self
                        .file_extensions
                        .iter()
                        .filter_map(|(type_id, exts)| {
                            exts.iter().find_map(|ext| {
                                let file_name = format!("{}.{}", file_name_without_extension, ext);
                                let path = dir.join(file_name);
                                self.loaded_handles
                                    .get(&path)
                                    .map(|handle| (*type_id, handle.clone()))
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
                    self.loading_handles.clear();
                    self.loaded_handles.clear();
                    let Some(asset_server) = self.asset_server.as_ref() else {
                        return;
                    };
                    for line_id in self.line_ids.iter() {
                        for extension in self.file_extensions.values().flatten() {
                            let file_name =
                                format!("{}.{extension}", line_id.0.trim_start_matches("line:"));
                            let path = dir.join(file_name);
                            let asset_path = path.to_string_lossy().replace('\\', "/");
                            let handle = asset_server.load_untyped(asset_path);
                            self.loading_handles.insert(path, handle);
                        }
                    }
                } else {
                    panic!("Tried to find an asset for \"{language}\", which is a language that is not supported by localizations");
                }
            }
        }
    }
}
