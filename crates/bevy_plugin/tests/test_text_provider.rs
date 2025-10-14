use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use utils::prelude::*;

mod utils;

#[test]
fn loads_line_without_localization() {
    let mut app = App::new();

    app.setup_default_plugins()
        .add_plugins(YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file(
            "lines_with_ids.yarn",
        )));

    let line = app
        .dialogue_runner()
        .text_provider()
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!(
        "Man: All right. I don't believe this; but there's no harm in wishing. I wish to know who I am.",
        line
    );
}

#[test]
fn fails_to_get_invalid_line() {
    let mut app = App::new();

    app.setup_default_plugins()
        .add_plugins(YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file(
            "lines_with_ids.yarn",
        )));

    let result = app
        .dialogue_runner()
        .text_provider()
        .get_text(&LineId("line:99".to_owned()));
    assert!(result.is_none());
}

#[test]
fn loads_line_from_base_language_without_explicit_language() {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    app.load_lines();

    let line = app
        .dialogue_runner()
        .text_provider()
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!(
        "Man: All right. I don't believe this; but there's no harm in wishing. I wish to know who I am.",
        line
    );
}

#[test]
fn loads_line_from_base_language_with_explicit_language() {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    app.dialogue_runner_mut().set_text_language("en-US");

    app.load_lines();

    let line = app
        .dialogue_runner()
        .text_provider()
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!(
        "Man: All right. I don't believe this; but there's no harm in wishing. I wish to know who I am.",
        line
    );
}

#[test]
#[should_panic]
fn panics_when_loading_missing_language() {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    app.dialogue_runner_mut().set_text_language("fr-FR");

    app.load_lines();
}

#[test]
fn loads_line_from_fallback_on_missing_line() {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    app.dialogue_runner_mut().set_text_language("de-CH");

    app.load_lines();

    let line = app
        .dialogue_runner()
        .text_provider()
        .get_text(&LineId("line:10".to_owned()))
        .unwrap();
    assert_eq!("Hag: Funny,", line);
}

#[test]
fn loads_line_from_translated_language() {
    let mut app = App::new();

    app.setup_default_plugins().add_plugins(
        YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("lines_with_ids.yarn"))
            .with_localizations(Localizations {
                base_localization: "en-US".into(),
                translations: vec!["de-CH".into()],
            })
            .with_development_file_generation(DevelopmentFileGeneration::None),
    );

    app.dialogue_runner_mut().set_text_language("de-CH");

    app.load_lines();

    let line = app
        .dialogue_runner()
        .text_provider()
        .get_text(&LineId("line:9".to_owned()))
        .unwrap();
    assert_eq!(
        "Mann: Also gut. Ich glaub das zwar nicht, aber es kann ja nicht schaden, wenn ich mir was wünsche. Ich möchte wissen, wer ich bin.",
        line
    );
}
