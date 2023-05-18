//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Editor/Utility/YarnProjectUtility.cs#L259>
use crate::localization::strings_file::{Lock, StringsFile, StringsFileRecord};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::fs::File;

pub(crate) fn strings_file_creation_plugin(app: &mut App) {
    app.init_resource::<StringsFiles>().add_system(
        create_strings_files
            .pipe(panic_on_err)
            .run_if(resource_exists_and_changed::<Localizations>()),
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
struct StringsFiles(HashMap<Language, Handle<StringsFile>>);

fn create_strings_files(
    localizations: Res<Localizations>,
    asset_server: Res<AssetServer>,
    mut strings_file_assets: ResMut<Assets<StringsFile>>,
    mut strings_files: ResMut<StringsFiles>,
    yarn_files: Res<Assets<YarnFile>>,
) -> SystemResult {
    for localization in &localizations.translations {
        if strings_file_assets
            .iter()
            .any(|(_, strings_file)| strings_file.has_language(&localization.language))
            || strings_files.0.contains_key(&localization.language)
        {
            // Already loaded this language outside of this system.
            continue;
        }
        let path = localization.strings_file.as_path();
        let handle = if asset_server.asset_io().is_file(path) {
            asset_server.load(path)
        } else if localizations.file_generation_mode == FileGenerationMode::Development {
            let mut yarn_files: Vec<(&LineId, &StringInfo, &str)> = yarn_files
                .iter()
                .flat_map(|(_, yarn_file)| {
                    yarn_file
                        .string_table
                        .iter()
                        .map(|(id, line_info)| (id, line_info, yarn_file.file.file_name.as_str()))
                        .collect::<Vec<_>>()
                })
                .collect();
            yarn_files.sort_by(
                |(_, lhs_string_info, lhs_file_name), (_, rhs_string_info, rhs_file_name)| {
                    lhs_file_name.cmp(rhs_file_name).then(
                        lhs_string_info
                            .line_number
                            .cmp(&rhs_string_info.line_number),
                    )
                },
            );
            let strings_file_records: Vec<_> = yarn_files
                .into_iter()
                .map(|(line_id, string_info, file_name)| StringsFileRecord {
                    language: localization.language.clone(),
                    id: line_id.clone(),
                    text: string_info.text.clone(),
                    file: file_name.to_string(),
                    node: string_info.node_name.clone(),
                    line_number: string_info.line_number,
                    lock: Lock::compute_from(&string_info.text),
                    comment: read_comments(&string_info.metadata),
                })
                .collect();
            let path = get_assets_dir_path(&asset_server)?;
            let path = path.as_ref();
            let file = File::create(path).unwrap();
            let mut writer = csv::Writer::from_writer(file);
            for record in strings_file_records.iter() {
                writer.serialize(record)?;
            }
            writer.flush()?;
            let strings_file = StringsFile(strings_file_records);
            info!(
                "Generated strings file \"{}\" for language {}.",
                path.display(),
                localization.language
            );
            strings_file_assets.add(strings_file)
        } else {
            panic!(
                "Can't load strings file \"{}\" because it does not exist on disk, but can't generate it either because the file generation mode is not set to \"Development\".",
                path.display()
            );
        };
        strings_files
            .0
            .insert(localization.language.clone(), handle);
    }
    Ok(())
}

/// Generates a string with the line metadata. This string is intended
/// to be used in the "comment" column of a strings table CSV. Because
/// of this, it will ignore the line ID if it exists (which is also
/// part of the line metadata).
///
/// ## Return value
/// A string prefixed with "Line metadata: ", followed by each
/// piece of metadata separated by whitespace. If no metadata exists or
/// only the line ID is part of the metadata, returns an empty string
/// instead.
fn read_comments(metadata: &[String]) -> String {
    // Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Editor/Importers/YarnProjectImporter.cs#L652>
    let cleaned_metadata: Vec<_> = metadata
        .iter()
        .filter(|metadata| !metadata.starts_with("line:"))
        .cloned()
        .collect();
    if cleaned_metadata.is_empty() {
        String::new()
    } else {
        format!("Line metadata: {}", cleaned_metadata.join(" "))
    }
}
