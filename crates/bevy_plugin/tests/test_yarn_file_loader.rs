use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use utils::prelude::*;

mod utils;

#[test]
fn loads_yarn_assets() {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("lines.yarn"))
            .with_localizations(None),
    );

    let yarn_files: Vec<_> = app.load_project().yarn_files().cloned().collect();
    assert_eq!(1, yarn_files.len());

    let yarn_file_assets = app.world().get_resource::<Assets<YarnFile>>().unwrap();
    let yarn_file = yarn_file_assets.get(&yarn_files[0]).unwrap();

    let expected_source = include_str!("../assets/lines.yarn");
    assert_eq!(expected_source, yarn_file.content());
    assert_eq!("lines.yarn", yarn_file.file_name());
}
