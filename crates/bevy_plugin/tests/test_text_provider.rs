use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn loads_line_without_localization() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]));

    while app.world.get_resource::<YarnProject>().is_none() {
        app.update();
    }

    let project = app.world.get_resource::<YarnProject>().unwrap();
    let text_provider = &project.text_provider;
    text_provider.get_text()
    let yarn_files = &project.yarn_files;
    assert_eq!(1, yarn_files.len());

    let yarn_file_assets = app.world.get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file = yarn_file_assets
        .get(yarn_files.iter().next().unwrap())
        .unwrap();

    let expected_source = include_str!("../assets/lines.yarn");
    assert_eq!(expected_source, yarn_file.content());
    assert_eq!("lines.yarn", yarn_file.file_name());
}
