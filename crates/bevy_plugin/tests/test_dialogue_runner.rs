use anyhow::Result;
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarn_slinger::prelude::*;
use std::thread::sleep;
use std::time::Duration;
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
        PresentLineEvent with |event| event.line.text == FIRST_LINE,
    ]);

    Ok(())
}

#[test]
fn presents_all_lines() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_without_localizations(&mut app).start()?;
    for i in 1..=12 {
        println!("Line {i}");
        app.dialogue_runner_mut().continue_in_next_update();
        app.update();
        assert_events!(app contains PresentLineEvent);
    }
    assert_events!(app contains [
        NodeCompleteEvent (n = 0),
        DialogueCompleteEvent (n = 0),
    ]);
    println!("End of lines");
    app.dialogue_runner_mut().continue_in_next_update();
    app.update();
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

    app.wait_until_ready().update();
    assert_events!(app contains [
        DialogueStartEvent (n = 0),
        LineHintsEvent (n = 0),
        NodeStartEvent,
        PresentLineEvent with |event| event.line.text == FIRST_LINE,
        PresentLineEvent with |event| event.line.assets.get_handle::<AudioSource>().is_some(),
    ]);

    Ok(())
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
        .add_asset_provider(FileExtensionAssetProvider::new().with_audio())
        .build()
        .unwrap();
    app.world.spawn(dialogue_runner);
    app.world
        .query::<&mut DialogueRunner>()
        .single_mut(&mut app.world)
}

const FIRST_LINE: & str =
    "An elderly man was sitting alone on a dark path. He wasn't certain of which direction to go, and he'd forgotten both where he was travelling to and who he was. \
            He'd sat down for a moment to rest his weary legs, and suddenly looked up to see an elderly woman before him. She grinned toothlessly and with a cackle, spoke:";

trait AppExt2 {
    fn wait_until_ready(&mut self) -> &mut App;
}

impl AppExt2 for App {
    fn wait_until_ready(&mut self) -> &mut App {
        let start = Instant::now();
        while !self
            .dialogue_runner()
            .data_providers()
            .are_lines_available()
        {
            if start.elapsed().as_secs() > 2 {
                panic!("Timeout while waiting for lines to be available");
            }
            sleep(Duration::from_millis(100));
        }
        self
    }
}
