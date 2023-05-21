//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Editor/Utility/YarnProjectUtility.cs#L259>
use crate::localization::StringsFile;
use crate::prelude::*;
use crate::project::YarnCompilation;
use anyhow::bail;
use bevy::prelude::*;

pub(crate) fn strings_file_creation_plugin(app: &mut App) {
    app.add_event::<CreateMissingStringsFilesEvent>()
        .add_systems(
            (
                create_strings_files
                    .pipe(panic_on_err)
                    .in_set(CreateMissingStringsFilesSystemSet)
                    .run_if(
                        in_development
                            .and_then(resource_exists::<Localizations>())
                            .and_then(resource_exists::<YarnCompilation>())
                            .and_then(
                                resource_changed::<YarnCompilation>()
                                    .or_else(resource_changed::<Localizations>())
                                    .or_else(on_event::<CreateMissingStringsFilesEvent>()),
                            ),
                    ),
                ensure_right_language
                    .pipe(panic_on_err)
                    .run_if(resource_exists::<CurrentLanguage>()),
            )
                .chain(),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Reflect, FromReflect)]
#[reflect(Debug, Hash, Default, PartialEq)]
pub struct CreateMissingStringsFilesEvent;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) struct CreateMissingStringsFilesSystemSet;

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
    compilation: Res<YarnCompilation>,
) -> SystemResult {
    for localization in &localizations.translations {
        let path = localization.strings_file.as_path();
        if asset_server.asset_io().is_file(path) {
            return Ok(());
        }
        let strings_file = StringsFile::from_string_table(
            localization.language.clone(),
            &compilation.0.string_table,
        )
        .unwrap_or_default();

        strings_file.write_asset(&asset_server, path)?;
        info!(
            "Generated \"{}\" (lang: {}).",
            path.display(),
            localization.language
        );
    }
    Ok(())
}
