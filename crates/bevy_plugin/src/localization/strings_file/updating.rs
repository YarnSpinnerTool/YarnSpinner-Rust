use crate::localization::line_id_generation::LineIdUpdateSystemSet;
use crate::localization::strings_file::{LanguagesToStringsFiles, StringsFile};
use crate::prelude::*;
use bevy::prelude::*;
use std::iter;

pub(crate) fn strings_file_updating_plugin(app: &mut App) {
    app.add_system(
        update_strings_file_on_yarn_file_change
            .pipe(panic_on_err)
            .before(LineIdUpdateSystemSet)
            .run_if(resource_exists::<Localizations>()),
    );
}

fn update_strings_file_on_yarn_file_change(
    mut events: EventReader<AssetEvent<YarnFile>>,
    yarn_files: Res<Assets<YarnFile>>,
    languages_to_strings_files: Res<LanguagesToStringsFiles>,
    mut strings_files: ResMut<Assets<StringsFile>>,
    asset_server: Res<AssetServer>,
    localizations: Res<Localizations>,
) -> SystemResult {
    for event in events.iter() {
        let (handle, reason) = match event {
            AssetEvent::Created { handle } => (handle, "newly added"),
            AssetEvent::Modified { handle } => (handle, "modified"),
            AssetEvent::Removed { .. } => continue,
        };
        for (language, strings_file_handle) in languages_to_strings_files.0.iter() {
            let Some(strings_file) = strings_files.get_mut(strings_file_handle) else {
                continue;
            };
            let yarn_file = yarn_files.get(handle).unwrap();
            let strings_file_path = localizations.strings_file_path(language.clone()).unwrap();
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
                    if localizations.file_generation_mode != FileGenerationMode::Development {
                        warn!(
                            "Tried to update \"{}\" (lang: {language}) because \"{}\" was {reason}, but couldn't because: {e}",
                            strings_file_path.display(),
                            yarn_file_path.display(),
                        );
                    }
                    continue;
                }
            };
            strings_file.update_file(new_strings_file)?;
            strings_file.write_asset(&asset_server, strings_file_path)?;

            info!(
                "Updated \"{}\" (lang: {language}) because \"{}\" was {reason}.",
                strings_file_path.display(),
                yarn_file_path.display(),
            );
        }
    }
    Ok(())
}
