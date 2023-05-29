use crate::commands::UntypedYarnCommand;
use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn command_execution_plugin(app: &mut App) {
    app.add_system(execute_commands);
}

fn execute_commands(world: &mut World) {
    let events = clone_events(world);
    for event in events {
        let Some(mut command) = clone_command(world, &event) else {
            continue;
        };
        let params = event.command.parameters;
        let task = command.call(params, world);
        if let Some(task) = task {
            let mut dialogue_runner = get_dialogue_runner_mut(world, event.source);
            dialogue_runner.command_tasks.push(task);
        }
    }
}

fn clone_events(world: &mut World) -> Vec<ExecuteCommandEvent> {
    let events = world.resource::<Events<ExecuteCommandEvent>>();
    let mut reader = events.get_reader();
    reader.iter(&events).cloned().collect()
}

fn clone_command(
    world: &mut World,
    event: &ExecuteCommandEvent,
) -> Option<Box<dyn UntypedYarnCommand>> {
    let dialogue_runner = get_dialogue_runner(world, event.source);
    let command_name = event.command.name.as_str();
    dialogue_runner
        .commands
        .get(command_name)
        .map(|command| command.clone_box())
}

fn get_dialogue_runner(world: &mut World, entity: Entity) -> &DialogueRunner {
    let mut dialogue_runners = world.query::<&DialogueRunner>();
    let dialogue_runner = dialogue_runners.get(world, entity).unwrap();
    dialogue_runner
}

fn get_dialogue_runner_mut(world: &mut World, entity: Entity) -> Mut<DialogueRunner> {
    let mut dialogue_runners = world.query::<&mut DialogueRunner>();
    let dialogue_runner = dialogue_runners.get_mut(world, entity).unwrap();
    dialogue_runner
}
