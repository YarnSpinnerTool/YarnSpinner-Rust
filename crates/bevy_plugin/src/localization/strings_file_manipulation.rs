use crate::localization::strings_file_asset::{StringsFile, StringsFileRecord};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub(crate) fn strings_file_manipulation_plugin(app: &mut App) {
    app.init_resource::<StringsFiles>().add_system(
        create_strings_files
            .pipe(panic_on_err)
            .run_if(resource_changed::<Localizations>()),
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
struct StringsFiles(HashMap<Language, Handle<StringsFile>>);

fn create_strings_files(
    mut commands: Commands,
    localizations: Res<Localizations>,
    asset_server: Res<AssetServer>,
    strings_file_assets: Res<Assets<StringsFile>>,
    strings_files: Res<StringsFiles>,
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
            asset_server.load(path);
            todo!()
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
                    lhs_file_name.cmp(lhs_file_name).then(
                        lhs_string_info
                            .line_number
                            .cmp(&rhs_string_info.line_number),
                    )
                },
            );
            let strings_file_records =
                yarn_files
                    .into_iter()
                    .map(|(line_id, string_info, file_name)| StringsFileRecord {
                        language: localization.language.clone(),
                        id: line_id.clone(),
                        text: string_info.text.clone(),
                        file: file_name.to_string(),
                        node: string_info.node_name.clone(),
                        line_number: string_info.line_number,
                        lock: todo!(),
                        comment: todo!(),
                    });
            todo!()
        };
    }
    Ok(())
}
