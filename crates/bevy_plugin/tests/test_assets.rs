use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn loads_yarn_assets() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_localizations(None));

    let asset_server = app.world.get_resource::<AssetServer>().unwrap();
    let handle = asset_server.load("lines.yarn");

    app.update();

    let yarn_file_assets = app.world.get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file = yarn_file_assets.get(&handle).unwrap();

    let expected_source = include_str!("../assets/lines.yarn");
    assert_eq!(expected_source, yarn_file.file.source);
    assert_eq!("lines.yarn", yarn_file.file.file_name);
}

#[test]
fn generates_localization_files() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let yarn_path = dir.path().join("lines.yarn");
    fs::copy(project_root_path().join("assets/lines.yarn"), &yarn_path)?;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        asset_folder: dir.path().to_str().unwrap().to_string(),
        ..default()
    }))
    .add_plugin(YarnSlingerPlugin::with_localizations(Some(Localizations {
        base: "en-US".into(),
        translations: vec!["de-CH".into()],
        file_generation_mode: FileGenerationMode::Development,
    })));
    let asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
    let handle = asset_server.load("lines.yarn");

    app.update();

    let yarn_file_assets = app.world.get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file = yarn_file_assets.get(&handle).unwrap();

    let expected_source = include_str!("../assets/lines.yarn");
    assert_eq!(expected_source, yarn_file.file.source);
    assert_eq!("lines.yarn", yarn_file.file.file_name);
    Ok(())
}
pub fn project_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
