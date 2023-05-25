use crate::prelude::*;
use crate::{UnderlyingTextProvider, UnderlyingYarnLine};
use bevy::asset::{Asset, HandleId, LoadState};
use bevy::prelude::*;
use bevy::utils::HashSet;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub(crate) fn asset_provider_plugin(app: &mut App) {
    app.add_system(fetch_resources);
}

pub trait AssetProvider: Debug + Send + Sync {
    fn get_language(&self) -> Option<Language>;
    fn set_language(&mut self, language: Option<Language>);
    fn are_assets_available(&self) -> bool;
    fn accept_line_hints(&mut self, line_ids: &[LineId]);
    fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets;
}

pub trait TextProvider: UnderlyingTextProvider {
    fn set_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>);
    fn extend_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>);
    fn accept_fetched_assets(&mut self, asset: Box<dyn Any>);
    fn fetch_assets(&self) -> Box<dyn Fn(&World) -> Option<Box<dyn Any + 'static>> + '_>;
}

#[derive(Clone)]
pub struct StringsFileTextProvider {
    asset_server: AssetServer,
    localizations: Option<Localizations>,
    language: Option<Language>,
    base_string_table: HashMap<LineId, StringInfo>,
    strings_file_handle: Option<Handle<StringsFile>>,
    translation_string_table: Option<HashMap<LineId, String>>,
}

impl Debug for StringsFileTextProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringsTableTextProvider")
            .field("asset_server", &())
            .field("localizations", &self.localizations)
            .finish()
    }
}

impl UnderlyingTextProvider for StringsFileTextProvider {
    fn accept_line_hints(&mut self, _line_ids: &[LineId]) {
        // no-op
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        self.translation_string_table
            .as_ref()
            .and_then(|table| table.get(id).cloned())
            .or_else(|| {
                if let Some(language) = self.language.as_ref() {
                    if self.translation_string_table.is_some() {
                        warn!("Did not find translation for line {id} in language {language} because it is untranslated, falling back to base language.");
                    } else {
                        warn!("Did not find translation for line {id} in language {language} because the strings file has not been loaded yet, falling back to base language.");
                    }
                }
                self.base_string_table.get(id).map(|info| info.text.clone())
            })
    }

    fn set_language(&mut self, language: Option<Language>) {
        if language == self.language {
            return;
        }

        self.set_language_invalidating_translation(language.clone());
        let Some(language) = language else {
            return;
        };

        let Some(localizations) = self.localizations.as_ref() else {
            panic!("Set language to {language}, but no localizations have been registered as supported.");
        };
        if language == localizations.base_language.language {
            return;
        }
        let Some(localization) = localizations.translation(&language) else {
            let languages = localizations.supported_languages().map(|l| l.0.as_str()).collect::<Vec<_>>().join(", ");
            panic!("Set language to {language}, but that language is not supported. Expected one of {languages}.");
        };
        let path = localization.strings_file.as_path();
        if self.asset_server.asset_io().is_file(path) {
            self.strings_file_handle
                .replace(self.asset_server.load(path));
        } else {
            panic!("Set language to {language}, but the expected strings file at {path} does not exist.", path = path.display());
        }
    }

    fn get_language(&self) -> Option<Language> {
        self.language.clone()
    }

    fn are_lines_available(&self) -> bool {
        let is_base_language = self.language.is_none();
        let has_fetched_translation = || self.translation_string_table.is_some();
        is_base_language || has_fetched_translation()
    }
}

impl StringsFileTextProvider {
    pub fn from_yarn_project(yarn_project: &YarnProject) -> Self {
        Self {
            asset_server: yarn_project.asset_server.clone(),
            localizations: yarn_project.localizations.clone(),
            language: None,
            base_string_table: yarn_project.compilation.string_table.clone(),
            strings_file_handle: None,
            translation_string_table: None,
        }
    }
    fn set_language_invalidating_translation(&mut self, language: impl Into<Option<Language>>) {
        self.language = language.into();
        self.translation_string_table = None;
        self.strings_file_handle = None;
    }
}

