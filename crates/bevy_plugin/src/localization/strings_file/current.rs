use crate::localization::strings_file::StringsFile;
use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;

pub(crate) fn current_strings_file_plugin(app: &mut App) {
    app.register_type::<CurrentStringsFile>()
        .init_resource::<CurrentStringsFile>()
        .add_system(
            update_current_strings_file
                .pipe(panic_on_err)
                .run_if(resource_exists_and_changed::<YarnProject>()),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct CurrentStringsFile(pub(crate) Option<Handle<StringsFile>>);

fn update_current_strings_file(
    mut current_strings_file: ResMut<CurrentStringsFile>,
    mut project: ResMut<YarnProject>,
    asset_server: Res<AssetServer>,
) -> SystemResult {
    let (Some(localizations), Some(language)) = (
        project.localizations.as_ref(),
        project.text_provider.get_language_code(),
    ) else {
        current_strings_file.0 = None;
        return Ok(());
    };
    let Some(localization) = localizations.translations.iter().find(|t| t.language == language) else {
        bail!("Language was set to {language}, but no localization for that language was configured");
    };
    let path = &localization.strings_file;
    let handle = asset_server.load(path.as_path());
    current_strings_file.0 = Some(handle);
    Ok(())
}
