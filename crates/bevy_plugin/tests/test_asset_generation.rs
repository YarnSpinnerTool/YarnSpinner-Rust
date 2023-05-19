use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;
use yarn_slinger::prelude::{CompilationType, YarnCompiler};

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
    assert_eq!(expected_source, yarn_file.content());
    assert_eq!("lines.yarn", yarn_file.file_name());
}

#[test]
fn generates_line_ids() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let original_yarn_path = project_root_path().join("assets/lines.yarn");
    let yarn_path = dir.path().join("lines.yarn");
    fs::copy(&original_yarn_path, &yarn_path)?;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        asset_folder: dir.path().to_str().unwrap().to_string(),
        ..default()
    }))
    .add_plugin(YarnSlingerPlugin::with_localizations(Localizations {
        base_language: "en-US".into(),
        translations: vec!["de-CH".into()],
        file_generation_mode: FileGenerationMode::Development,
    }));
    let asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
    let handle = asset_server.load("lines.yarn");

    app.update(); // read yarn
    app.update(); // write line IDs
    app.update();

    let yarn_file_assets = app.world.get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file_in_app = yarn_file_assets.get(&handle).unwrap();
    let yarn_file_on_disk = fs::read_to_string(&yarn_path)?;

    assert_eq!(yarn_file_in_app.content(), yarn_file_on_disk);
    let string_table_without_line_ids = YarnCompiler::new()
        .read_file(&original_yarn_path)
        .with_compilation_type(CompilationType::StringsOnly)
        .compile()?
        .string_table;
    let string_table_with_line_ids = YarnCompiler::new()
        .read_file(&yarn_path)
        .with_compilation_type(CompilationType::StringsOnly)
        .compile()?
        .string_table;

    println!("{string_table_with_line_ids:#?}");
    assert!(string_table_with_line_ids
        .values()
        .all(|string_info| !string_info.is_implicit_tag));
    assert_eq!(
        string_table_without_line_ids.len(),
        string_table_with_line_ids.len()
    );
    Ok(())
}

#[test]
fn generates_strings_file() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let original_yarn_path = project_root_path().join("assets/lines.yarn");
    let yarn_path = dir.path().join("lines.yarn");
    fs::copy(original_yarn_path, &yarn_path)?;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        asset_folder: dir.path().to_str().unwrap().to_string(),
        ..default()
    }))
    .add_plugin(YarnSlingerPlugin::with_localizations(Localizations {
        base_language: "en-US".into(),
        translations: vec!["de-CH".into()],
        file_generation_mode: FileGenerationMode::Development,
    }));
    let asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
    let _handle: Handle<YarnFile> = asset_server.load("lines.yarn");

    app.update(); // read yarn
    app.update(); // write line IDs and strings file
    app.update(); // write updated strings file
    app.update();

    let string_table = YarnCompiler::new()
        .read_file(&yarn_path)
        .with_compilation_type(CompilationType::StringsOnly)
        .compile()?
        .string_table;

    assert!(!dir.path().join("en-US.strings.csv").exists());
    let strings_file_path = dir.path().join("de-CH.strings.csv");
    assert!(strings_file_path.exists());
    let strings_file_source = fs::read_to_string(&strings_file_path)?;
    let strings_file_line_ids: Vec<_> = strings_file_source
        .lines()
        .skip(1)
        .map(|line| line.split(',').nth(1).unwrap())
        .collect();

    assert_eq!(string_table.len(), strings_file_line_ids.len());

    assert!(strings_file_line_ids
        .iter()
        .all(|line_id| string_table.contains_key(&LineId(line_id.to_string()))));

    Ok(())
}

#[test]
fn regenerates_strings_files_on_changed_localization() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let original_yarn_path = project_root_path().join("assets/lines.yarn");
    let yarn_path = dir.path().join("lines.yarn");
    fs::copy(original_yarn_path, yarn_path)?;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        asset_folder: dir.path().to_str().unwrap().to_string(),
        ..default()
    }))
    .add_plugin(YarnSlingerPlugin::with_localizations(Localizations {
        base_language: "en-US".into(),
        translations: vec!["de-CH".into()],
        file_generation_mode: FileGenerationMode::Development,
    }));
    let asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
    let _handle: Handle<YarnFile> = asset_server.load("lines.yarn");

    app.update(); // read yarn
    app.update(); // write line IDs and strings file
    app.update(); // write updated strings file

    let mut localizations = app.world.get_resource_mut::<Localizations>().unwrap();
    localizations.translations = vec!["fr-FR".into()];

    app.update(); // write updated strings file

    assert!(!dir.path().join("en-US.strings.csv").exists());
    let strings_file_path = dir.path().join("fr-FR.strings.csv");
    assert!(strings_file_path.exists());
    let strings_file_source = fs::read_to_string(&strings_file_path)?;
    let strings_file_line_languages: Vec<_> = strings_file_source
        .lines()
        .skip(1)
        .map(|line| line.split(',').next().unwrap())
        .collect();

    assert!(!strings_file_line_languages.is_empty());

    assert!(strings_file_line_languages
        .iter()
        .all(|&language| language == "fr-FR"));

    Ok(())
}

pub fn project_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
