use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger::UnderlyingYarnLine;
use utils::prelude::*;

mod utils;

#[test]
fn does_not_load_asset_without_localization() {
    todo!()
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
    let dialogue_runner = project
        .build_dialogue_runner()
        .with_asset_provider(FileExtensionAssetProvider::from(&["ogg"]))
        .build();
    app.world.spawn(dialogue_runner);
    app.load_assets();

    let assets = app.dialogue_runner().get_assets_for_id("line:9");
    assert_eq!(1, assets.len());
    let asset: Handle<AudioSource> = assets.get_handle().unwrap();
    let asset_server = app.world.resource::<AssetServer>();
    let path = asset_server.get_handle_path(asset).unwrap();
    assert_eq!("9.ogg", path.path().to_str().unwrap())
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
