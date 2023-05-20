use crate::filesystem_events::CreateMissingStringsFilesEvent;
use crate::localization::line_id_generation::LineIdUpdateSystemSet;
use crate::localization::strings_file::creation::CreateMissingStringsFilesSystemSet;
use crate::prelude::*;
use anyhow::bail;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use std::iter;

pub(crate) fn strings_file_updating_plugin(app: &mut App) {
    app.add_event::<UpdateAllStringsFilesForYarnFileEvent>()
        .add_systems(
            (
                send_update_events_on_yarn_file_changes
                    .run_if(in_development.and_then(resource_exists::<LoadedYarnFiles>())),
                update_all_strings_files_for_yarn_file
                    .pipe(panic_on_err)
                    .after(LineIdUpdateSystemSet)
                    .after(CreateMissingStringsFilesSystemSet)
                    .run_if(
                        resource_exists::<Localizations>()
                            .and_then(in_development)
                            .and_then(resource_exists::<LoadedYarnFiles>()),
                    ),
            )
                .chain(),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect)]
#[reflect(Debug, Hash, Default, PartialEq)]
pub struct UpdateAllStringsFilesForYarnFileEvent(pub Handle<YarnFile>);

fn send_update_events_on_yarn_file_changes(
    mut events: EventReader<AssetEvent<YarnFile>>,
    loaded_yarn_files: Res<LoadedYarnFiles>,
    mut update_writer: EventWriter<UpdateAllStringsFilesForYarnFileEvent>,
) {
    for event in events.iter() {
        let (AssetEvent::Created { handle } | AssetEvent::Modified { handle }) = event else {
            continue;
        };
        if loaded_yarn_files.0.contains(handle) {
            update_writer.send(UpdateAllStringsFilesForYarnFileEvent(handle.clone()));
        }
    }
}

fn update_all_strings_files_for_yarn_file(
    mut events: EventReader<UpdateAllStringsFilesForYarnFileEvent>,
    mut missing_writer: EventWriter<CreateMissingStringsFilesEvent>,
    yarn_files: Res<Assets<YarnFile>>,
    mut strings_files: ResMut<Assets<StringsFile>>,
    asset_server: Res<AssetServer>,
    localizations: Res<Localizations>,
    mut languages_to_update: Local<HashMap<Language, Handle<StringsFile>>>,
    current_strings_file: Res<CurrentStringsFile>,
    loaded_yarn_files: Res<LoadedYarnFiles>,
) -> SystemResult {
    if !events.is_empty() {
        let supported_languages: HashSet<_> = localizations
            .translations
            .iter()
            .map(|t| t.language.clone())
            .collect();
        let updated_languages: HashSet<_> = languages_to_update.keys().cloned().collect();
        let languages_to_remove = updated_languages.difference(&supported_languages);
        let languages_to_add = supported_languages.difference(&updated_languages);
        for language in languages_to_remove {
            languages_to_update.remove(language);
        }
        for language in languages_to_add {
            let strings_file_path = localizations.strings_file_path(language).unwrap();
            let handle = if let Some(handle) = &current_strings_file.0 {
                handle.clone()
            } else if asset_server.asset_io().is_file(strings_file_path) {
                asset_server.load(strings_file_path)
            } else {
                missing_writer.send(CreateMissingStringsFilesEvent);
                return Ok(());
            };
            languages_to_update.insert(language.clone(), handle);
        }
        for handle in languages_to_update.values() {
            if asset_server.get_load_state(handle) != LoadState::Loaded {
                return Ok(());
            }
        }
    }
    for handle in events.iter().map(|e| &e.0) {
        if !loaded_yarn_files.0.contains(handle) {
            bail!("Sent `UpdateAllStringsFilesForYarnFileEvent` for a Yarn file that was not passed to the `YarnPlugin`");
        }

        for (language, strings_file_handle) in languages_to_update.drain() {
            let Some(strings_file) = strings_files.get_mut(&strings_file_handle) else {
                continue;
            };
            let strings_file_path = localizations.strings_file_path(&language).unwrap();
            let Some(yarn_file) = yarn_files.get(handle) else {
                bail!(
                    "Tried to update \"{}\" (lang: {language}) because a Yarn file was meant to be changed or loaded, but couldn't because it was actually not loaded.\
                    If you sent out an `UpdateAllStringsFilesForYarnFile`, please make sure that the provided handle is done loading via `AssetServer::get_load_state`.\
                    If you did not send out such an event: this is a bug, please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new",
                    strings_file_path.display(),);
            };
            let yarn_file_path = asset_server
                .get_handle_path(handle)
                .unwrap()
                .path()
                .to_path_buf();

            let new_strings_file = match StringsFile::from_yarn_files(
                language.clone(),
                iter::once(yarn_file),
            ) {
                Ok(new_strings_file) => new_strings_file,
                Err(e) => {
                    if localizations.file_generation_mode == FileGenerationMode::Development {
                        info!("Updating \"{}\" soon (lang: {language}) because \"{}\" was changed or loaded but does not have full line IDs yet.",
                            strings_file_path.display(),
                            yarn_file_path.display(),)
                    } else {
                        warn!(
                            "Tried to update \"{}\" (lang: {language}) because \"{}\" was changed or loaded, but couldn't because: {e}",
                            strings_file_path.display(),
                            yarn_file_path.display(),
                        );
                    }
                    continue;
                }
            };
            if strings_file.update_file(new_strings_file)? {
                strings_file.write_asset(&asset_server, strings_file_path)?;
            }

            info!(
                "Updated \"{}\" (lang: {language}) because \"{}\" was changed or loaded.",
                strings_file_path.display(),
                yarn_file_path.display(),
            );
        }
    }
    Ok(())
}
