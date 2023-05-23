use crate::default_impl::StringTableTextProvider;
use crate::localization::strings_file::StringsFile;
use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;

pub(crate) fn current_strings_file_plugin(app: &mut App) {
    app.register_type::<CurrentStringsFile>()
        .init_resource::<CurrentStringsFile>()
        .add_systems((
            update_current_strings_file
                .pipe(panic_on_err)
                .run_if(resource_exists_and_changed::<YarnProject>()),
            load_project_text_provider.run_if(resource_exists::<YarnProject>()),
        ));
}

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

fn load_project_text_provider(
    strings_files: Res<Assets<StringsFile>>,
    current_strings_file: Res<CurrentStringsFile>,
    mut project: ResMut<YarnProject>,
    mut dirty: Local<bool>,
) {
    if current_strings_file.is_changed() {
        *dirty = true;
    }
    if !*dirty {
        return;
    }

    let Some(handle) = current_strings_file.0.as_ref() else {
        let text_table = project.compilation.string_table.to_text_table();

        let Some(text_provider) = project.text_provider.downcast_to_string_table_text_provider() else {
            *dirty = false;
            return;
        };
        text_provider.set_base_language(text_table);
        *dirty = false;
        return;
    };

    let Some(strings_file) = strings_files.get(handle) else {
        return;
    };
    let Some(text_provider) = project.text_provider.downcast_to_string_table_text_provider() else {
        *dirty = false;
        return;
    };

    let Some(language) = text_provider.get_language_code() else {
        *dirty = false;
        return;
    };
    let string_table = strings_file.to_text_table();
    text_provider.set_translation(language, string_table);
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
