//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Editor/Utility/YarnProjectUtility.cs#L259>
use crate::localization::StringsFile;
use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;

pub(crate) fn strings_file_creation_plugin(app: &mut App) {
    app.add_systems(
        (
            create_strings_files
                .pipe(panic_on_err)
                .run_if(resource_exists_and_changed::<Localizations>()),
            ensure_right_language.pipe(panic_on_err),
        )
            .chain(),
    );
}

fn ensure_right_language(
    mut events: EventReader<AssetEvent<StringsFile>>,
    languages_to_strings_files: Res<LanguagesToStringsFiles>,
    assets: Res<Assets<StringsFile>>,
) -> SystemResult {
    for event in events.iter() {
        if let AssetEvent::Created { handle } | AssetEvent::Modified { handle } = event {
            let strings_file = assets.get(handle).unwrap();
            if let Some(expected_language) = languages_to_strings_files.get_language(handle) {
                if let Some(language) = strings_file.language() {
                    if language != expected_language {
                        bail!(
                                "The language the strings registered for language \"{expected_language}\" \
                                actually contains the language \"{language}\""
                            );
                    }
                }
            }
        }
    }
    Ok(())
}

fn create_strings_files(
    localizations: Res<Localizations>,
    asset_server: Res<AssetServer>,
    mut languages_to_strings_files: ResMut<LanguagesToStringsFiles>,
    yarn_files: Res<Assets<YarnFile>>,
) -> SystemResult {
    languages_to_strings_files
        .0
        .retain(|lang, _| localizations.supports_translation(lang));
    for localization in &localizations.translations {
        if languages_to_strings_files
            .0
            .contains_key(&localization.language)
        {
            continue;
        }
        let path = localization.strings_file.as_path();
        let handle = if asset_server.asset_io().is_file(path) {
            asset_server.load(path)
        } else if localizations.file_generation_mode == FileGenerationMode::Development {
            let yarn_files = yarn_files.iter().map(|(_, yarn_file)| yarn_file);
            let (strings_file, error) =
                StringsFile::from_yarn_files(localization.language.clone(), yarn_files)
                    .map(|strings_file| (strings_file, None))
                    .unwrap_or_else(|e| (StringsFile::default(), Some(e)));

            strings_file.write_asset(&asset_server, path)?;
            match error {
                Some(e)
                    if localizations.file_generation_mode != FileGenerationMode::Development =>
                {
                    warn!(
                        "Generated \"{}\" (lang: {}), but it is empty because: {e}",
                        path.display(),
                        localization.language
                    )
                }
                _ => info!(
                    "Generated \"{}\" (lang: {}).",
                    path.display(),
                    localization.language
                ),
            };

            asset_server.load(path)
        } else {
            bail!(
                "Can't load strings file \"{}\" because it does not exist on disk, \
                but can't generate it either because the file generation mode is not set to \"Development\".",
                path.display());
        };
        languages_to_strings_files
            .0
            .insert(localization.language.clone(), handle);
    }
    Ok(())
}
