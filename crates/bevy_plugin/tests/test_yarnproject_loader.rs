use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use utils::prelude::*;
use yarnspinner::compiler::CURRENT_PROJECT_FILE_VERSION;

mod utils;

#[test]
fn loads_yarnproject_as_asset() {
    let mut app = App::new();
    app.setup_default_plugins()
        .add_plugins(YarnSpinnerPlugin::deferred());
    let handle = load_yarnproject(&mut app, "lines.yarnproject");

    assert!(settle(&mut app, &handle).is_loaded());
    let project = app
        .world()
        .resource::<Assets<YarnProjectFile>>()
        .get(&handle)
        .unwrap()
        .project();
    assert_eq!(project.project_file_version, CURRENT_PROJECT_FILE_VERSION);
    assert_eq!(project.source_file_patterns, ["lines.yarn"]);
    assert_eq!(project.base_language, "en");
}

fn load_yarnproject(app: &mut App, path: &'static str) -> Handle<YarnProjectFile> {
    app.world().resource::<AssetServer>().load(path)
}

/// Updates the app until `handle` stops loading, then returns its final state.
fn settle(app: &mut App, handle: &Handle<YarnProjectFile>) -> LoadState {
    for _ in 0..1000 {
        let state = app.world().resource::<AssetServer>().load_state(handle);
        if !state.is_loading() {
            return state;
        }
        app.update();
    }
    panic!("YarnProjectFile never finished loading");
}
