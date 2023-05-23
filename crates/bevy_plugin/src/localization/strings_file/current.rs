use crate::default_impl::StringTableTextProvider;
use crate::localization::line_id_generation::LineIdUpdateSystemSet;
use crate::localization::strings_file::StringsFile;
use crate::prelude::*;
use crate::project::{CompilationSystemSet, YarnProjectConfigToLoad};
use anyhow::bail;
use bevy::prelude::*;

pub(crate) fn current_strings_file_plugin(app: &mut App) {
    app.register_type::<CurrentStringsFile>()
        .add_event::<UpdateBaseLanguageTextProviderForStringTableEvent>()
        .init_resource::<CurrentStringsFile>()
        .add_systems(
            (
                update_current_strings_file
                    .pipe(panic_on_err)
                    .run_if(resource_exists_and_changed::<YarnProject>()),
                update_base_language_string_provider.run_if(events_in_world::<
                    UpdateBaseLanguageTextProviderForStringTableEvent,
                >()),
                update_translation_string_provider_from_disk.run_if(
                    resource_exists::<YarnProject>()
                        .and_then(events_in_queue::<AssetEvent<StringsFile>>()),
                ),
                update_translation_string_provider_from_loaded_handle
                    .pipe(panic_on_err)
                    .run_if(resource_exists::<YarnProject>()),
            )
                .chain()
                .after(LineIdUpdateSystemSet)
                .after(CompilationSystemSet),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Reflect, FromReflect)]
#[reflect(Debug, Default, PartialEq)]
pub(crate) struct UpdateBaseLanguageTextProviderForStringTableEvent(
    pub std::collections::HashMap<LineId, String>,
);

impl From<&std::collections::HashMap<LineId, StringInfo>>
    for UpdateBaseLanguageTextProviderForStringTableEvent
{
    fn from(map: &std::collections::HashMap<LineId, StringInfo>) -> Self {
        Self(
            map.iter()
                .map(|(k, v)| (k.clone(), v.text.clone()))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct CurrentStringsFile(pub(crate) Option<Handle<StringsFile>>);

fn update_current_strings_file(
    mut current_strings_file: ResMut<CurrentStringsFile>,
    mut project: ResMut<YarnProject>,
    asset_server: Res<AssetServer>,
    mut last_language: Local<Option<Language>>,
) -> SystemResult {
    let Some(language) = project.text_language() else {
        current_strings_file.0 = None;
        return Ok(());
    };
    let Some(localizations) = project.localizations.as_ref() else {
        bail!("Language was set to {language}, but no localizations were configured");
    };
    if localizations.base_language.language == language {
        current_strings_file.0 = None;
        project.set_text_language(None);
        return Ok(());
    }
    if last_language.as_ref() == Some(&language) {
        return Ok(());
    }
    *last_language = Some(language.clone());
    let Some(localization) = localizations.translations.iter().find(|t| t.language == language) else {
        bail!("Language was set to {language}, but no localization for that language was configured");
    };
    let path = localization.strings_file.as_path();
    if !asset_server.asset_io().is_file(path) {
        bail!(
            "Language was set to {language}, but no strings file was found at {path}",
            path = path.display()
        );
    }
    let handle = asset_server.load(path);
    current_strings_file.0 = Some(handle);
    Ok(())
}

fn update_base_language_string_provider(
    mut events: ResMut<Events<UpdateBaseLanguageTextProviderForStringTableEvent>>,
    mut project: Option<ResMut<YarnProject>>,
    mut project_to_load: Option<ResMut<YarnProjectConfigToLoad>>,
) {
    let text_provider = project
        .as_mut()
        .map(|p| &mut p.text_provider)
        .unwrap_or_else(|| {
            project_to_load
                .as_mut()
                .map(|p| p.text_provider.as_mut())
                .unwrap()
                .unwrap()
        });
    let Some(text_provider) = text_provider.downcast_to_string_table_text_provider_mut() else {
        events.clear();
        return;
    };
    for event in events.drain() {
        let string_table = event.0;
        text_provider.extend_base_language(string_table);
    }
}

fn update_translation_string_provider_from_disk(
    mut events: EventReader<AssetEvent<StringsFile>>,
    current_strings_file: Res<CurrentStringsFile>,
    strings_files: Res<Assets<StringsFile>>,
    mut project: ResMut<YarnProject>,
) {
    let Some(language) = project.text_provider.downcast_to_string_table_text_provider().and_then(|p| p.get_language()) else {
        events.clear();
        return;
    };

    let text_provider = project
        .text_provider
        .downcast_to_string_table_text_provider_mut()
        .unwrap();
    for event in events.iter() {
        let (AssetEvent::Created { handle } | AssetEvent::Modified { handle }) = event else {
            continue;
        };
        if Some(handle) != current_strings_file.0.as_ref() {
            continue;
        }
        let strings_file = strings_files.get(handle).unwrap();
        text_provider.extend_translation(language.clone(), strings_file.to_text_table());
    }
}

fn update_translation_string_provider_from_loaded_handle(
    mut project: ResMut<YarnProject>,
    strings_files: Res<Assets<StringsFile>>,
    current_strings_file: Res<CurrentStringsFile>,
    mut dirty: Local<bool>,
) -> SystemResult {
    if current_strings_file.is_changed() {
        *dirty = true;
    }
    if !*dirty {
        return Ok(());
    }
    let Some(handle) = current_strings_file.0.as_ref() else {
        *dirty = false;
        return Ok(());
    };
    let Some(language) = project.text_language() else {
        *dirty = false;
        return Ok(());
    };
    let Some(strings_file) = strings_files.get(handle) else {
        return Ok(());
    };

    let Some(text_provider) = project.text_provider.downcast_to_string_table_text_provider_mut() else {
        *dirty = false;
        return Ok(());
    };

    text_provider.extend_translation(language, strings_file.to_text_table());

    *dirty = false;
    Ok(())
}

trait ToTextTable {
    fn to_text_table(&self) -> std::collections::HashMap<LineId, String>;
}

impl ToTextTable for std::collections::HashMap<LineId, StringInfo> {
    fn to_text_table(&self) -> std::collections::HashMap<LineId, String> {
        self.iter()
            .map(|(id, line)| (id.clone(), line.text.clone()))
            .collect()
    }
}

impl ToTextTable for StringsFile {
    fn to_text_table(&self) -> std::collections::HashMap<LineId, String> {
        self.0
            .iter()
            .map(|(id, line)| (id.clone(), line.text.clone()))
            .collect()
    }
}

trait TextProviderExt {
    fn downcast_to_string_table_text_provider(&self) -> Option<&StringTableTextProvider>;
    fn downcast_to_string_table_text_provider_mut(
        &mut self,
    ) -> Option<&mut StringTableTextProvider>;
}

impl TextProviderExt for Box<dyn TextProvider> {
    fn downcast_to_string_table_text_provider(&self) -> Option<&StringTableTextProvider> {
        self.as_any().downcast_ref::<StringTableTextProvider>()
    }

    fn downcast_to_string_table_text_provider_mut(
        &mut self,
    ) -> Option<&mut StringTableTextProvider> {
        self.as_any_mut().downcast_mut::<StringTableTextProvider>()
    }
}
