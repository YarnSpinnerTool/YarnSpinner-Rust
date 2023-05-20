//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Editor/Utility/YarnProjectUtility.cs#L259>
use crate::localization::StringsFile;
use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;

pub(crate) fn strings_file_creation_plugin(app: &mut App) {
    app.add_event::<CreateMissingStringsFilesEvent>()
        .add_systems(
            (
                create_strings_files.pipe(panic_on_err).run_if(
                    resource_exists::<Localizations>()
                        .and_then(on_event::<CreateMissingStringsFilesEvent>())
                        .or_else(resource_changed::<Localizations>()),
                ),
                ensure_right_language.pipe(panic_on_err),
            )
                .chain(),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect)]
#[reflect(Debug, Hash, Default, PartialEq)]
pub struct CreateMissingStringsFilesEvent;

fn ensure_right_language(
    current_strings_file: Res<CurrentStringsFile>,
    current_language: Res<CurrentLanguage>,
    assets: Res<Assets<StringsFile>>,
    mut done: Local<bool>,
) -> SystemResult {
    if current_strings_file.is_changed() {
        *done = false;
    }
    if current_strings_file.0.is_none() {
        *done = true;
    }
    if *done {
        return Ok(());
    }

    let handle = current_strings_file.0.as_ref().unwrap();
    let Some(strings_file) = assets.get(handle) else {
        return Ok(());
    };
    let expected_language = &current_language.0;
    if let Some(language) = strings_file.language() {
        if language != expected_language {
            bail!(
                "The language the strings registered for language \"{expected_language}\" \
                            actually contains the language \"{language}\""
            );
        }
    }
    *done = true;
    Ok(())
}

fn create_strings_files(
    localizations: Res<Localizations>,
    asset_server: Res<AssetServer>,
    yarn_files: Res<Assets<YarnFile>>,
) -> SystemResult {
    for localization in &localizations.translations {
        let path = localization.strings_file.as_path();
        if asset_server.asset_io().is_file(path) {
            return Ok(());
        }
        if localizations.file_generation_mode != FileGenerationMode::Development {
            bail!(
                "Can't load strings file \"{}\" because it does not exist on disk, \
                but can't generate it either because the file generation mode is not set to \"Development\".",
                path.display());
        };
        let yarn_files = yarn_files.iter().map(|(_, yarn_file)| yarn_file);
        let (strings_file, error) =
            StringsFile::from_yarn_files(localization.language.clone(), yarn_files)
                .map(|strings_file| (strings_file, None))
                .unwrap_or_else(|e| (StringsFile::default(), Some(e)));

        strings_file.write_asset(&asset_server, path)?;
        match error {
            Some(e) if localizations.file_generation_mode != FileGenerationMode::Development => {
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
    }
    Ok(())
}
