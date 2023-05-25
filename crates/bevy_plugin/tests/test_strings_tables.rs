use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;
use utils::prelude::*;
use yarn_slinger::prelude::{CompilationType, YarnCompiler};

mod utils;

#[test]
fn loads_yarn_assets() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines.yarn"]).with_localizations(None),
    );

    let yarn_files: Vec<_> = app.load_project().yarn_files().cloned().collect();
    assert_eq!(1, yarn_files.len());

    let yarn_file_assets = app.world.get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file = yarn_file_assets.get(&yarn_files[0]).unwrap();

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
    .add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines.yarn"]).with_localizations(Localizations {
            base_language: "en-US".into(),
            translations: vec!["de-CH".into()],
            file_generation_mode: FileGenerationMode::Development,
        }),
    );

    let yarn_file = app.load_project().yarn_files().next().unwrap().clone();

    let yarn_file_assets = app.world.get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file_in_app = yarn_file_assets.get(&yarn_file).unwrap();
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
    .add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines.yarn"]).with_localizations(Localizations {
            base_language: "en-US".into(),
            translations: vec!["de-CH".into()],
            file_generation_mode: FileGenerationMode::Development,
        }),
    );

    app.load_project();
    app.update(); // Generate the strings file

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
fn replaces_entries_in_strings_file() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let original_yarn_path = project_root_path().join("assets/lines_with_ids.yarn");
    let yarn_path = dir.path().join("lines_with_ids.yarn");
    fs::copy(original_yarn_path, yarn_path)?;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        asset_folder: dir.path().to_str().unwrap().to_string(),
        ..default()
    }))
    .add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec!["de-CH".into()],
                file_generation_mode: FileGenerationMode::Development,
            },
        ),
    );

    app.load_project();

    {
        let project = app.world.resource::<YarnProject>();
        let handle = project.yarn_files().next().unwrap().clone();

        let mut yarn_file_assets = app.world.get_resource_mut::<Assets<YarnFile>>().unwrap();
        let yarn_file = yarn_file_assets.get_mut(&handle).unwrap();

        let mut lines: Vec<_> = yarn_file.content().lines().collect();
        *lines.get_mut(3).unwrap() = "Changed line #line:2";
        lines.insert(4, "Inserted line #line:13");
        lines.remove(6);
        yarn_file.set_content(lines.join("\n"))?;
    }

    while !app
        .world
        .resource::<Events<AssetEvent<YarnFile>>>()
        .is_empty()
    {
        app.update();
    }

    let strings_file_source = fs::read_to_string(dir.path().join("de-CH.strings.csv"))?;
    let strings_file_lines: Vec<_> = strings_file_source
        .lines()
        .skip(1)
        .map(|line| line.split(',').collect::<Vec<_>>())
        .collect();

    println!("{strings_file_lines:#?}");
    assert_eq!(strings_file_lines[1][1], "line:2");
    assert_eq!(
        strings_file_lines[1][2],
        "(NEEDS UPDATE) Hag: Now your *third* wish. What will it be?"
    );
    assert_eq!(strings_file_lines[2][1], "line:13");
    assert_eq!(strings_file_lines[2][2], "Inserted line");
    assert_eq!(strings_file_lines[4][1], "line:5");
    assert_eq!(
        strings_file_lines[4][2],
        "Man: How can it be a third wish if I haven't had a first and second wish?"
    );

    assert_eq!(strings_file_lines.len(), 12);

    Ok(())
}

#[test]
fn does_not_panic_on_missing_language_when_not_selected() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec!["fr-FR".into()],
                file_generation_mode: FileGenerationMode::Production,
            },
        ),
    );

    app.load_project();
}

pub fn project_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
