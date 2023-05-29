use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger::UnderlyingYarnLine;
use utils::prelude::*;

mod utils;

#[test]
fn does_not_load_asset_without_localizations() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec![
            "lines_with_ids.yarn",
        ]));

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .build();
    dialogue_runner.continue_in_next_update();
    app.world.spawn(dialogue_runner);

    app.load_project();
    let start = Instant::now();
    while !app.dialogue_runner().are_line_assets_available() {
        if start.elapsed().as_secs() > 2 {
            return;
        }
        app.update();
    }
    panic!("Did not expect to load assets without localizations");
}

#[test]
fn does_not_load_asset_without_language() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec![],
                file_generation_mode: FileGenerationMode::Production,
            },
        ),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .build();
    dialogue_runner.continue_in_next_update();
    app.world.spawn(dialogue_runner);

    let start = Instant::now();
    while !app.dialogue_runner().are_line_assets_available() {
        if start.elapsed().as_secs() > 2 {
            return;
        }
        app.update();
    }
    panic!("Did not expect to load assets without language");
}

#[test]
fn does_not_load_invalid_asset_id() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec![],
                file_generation_mode: FileGenerationMode::Production,
            },
        ),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .with_asset_language(Language::new("en-US"))
        .build();
    dialogue_runner.continue_in_next_update();
    app.world.spawn(dialogue_runner);
    app.load_assets();

    let assets = app.dialogue_runner().get_assets_for_id("line:99");
    assert!(assets.is_empty());
}

#[test]
fn loads_asset_from_base_language_localization() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec![],
                file_generation_mode: FileGenerationMode::Production,
            },
        ),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .with_asset_language(Language::new("en-US"))
        .build();
    dialogue_runner.continue_in_next_update();
    app.world.spawn(dialogue_runner);
    app.load_assets();

    let assets = app.dialogue_runner().get_assets_for_id("line:9");
    assert_eq!(1, assets.len());
    let asset: Handle<AudioSource> = assets.get_handle().unwrap();
    let asset_server = app.world.resource::<AssetServer>();
    let path = asset_server.get_handle_path(asset).unwrap();

    // Note that this does not contains backslashes on Windows
    assert_eq!("en-US/9.ogg", path.path().to_str().unwrap())
}

#[test]
fn loads_asset_from_translated_localization() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec!["de-CH".into()],
                file_generation_mode: FileGenerationMode::Production,
            },
        ),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .with_asset_language(Language::new("de-CH"))
        .build();
    dialogue_runner.continue_in_next_update();
    app.world.spawn(dialogue_runner);
    app.load_assets();

    let assets = app.dialogue_runner().get_assets_for_id("line:10");
    assert_eq!(1, assets.len());
    let asset: Handle<AudioSource> = assets.get_handle().unwrap();
    let asset_server = app.world.resource::<AssetServer>();
    let path = asset_server.get_handle_path(asset).unwrap();

    // Note that this does not contains backslashes on Windows
    assert_eq!("de-CH/10.ogg", path.path().to_str().unwrap())
}

#[test]
#[should_panic]
fn panics_on_invalid_language() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec!["de-CH".into()],
                file_generation_mode: FileGenerationMode::Production,
            },
        ),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .with_asset_language(Language::new("fr-FR"))
        .build();
    dialogue_runner.continue_in_next_update();
    app.world.spawn(dialogue_runner);
    app.load_assets();
}

#[test]
fn does_not_load_asset_with_invalid_type() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins).add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
            Localizations {
                base_language: "en-US".into(),
                translations: vec![],
                file_generation_mode: FileGenerationMode::Production,
            },
        ),
    );

    let project = app.load_project();
    let mut dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .with_asset_language(Language::new("en-US"))
        .build();
    dialogue_runner.continue_in_next_update();
    app.world.spawn(dialogue_runner);
    app.load_assets();

    let assets = app.dialogue_runner().get_assets_for_id("line:9");
    assert_eq!(1, assets.len());
    let asset: Option<Handle<YarnFile>> = assets.get_handle();
    assert!(asset.is_none())
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
