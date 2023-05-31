use anyhow::Result;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use utils::prelude::*;

mod utils;

#[test]
fn errs_on_continue_without_start() -> Result<()> {
    let mut app = App::new();
    let mut dialogue_runner = setup_dialogue_runner_without_localizations(&mut app);
    let result = dialogue_runner.try_continue_in_next_update();
    assert!(result.is_err());
    Ok(())
}

#[test]
fn start_implies_continue() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_without_localizations(&mut app).start()?;
    app.update();
    assert_events!(app contains [
        DialogueStartEvent,
        NodeStartEvent,
        LineHintsEvent,
        PresentLineEvent with |event| event.line.text == english_lines()[0],
    ]);

    Ok(())
}

#[test]
fn presents_all_lines() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_without_localizations(&mut app).start()?;
    for i in 1..=12 {
        println!("Line {i}");
        app.continue_dialogue_and_update();
        assert_events!(app contains PresentLineEvent);
    }
    assert_events!(app contains [
        NodeCompleteEvent (n = 0),
        DialogueCompleteEvent (n = 0),
    ]);
    println!("End of lines");
    app.continue_dialogue_and_update();
    assert_events!(app contains [
        NodeCompleteEvent,
        DialogueCompleteEvent,
        PresentLineEvent (n = 0),
    ]);
    assert!(!app.dialogue_runner().is_running());
    Ok(())
}

#[test]
fn serves_assets_after_loading() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_with_localizations(&mut app).start()?;
    app.update();
    assert_events!(app contains [
        DialogueStartEvent,
        LineHintsEvent,
        NodeStartEvent (n = 0),
        PresentLineEvent (n = 0),
    ]);

    app.load_lines();
    assert_events!(app contains [
        DialogueStartEvent (n = 0),
        LineHintsEvent (n = 0),
        NodeStartEvent,
        PresentLineEvent with |event| event.line.text == english_lines()[0],
        PresentLineEvent with |event| event.line.assets.is_empty(),
    ]);

    for _ in 2..=8 {
        app.continue_dialogue_and_update();
        assert_events!(app contains PresentLineEvent with |event| event.line.assets.is_empty() );
    }
    app.continue_dialogue_and_update();
    assert_events!(app contains PresentLineEvent with |event| event.line.assets.get_handle::<AudioSource>().is_some());
    Ok(())
}

#[test]
fn serves_translations() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_with_localizations(&mut app).start()?;
    app.load_lines();

    for _ in 1..=6 {
        app.continue_dialogue_and_update();
    }
    app.dialogue_runner_mut()
        .set_asset_language("de-CH")
        .continue_in_next_update();
    app.load_lines();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == english_lines()[7],
        PresentLineEvent with |event| event.line.assets.get_handle::<AudioSource>().is_some(),
    ]);

    app.dialogue_runner_mut()
        .set_text_language("de-CH")
        .continue_in_next_update();
    app.load_lines();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == german_lines()[8],
        PresentLineEvent with |event| event.line.assets.get_handle::<AudioSource>().is_none(),
    ]);
    app.dialogue_runner_mut()
        .set_language("en-US")
        .continue_in_next_update();
    app.load_lines();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == english_lines()[9], // There's no German line 10 (1-indexed), so this falls back to English anyway
        PresentLineEvent with |event| event.line.assets.get_handle::<AudioSource>().is_none(),
    ]);

    Ok(())
}

#[test]
fn default_language_is_none_without_localizations() {
    let mut app = App::new();
    let dialogue_runner = setup_dialogue_runner_without_localizations(&mut app);
    assert_eq!(None, dialogue_runner.text_language());
}

#[test]
#[should_panic]
fn panics_on_invalid_language() {
    let mut app = App::new();
    let mut dialogue_runner = setup_dialogue_runner_with_localizations(&mut app);
    dialogue_runner.set_language("fr-FR");
}

#[test]
#[should_panic]
fn panics_on_setting_language_without_localizations() {
    let mut app = App::new();
    let mut dialogue_runner = setup_dialogue_runner_without_localizations(&mut app);
    dialogue_runner.set_language("en-US");
}

#[test]
fn default_language_is_base_language() {
    let mut app = App::new();
    let dialogue_runner = setup_dialogue_runner_with_localizations(&mut app);
    assert_eq!(
        Some(Language::from("en-US")),
        dialogue_runner.text_language()
    );
    assert_eq!(
        Some(Language::from("en-US")),
        dialogue_runner.asset_language()
    );
}

fn setup_dialogue_runner_without_localizations(app: &mut App) -> Mut<DialogueRunner> {
    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["lines.yarn"]))
        .dialogue_runner_mut()
}

fn setup_dialogue_runner_with_localizations(app: &mut App) -> Mut<DialogueRunner> {
    let dialogue_runner = app
        .add_plugins(DefaultPlugins)
        .add_plugin(
            YarnSlingerPlugin::with_yarn_files(vec!["lines_with_ids.yarn"]).with_localizations(
                Localizations {
                    base_language: "en-US".into(),
                    translations: vec!["de-CH".into()],
                    file_generation_mode: FileGenerationMode::Production,
                },
            ),
        )
        .load_project()
        .build_dialogue_runner()
        .add_asset_provider(AudioAssetProvider::new())
        .build()
        .unwrap();
    app.world.spawn(dialogue_runner);
    app.world
        .query::<&mut DialogueRunner>()
        .single_mut(&mut app.world)
}

fn english_lines() -> Vec<String> {
    let mut lines: Vec<_> = include_str!("../assets/lines.yarn")
        .lines()
        .skip(2)
        .filter(|l| !l.is_empty())
        .map(|line| line.trim().to_owned())
        .collect();
    lines.pop();
    lines
}
fn german_lines() -> Vec<String> {
    let file = include_str!("../assets/de-CH.strings.csv");
    let mut reader = csv::Reader::from_reader(file.as_bytes());
    reader
        .records()
        .map(|r| r.unwrap().get(2).unwrap().to_string())
        .collect()
}
