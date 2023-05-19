use crate::localization::strings_file::{LanguagesToStringsFiles, StringsFile};
use crate::prelude::*;
use anyhow::Context;
use bevy::prelude::*;

pub(crate) fn strings_file_updating_plugin(app: &mut App) {
    app.add_system(
        update_strings_file_on_yarn_file_change
            .pipe(panic_on_err)
            .run_if(resource_exists::<Localizations>()),
    );
}

fn update_strings_file_on_yarn_file_change(
    mut events: EventReader<AssetEvent<YarnFile>>,
    yarn_files: Res<Assets<YarnFile>>,
    mut languages_to_strings_files: ResMut<LanguagesToStringsFiles>,
    mut strings_files: ResMut<Assets<StringsFile>>,
    asset_server: Res<AssetServer>,
    localizations: Res<Localizations>,
) -> SystemResult {
    for event in events.iter() {
        if let AssetEvent::Created { .. } | AssetEvent::Modified { .. } = event {
            let mut outdated_unloaded_languages = Vec::new();
            for (language, strings_file_handle) in languages_to_strings_files.0.iter() {
                let yarn_files = yarn_files.iter().map(|(_, yarn_file)| yarn_file);
                let strings_file = StringsFile::from_yarn_files(language.clone(), yarn_files);
                let path = localizations.get_strings_file(language.clone()).unwrap();
                strings_file.write_asset(&asset_server, path)?;
                if let Some(outdated_strings_file) = strings_files.get_mut(strings_file_handle) {
                    *outdated_strings_file = strings_file;
                } else {
                    outdated_unloaded_languages.push((language.clone(), path.to_path_buf()));
                }
            }
            for (language, path) in outdated_unloaded_languages {
                languages_to_strings_files
                    .0
                    .insert(language, asset_server.load(path));
            }
        }
    }
    Ok(())
}
