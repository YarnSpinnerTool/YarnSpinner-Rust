use crate::default_impl::StringTableTextProvider;
use crate::events::UpdateAllStringsFilesForStringTableEvent;
use crate::localization::strings_file::StringsFile;
use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;

pub(crate) fn current_strings_file_plugin(app: &mut App) {
    app.register_type::<CurrentStringsFile>()
        .add_event::<UpdateTextProviderForStringTableEvent>()
        .init_resource::<CurrentStringsFile>()
        .add_systems(
            (
                update_current_strings_file
                    .pipe(panic_on_err)
                    .run_if(resource_exists_and_changed::<YarnProject>()),
                update_base_language_string_provider.run_if(
                    resource_exists::<YarnProject>()
                        .and_then(on_event::<UpdateTextProviderForStringTableEvent>()),
                ),
                update_translation_string_provider_from_disk.run_if(
                    resource_exists::<YarnProject>()
                        .and_then(on_event::<AssetEvent<StringsFile>>()),
                ),
            )
                .chain(),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Reflect, FromReflect)]
#[reflect(Debug, Default, PartialEq)]
pub(crate) struct UpdateTextProviderForStringTableEvent(
    pub std::collections::HashMap<LineId, String>,
);

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct CurrentStringsFile(pub(crate) Option<Handle<StringsFile>>);

fn update_current_strings_file(
    mut current_strings_file: ResMut<CurrentStringsFile>,
    project: Res<YarnProject>,
    asset_server: Res<AssetServer>,
    mut last_language: Local<Option<Language>>,
) -> SystemResult {
    let (Some(localizations), Some(language)) = (
        project.localizations.as_ref(),
        project.text_provider.get_language_code(),
    ) else {
        current_strings_file.0 = None;
        return Ok(());
    };
    if localizations.base_language.language == language {
        current_strings_file.0 = None;
        return Ok(());
    }
    if last_language.as_ref() == Some(&language) {
        return Ok(());
    }
    *last_language = Some(language.clone());
    let Some(localization) = localizations.translations.iter().find(|t| t.language == language) else {
        bail!("Language was set to {language}, but no localization for that language was configured");
    };
    let path = &localization.strings_file;
    let handle = asset_server.load(path.as_path());
    current_strings_file.0 = Some(handle);
    Ok(())
}

fn update_base_language_string_provider(
    mut events: EventReader<UpdateTextProviderForStringTableEvent>,
    mut project: ResMut<YarnProject>,
) {
    let Some(text_provider) = project.text_provider.downcast_to_string_table_text_provider() else {
        events.clear();
        return;
    };
    for event in events.iter() {
        let string_table = &event.0;
        text_provider.extend_base_language(string_table.clone());
    }
}

fn update_translation_string_provider_from_disk(
    mut events: EventReader<AssetEvent<StringsFile>>,
    current_strings_file: Res<CurrentStringsFile>,
    strings_files: Res<Assets<StringsFile>>,
    mut project: ResMut<YarnProject>,
) {
    let Some(text_provider) = project.text_provider.downcast_to_string_table_text_provider() else {
        events.clear();
        return;
    };
    let Some(language) = project.text_provider.get_language_code() else {
        events.clear();
        return;
    };
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
    fn downcast_to_string_table_text_provider(&mut self) -> Option<&mut StringTableTextProvider>;
}

impl TextProviderExt for Box<dyn TextProvider> {
    fn downcast_to_string_table_text_provider(&mut self) -> Option<&mut StringTableTextProvider> {
        self.as_any_mut().downcast_mut::<StringTableTextProvider>()
    }
}
