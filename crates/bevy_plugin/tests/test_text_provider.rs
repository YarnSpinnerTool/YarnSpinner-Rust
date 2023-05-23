use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;

#[test]
fn loads_line_without_localization() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec![
            "lines_with_ids.yarn",
        ]));

    while !app.world.contains_resource::<YarnProject>() {
        app.update();
    }

    let project = app.world.resource::<YarnProject>();
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

    while !app.world.contains_resource::<YarnProject>() {
        app.update();
    }

    let project = app.world.resource::<YarnProject>();
    let text_provider = &project.text_provider;
    let result = text_provider.get_text(&LineId("line:99".to_owned()));
    assert!(result.is_none());
}

#[test]
fn loads_line_from_base_language_without_explicit_language() {
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

    while !app.world.contains_resource::<YarnProject>() {
        app.update();
    }
    app.update_until_translation_was_loaded();

    let project = app.world.resource::<YarnProject>();
    let text_provider = &project.text_provider;
    let line = text_provider
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!("Man: All right. I don't believe this; but there's no harm in wishing. I wish to know who I am.", line);
}

#[test]
fn loads_line_from_base_language_with_explicit_language() {
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

    while !app.world.contains_resource::<YarnProject>() {
        app.update();
    }

    {
        let mut project = app.world.resource_mut::<YarnProject>();
        project.set_text_language(Language::from("en-US"));
    }
    app.update_until_translation_was_loaded();

    let project = app.world.resource::<YarnProject>();
    let text_provider = &project.text_provider;
    let line = text_provider
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!("Man: All right. I don't believe this; but there's no harm in wishing. I wish to know who I am.", line);
}

#[test]
#[should_panic]
fn panics_when_loading_missing_language() {
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

    while !app.world.contains_resource::<YarnProject>() {
        app.update();
    }

    {
        let mut project = app.world.resource_mut::<YarnProject>();
        project.set_text_language(Language::from("fr-FR"));
    }
    app.update_until_translation_was_loaded();
}

#[test]
fn loads_line_from_fallback_on_missing_line() {
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

    while !app.world.contains_resource::<YarnProject>() {
        app.update();
    }

    {
        let mut project = app.world.resource_mut::<YarnProject>();
        project.set_text_language(Language::from("de-CH"));
    }
    app.update_until_translation_was_loaded();

    let project = app.world.resource::<YarnProject>();
    let text_provider = &project.text_provider;
    let line = text_provider
        .get_text(&LineId("line:10".to_owned()))
        .unwrap();
    assert_eq!("Hag: Funny,", line);
}

#[test]
fn loads_line_from_translated_language() {
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

    while !app.world.contains_resource::<YarnProject>() {
        app.update();
    }

    {
        let mut project = app.world.resource_mut::<YarnProject>();
        project.set_text_language(Language::from("de-CH"));
    }

    app.update_until_translation_was_loaded();

    let project = app.world.resource::<YarnProject>();
    let text_provider = &project.text_provider;
    let line = text_provider
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!("Mann: Also gut. Ich glaub das zwar nicht, aber es kann ja nicht schaden, wenn ich mir was wünsche. Ich möchte wissen, wer ich bin.", line);
}

trait AppExt {
    fn update_until_translation_was_loaded(&mut self);
}

impl AppExt for App {
    fn update_until_translation_was_loaded(&mut self) {
        while self.world.is_resource_changed::<YarnProject>() {
            self.update();
        }

        while !self
            .world
            .resource::<YarnProject>()
            .text_provider
            .has_loaded_translation_for_current_language()
        {
            self.update();
        }
    }
}
