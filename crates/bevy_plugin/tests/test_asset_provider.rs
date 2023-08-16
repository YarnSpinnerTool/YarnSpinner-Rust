#![cfg(feature = "audio_assets")]
use anyhow::{bail, Result};
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger::UnderlyingYarnLine;
use utils::prelude::*;

mod utils;

#[test]
fn does_not_load_asset_without_localizations() -> Result<()> {
    let mut app = App::new();

    app.setup_default_plugins()
        .add_plugins(YarnSlingerPlugin::with_yarn_source(YarnFileSource::file(
            "lines_with_ids.yarn",
        )));

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build();
    dialogue_runner.start_node("Start");
    app.world.spawn(dialogue_runner);

    app.load_project();
    let start = Instant::now();
    while !app.dialogue_runner().are_lines_available() {
        if start.elapsed().as_secs() > 2 {
            return Ok(());
        }
        app.update();
    }
    bail!("Did not expect to load assets without localizations");
}

#[test]
fn does_not_load_invalid_asset_id() -> Result<()> {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSlingerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec![],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build();
    dialogue_runner
        .set_asset_language("en-US")
        .start_node("Start");
    app.world.spawn(dialogue_runner);
    app.load_lines();

    let assets = app.dialogue_runner().get_assets_for_id("line:99");
    assert!(assets.is_empty());
    Ok(())
}

#[test]
fn loads_asset_from_base_language_localization() -> Result<()> {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSlingerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec![],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build();
    dialogue_runner.start_node("Start");
    app.world.spawn(dialogue_runner);
    app.load_lines();

    let assets = app.dialogue_runner().get_assets_for_id("line:9");
    assert_eq!(1, assets.len());
    let asset: Handle<AudioSource> = assets.get_handle().unwrap();
    let asset_server = app.world.resource::<AssetServer>();
    let path = asset_server.get_handle_path(asset).unwrap();

    // Note that this does not contains backslashes on Windows
    assert_eq!("dialogue/en-US/9.ogg", path.path().to_str().unwrap());

    Ok(())
}

#[test]
fn loads_asset_from_translated_localization() -> Result<()> {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSlingerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build();
    dialogue_runner
        .set_asset_language("de-CH")
        .start_node("Start");
    app.world.spawn(dialogue_runner);
    app.load_lines();

    let assets = app.dialogue_runner().get_assets_for_id("line:10");
    assert_eq!(1, assets.len());
    let asset: Handle<AudioSource> = assets.get_handle().unwrap();
    let asset_server = app.world.resource::<AssetServer>();
    let path = asset_server.get_handle_path(asset).unwrap();

    // Note that this does not contains backslashes on Windows
    assert_eq!("dialogue/de-CH/10.ogg", path.path().to_str().unwrap());
    Ok(())
}

#[test]
#[should_panic]
fn panics_on_invalid_language() {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSlingerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build();
    dialogue_runner
        .set_asset_language("fr-FR")
        .start_node("Start");
    app.world.spawn(dialogue_runner);
    app.load_lines();
}

#[test]
fn does_not_load_asset_with_invalid_type() -> Result<()> {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSlingerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec![],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build();

    dialogue_runner
        .set_asset_language("en-US")
        .start_node("Start");
    app.world.spawn(dialogue_runner);
    app.load_lines();

    let assets = app.dialogue_runner().get_assets_for_id("line:9");
    assert_eq!(1, assets.len());
    let asset: Option<Handle<YarnFile>> = assets.get_handle();
    assert!(asset.is_none());
    Ok(())
}

trait AssetProviderExt {
    fn get_assets_for_id(&self, id: &str) -> LineAssets;
}

impl<T> AssetProviderExt for T
where
    T: AssetProvider + ?Sized,
{
    fn get_assets_for_id(&self, id: &str) -> LineAssets {
        let line_id = LineId(id.to_owned());
        let yarn_line = UnderlyingYarnLine {
            id: line_id,
            text: String::new(),
            attributes: vec![],
        };
        T::get_assets(self, &yarn_line)
    }
}
