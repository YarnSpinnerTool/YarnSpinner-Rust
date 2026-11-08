#![cfg(not(any(target_arch = "wasm32", target_os = "android")))]
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use utils::prelude::*;

mod utils;

#[test]
fn loads_folder_with_relative_asset_root() {
    // SAFETY: this binary has a single test, so no other thread reads the environment.
    unsafe { std::env::set_var("BEVY_ASSET_ROOT", "./") };
    let mut app = App::new();
    app.setup_default_plugins_for_path("assets")
        .add_plugins(YarnSpinnerPlugin::with_yarn_source(YarnFileSource::folder(
            "folder",
        )));

    let project = app.load_project();

    assert_eq!(project.yarn_files().count(), 1);
    assert!(project.headers_for_node("Start").is_some());
}
