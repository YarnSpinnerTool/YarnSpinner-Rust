use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use utils::prelude::*;
use yarnspinner::core::CURRENT_PROJECT_FILE_VERSION;

mod utils;

#[derive(Resource)]
struct ProjectHandle(Handle<YarnProjectFile>);

#[test]
fn loads_yarnproject_as_asset() {
    let mut app = App::new();
    app.setup_default_plugins()
        .add_plugins(YarnSpinnerPlugin::deferred())
        .add_systems(
            Startup,
            |mut commands: Commands, asset_server: Res<AssetServer>| {
                commands.insert_resource(ProjectHandle(asset_server.load("lines.yarnproject")));
            },
        )
        .add_systems(Update, check_project);
    assert!(app.run().is_success());
}

fn check_project(
    handle: Res<ProjectHandle>,
    asset_server: Res<AssetServer>,
    projects: Res<Assets<YarnProjectFile>>,
    mut commands: Commands,
) {
    let state = asset_server.load_state(&handle.0);
    if state.is_loading() {
        return;
    }
    cmd_assert!(commands, state.is_loaded(), "Failed to load: {state:?}");
    let project = projects.get(&handle.0).unwrap().project();
    cmd_assert!(
        commands,
        project.project_file_version == CURRENT_PROJECT_FILE_VERSION,
        "Wrong project file version"
    );
    cmd_assert!(
        commands,
        project.source_file_patterns == ["lines.yarn"],
        "Wrong source patterns"
    );
    cmd_assert!(
        commands,
        project.base_language == "en",
        "Wrong base language"
    );

    commands.write_message(AppExit::Success);
}
