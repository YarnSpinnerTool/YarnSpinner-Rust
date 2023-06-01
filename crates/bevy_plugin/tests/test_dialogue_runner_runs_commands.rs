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
    setup_dialogue_runner_for_wait(&mut app).start();
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

#[test]
fn executes_commands_and_fns() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app).start();
    app.update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == "Setting variable",
        ExecuteCommandEvent (n = 0),
    ]);

    app.continue_dialogue_and_update();
    let data = app
        .dialogue_runner()
        .variable_storage()
        .get("$data")
        .unwrap();
    let string_data: String = data.into();
    assert_eq!("foo", string_data.as_str());
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == "Calling command",
        ExecuteCommandEvent (n = 0),
    ]);

    app.continue_dialogue_and_update();
    let resource = app.world.resource::<Data>().0.as_str();
    assert_eq!("foo", resource);
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        ExecuteCommandEvent with |event| event.command.name == "set_data",
        ExecuteCommandEvent with |event| event.command.parameters.len() == 1,
        ExecuteCommandEvent with |event| String::from(&event.command.parameters[0]).as_str() == "foo",
    ]);

    app.update(); // Commands imply continue
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == "Calling function",
        ExecuteCommandEvent (n = 0),
    ]);

    app.continue_dialogue_and_update();
    assert_events!(app contains [
        PresentLineEvent with |event| event.line.text == "Data three times is foofoofoo",
        ExecuteCommandEvent (n = 0),
    ]);

    app.continue_dialogue_and_update();
    assert_events!(app contains [
        PresentLineEvent (n = 0),
        ExecuteCommandEvent with |event| event.command.name == "unregistered",
        ExecuteCommandEvent with |event| event.command.parameters.len() == 1,
        ExecuteCommandEvent with |event| String::from(&event.command.parameters[0]).as_str() == "method",
        DialogueCompleteEvent (n = 0),
        NodeCompleteEvent (n = 0),
    ]);

    app.update(); // Commands imply continue
    assert_events!(app contains [
        DialogueCompleteEvent,
        NodeCompleteEvent,
    ]);

    Ok(())
}

#[derive(Debug, Resource)]
struct Data(String);

fn setup_dialogue_runner(app: &mut App) -> Mut<DialogueRunner> {
    let mut dialogue_runner = setup_default_plugins(app)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["commands.yarn"]))
        .dialogue_runner_mut();
    dialogue_runner
        .command_registrations_mut()
        .register_command(
            "set_data",
            |In(param): In<String>, mut commands: Commands| {
                commands.insert_resource(Data(param));
            },
        );
    dialogue_runner
        .library_mut()
        .register_function("triplicate_data", |data: &str| {
            format!("{data}{data}{data}", data = data)
        });
    dialogue_runner
}

fn setup_dialogue_runner_for_wait(app: &mut App) -> Mut<DialogueRunner> {
    setup_default_plugins(app)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["wait.yarn"]))
        .dialogue_runner_mut()
}
