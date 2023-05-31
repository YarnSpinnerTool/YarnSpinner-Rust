use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) fn strings_file_lints_plugin(app: &mut App) {
    app.add_system(
        check_validity_of_referenced_files.run_if(
            in_development
                .and_then(resource_exists::<YarnProject>())
                .and_then(on_event::<AssetEvent<StringsFile>>()),
        ),
    );
}

fn check_validity_of_referenced_files(
    mut events: EventReader<AssetEvent<StringsFile>>,
    strings_files: Res<Assets<StringsFile>>,
    project: Res<YarnProject>,
    asset_server: Res<AssetServer>,
) {
    let expected_file_names: HashSet<_> = project
        .compilation
        .string_table
        .values()
        .map(|string_info| string_info.file_name.as_str())
        .collect();
    for event in events.iter() {
        let AssetEvent::Created { handle } = event else {
            continue;
        };
        let source = asset_server
            .get_handle_path(handle)
            .map(|asset_path| format!("at {}", asset_path.path().display()))
            .unwrap_or_else(|| "created at runtime".to_owned());
        let strings_file = strings_files.get(handle).unwrap();
        let actual_file_names: HashSet<_> = strings_file
            .records()
            .map(|rec| rec.file.as_str())
            .collect();
        let superfluous_file_names = actual_file_names
            .difference(&expected_file_names)
            .map(|name| name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");
        warn!(
            "Strings file {source} contains the following strings for yarn files were not found in the project: {}. \
            Either you forgot to add these files to the project or the strings belonged to files that were deleted. \
            You may want to delete these entries from the strings file manually. Yarn Slinger will not do this for you because it may lead to loss of work.",
            superfluous_file_names
        );
    }
}
