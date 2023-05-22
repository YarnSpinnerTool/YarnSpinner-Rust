use crate::filesystem_events::CreateMissingStringsFilesEvent;
use crate::localization::line_id_generation::LineIdUpdateSystemSet;
use crate::localization::strings_file::creation::CreateMissingStringsFilesSystemSet;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub(crate) fn strings_file_updating_plugin(app: &mut App) {
    app.add_event::<UpdateAllStringsFilesForStringTableEvent>()
        .add_systems(
            (update_all_strings_files_for_string_table
                .pipe(panic_on_err)
                .after(LineIdUpdateSystemSet)
                .after(CreateMissingStringsFilesSystemSet)
                .run_if(
                    in_development
                        .and_then(resource_exists::<YarnProject>())
                        .and_then(on_event::<UpdateAllStringsFilesForStringTableEvent>()),
                ),)
                .chain(),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Reflect, FromReflect)]
#[reflect(Debug, Default, PartialEq)]
pub struct UpdateAllStringsFilesForStringTableEvent(
    pub std::collections::HashMap<LineId, StringInfo>,
);

fn update_all_strings_files_for_string_table(
    mut events: EventReader<UpdateAllStringsFilesForStringTableEvent>,
    mut missing_writer: EventWriter<CreateMissingStringsFilesEvent>,
    mut strings_files: ResMut<Assets<StringsFile>>,
    asset_server: Res<AssetServer>,
    project: Res<YarnProject>,
) -> SystemResult {
    let localizations = project.localizations.as_ref().unwrap();
    if localizations.translations.is_empty() {
        events.clear();
        return Ok(());
    }

    let mut languages_to_handles = HashMap::new();
    for localization in &localizations.translations {
        let language = &localization.language;
        let path = localization.strings_file.as_path();
        let handle = if asset_server.asset_io().is_file(path) {
            asset_server.load(path)
        } else {
            missing_writer.send(CreateMissingStringsFilesEvent);
            return Ok(());
        };
        if !strings_files.contains(&handle) {
            return Ok(());
        }
        languages_to_handles.insert(language.clone(), handle);
    }

    let mut dirty_paths = HashSet::new();

    for string_table in events.iter().map(|e| &e.0) {
        let file_names: HashSet<_> = string_table
            .values()
            .map(|s| s.file_name.as_str())
            .collect();
        let file_names = file_names.into_iter().collect::<Vec<_>>().join(", ");
        for (language, strings_file_handle) in languages_to_handles.clone() {
            let Some(strings_file) = strings_files.get_mut(&strings_file_handle) else {
                continue;
            };
            let strings_file_path = localizations.strings_file_path(&language).unwrap();

            let new_strings_file = match StringsFile::from_string_table(
                language.clone(),
                string_table,
            ) {
                Ok(new_strings_file) => new_strings_file,
                Err(e) => {
                    if localizations.file_generation_mode == FileGenerationMode::Development {
                        info!("Updating \"{}\" soon (lang: {language}) because the following yarn files were changed or loaded but do not have full line IDs yet: {file_names}",
                            strings_file_path.display())
                    } else {
                        warn!(
                            "Tried to update \"{}\" (lang: {language}) because the following yarn files were changed or loaded: {file_names}, but couldn't because: {e}",
                            strings_file_path.display(),
                        );
                    }
                    continue;
                }
            };
            if strings_file.update_file(new_strings_file)? {
                dirty_paths.insert((strings_file_handle, strings_file_path));

                info!(
                    "Updated \"{}\" (lang: {language}) because the following yarn files were changed or loaded: {file_names}",
                    strings_file_path.display(),
                );
            }
        }
    }
    for (handle, path) in &dirty_paths {
        let strings_file = strings_files.get(handle).unwrap();
        strings_file.write_asset(&asset_server, path)?;
    }
    Ok(())
}
