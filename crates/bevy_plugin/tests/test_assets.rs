use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use yarn_slinger::prelude::YarnFile;

#[test]
fn loads_yarn_assets() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin);

    let asset_server = app.world.get_resource_mut::<AssetServer>().unwrap();
    let handle = asset_server.load("minimal.yarn");

    app.update();

    let yarn_file_assets = app.world.get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file = yarn_file_assets.get(&handle).unwrap();

    let expected_source = include_str!("../assets/minimal.yarn");
    assert_eq!(expected_source, yarn_file.source);
    assert_eq!("minimal.yarn", yarn_file.file_name);
}
