use anyhow::Result;
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarn_slinger::prelude::*;
use std::thread::sleep;
use utils::prelude::*;

mod utils;

#[test]
fn waits_on_command() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner_for_wait(&mut app).start()?;
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == "Starting wait",
        ExecuteCommandEvent (n = 0),
    ]);
    app.continue_dialogue_and_update();
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        ExecuteCommandEvent with |event| event.command.name == "wait",
        ExecuteCommandEvent with |event| event.command.parameters.len() == 1,
        ExecuteCommandEvent with |event| f32::try_from(&event.command.parameters[0]).unwrap() == 1.0,
    ]);
    let now = Instant::now();
    while now.elapsed().as_millis() <= 950 {
        app.continue_dialogue_and_update();
        assert_events!(app contains [
            PresentLineEvent (n = 0),
            ExecuteCommandEvent (n = 0),
        ]);
    }
    sleep(std::time::Duration::from_millis(50));
    app.continue_dialogue_and_update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == "Ended wait",
        ExecuteCommandEvent (n = 0),
    ]);

    Ok(())
}

fn setup_dialogue_runner_for_wait(app: &mut App) -> Mut<DialogueRunner> {
    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["wait.yarn"]))
        .dialogue_runner_mut()
}
