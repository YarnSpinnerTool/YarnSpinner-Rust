use anyhow::Result;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use utils::prelude::*;

mod utils;

#[test]
fn delivers_option_set() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_without_localizations(&mut app).start()?;
    app.continue_dialogue_and_update_n_times(4);
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        PresentOptionsEvent with |event| event.options.len() == 2,
        PresentOptionsEvent with |event| event.options.iter().all(|o| o.is_available),
    ]);

    Ok(())
}

#[test]
fn option_selection_implies_continue() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_without_localizations(&mut app).start()?;
    app.continue_dialogue_and_update_n_times(4);
    app.dialogue_runner_mut().select_option(OptionId(0))?;
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == english_lines()[6],
        PresentOptionsEvent (n = 0),
    ]);

    Ok(())
}

#[test]
fn respects_conditional_availability() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_without_localizations(&mut app).start()?;
    app.continue_dialogue_and_update_n_times(4);
    app.dialogue_runner_mut().select_option(OptionId(0))?;
    app.continue_dialogue_and_update_n_times(2);
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        PresentOptionsEvent with |event| event.options.len() == 2,
        PresentOptionsEvent with |event| event.options.iter().filter(|o| o.is_available).count() == 1,
    ]);

    Ok(())
}

fn setup_dialogue_runner_without_localizations(app: &mut App) -> Mut<DialogueRunner> {
    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["options.yarn"]))
        .dialogue_runner_mut()
}

fn english_lines() -> Vec<String> {
    let mut lines: Vec<_> = include_str!("../assets/options.yarn")
        .lines()
        .filter(|l| !l.starts_with("title:"))
        .filter(|l| !l.starts_with("position:"))
        .filter(|l| !l.starts_with("---"))
        .filter(|l| !l.starts_with("==="))
        .filter(|l| !l.is_empty())
        .map(|line| line.trim().to_owned())
        .collect();
    lines.pop();
    lines
}
