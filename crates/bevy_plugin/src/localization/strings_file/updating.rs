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
    languages_to_strings_files: Res<LanguagesToStringsFiles>,
    mut strings_files: ResMut<Assets<StringsFile>>,
) -> SystemResult {
    for event in events.iter() {
        if let AssetEvent::Created { handle } | AssetEvent::Modified { handle } = event {
            let yarn_file = yarn_files.get(handle).unwrap();
            for (language, strings_file_handle) in languages_to_strings_files.0.iter() {
                let strings_file = strings_files.get_mut(strings_file_handle).unwrap();
            }
        }
    }
    Ok(())
}
