use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;

#[test]
fn loads_line_without_localization() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec![
            "lines_with_ids.yarn",
        ]));

    while app.world.get_resource::<YarnProject>().is_none() {
        app.update();
    }
    app.update(); // Fill text providers

    let project = app.world.get_resource::<YarnProject>().unwrap();
    let text_provider = &project.text_provider;
    let line = text_provider
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!("Man: All right. I don't believe this; but there's no harm in wishing. I wish to know who I am.", line);
}

#[test]
fn fails_to_get_invalid_line() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec![
            "lines_with_ids.yarn",
        ]));

    while app.world.get_resource::<YarnProject>().is_none() {
        app.update();
    }

    let project = app.world.get_resource::<YarnProject>().unwrap();
    let text_provider = &project.text_provider;
    let result = text_provider.get_text(&LineId("line:99".to_owned()));
    assert!(result.is_none());
}