impl TextProvider for StringsFileTextProvider {
    fn set_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.base_string_table = string_table;
    }

    fn extend_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.base_string_table.extend(string_table);
    }

    fn accept_fetched_assets(&mut self, asset: Box<dyn Any>) {
        let string_table: Box<HashMap<LineId, String>> = asset.downcast().unwrap();
        self.translation_string_table.replace(*string_table);
    }

    fn fetch_assets(&self) -> Box<dyn Fn(&World) -> Option<Box<dyn Any + 'static>> + '_> {
        let Some(handle) = self.strings_file_handle.as_ref() else {
            return Box::new(|_| None);
        };
        if self.asset_server.get_load_state(handle) != LoadState::Loaded {
            return Box::new(|_| None);
        }
        let has_no_translation_yet = self.translation_string_table.is_none();
        Box::new(move |world| {
            let asset_events = world
                .get_resource::<Events<AssetEvent<StringsFile>>>()
                .unwrap();
            let has_changed = || {
                asset_events
                    .get_reader()
                    .iter(&asset_events)
                    .filter_map(|event| {
                        if let AssetEvent::Modified { handle } = event {
                            Some(handle)
                        } else {
                            None
                        }
                    })
                    .any(|h| h == handle)
            };
            if has_no_translation_yet || has_changed() {
                let strings_file = world.resource::<Assets<StringsFile>>().get(handle).unwrap();
                let expected_language = self.language.as_ref().unwrap();
                if let Some(record) = strings_file.get_offending_language(expected_language) {
                    let path = self.asset_server.get_handle_path(handle).unwrap();
                    panic!("Expected strings file at {path} to only contain language {expected_language}, but its entry with id \"{id}\" is for language {actual_language}.",
                           path = path.path().display(),
                           id = record.id,
                           actual_language = record.language,
                    );
                }
                let string_table: HashMap<LineId, String> = strings_file
                    .0
                    .iter()
                    .map(|(id, record)| (id.clone(), record.text.clone()))
                    .collect();
                Some(Box::new(string_table))
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineAssets(HashSet<HandleUntyped>);
impl LineAssets {
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<HashSet<HandleUntyped>> for LineAssets {
    fn from(h: HashSet<HandleUntyped>) -> Self {
        Self(h)
    }
}

impl IntoIterator for LineAssets {
    type Item = <HashSet<HandleUntyped> as IntoIterator>::Item;
    type IntoIter = <HashSet<HandleUntyped> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<HandleUntyped> for LineAssets {
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
            .map(|s| s.as_ref().trim_start_matches(".").to_owned())
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
                if let Some(localization) = localizations.translation(language) {
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
                if let Some(localization) = localizations.translation(language) {
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

#[derive(Debug, Clone)]
pub struct SharedTextProvider(Arc<RwLock<dyn TextProvider>>);

impl SharedTextProvider {
    pub fn new(text_provider: impl TextProvider + 'static) -> Self {
        Self(Arc::new(RwLock::new(text_provider)))
    }
}

impl TextProvider for SharedTextProvider {
    fn set_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.0.write().unwrap().set_base_string_table(string_table)
    }

    fn extend_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>) {
        self.0
            .write()
            .unwrap()
            .extend_base_string_table(string_table)
    }

    fn accept_fetched_assets(&mut self, asset: Box<dyn Any>) {
        self.0.write().unwrap().accept_fetched_assets(asset)
    }

    fn fetch_assets(&self) -> Box<dyn Fn(&World) -> Option<Box<dyn Any + 'static>> + '_> {
        let clone = self.clone();
        Box::new(move |world| clone.0.read().unwrap().fetch_assets()(world))
    }
}

impl UnderlyingTextProvider for SharedTextProvider {
    fn accept_line_hints(&mut self, line_ids: &[LineId]) {
        self.0.write().unwrap().accept_line_hints(line_ids)
    }

    fn get_text(&self, id: &LineId) -> Option<String> {
        self.0.read().unwrap().get_text(id)
    }

    fn set_language(&mut self, language: Option<Language>) {
        self.0.write().unwrap().set_language(language)
    }

    fn get_language(&self) -> Option<Language> {
        self.0.read().unwrap().get_language()
    }

    fn are_lines_available(&self) -> bool {
        self.0.read().unwrap().are_lines_available()
    }
}

pub(crate) fn fetch_resources(world: &mut World) {
    let dialogue_runner_entities: Vec<_> = world
        .iter_entities()
        .map(|entity| entity.id())
        .filter(|entity| world.get::<DialogueRunner>(*entity).is_some())
        .collect();
    for entity in dialogue_runner_entities {
        let assets = {
            let dialogue_runner = world.get::<DialogueRunner>(entity).unwrap();
            let fetch_assets = dialogue_runner.text_provider().fetch_assets();
            fetch_assets(world)
        };
        if let Some(assets) = assets {
            let mut dialogue_runner = world.get_mut::<DialogueRunner>(entity).unwrap();
            dialogue_runner
                .text_provider_mut()
                .accept_fetched_assets(assets)
        }
    }
}
