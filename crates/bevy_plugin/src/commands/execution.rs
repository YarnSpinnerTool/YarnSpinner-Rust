use crate::commands::UntypedYarnCommand;
use crate::events::ExecuteCommand;
use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn command_execution_plugin(app: &mut App) {
    app.add_observer(execute_commands);
}

fn execute_commands(event: On<ExecuteCommand>, mut commands: Commands) {
    let event = event.clone();
    commands.queue(move |world: &mut World| {
        let Some(mut command) = clone_command(world, &event) else {
            return;
        };

        let params = event.command.parameters;
        let task_finished_indicator = command.call(params, world);
        if !task_finished_indicator.is_finished() {
            get_dialogue_runner_mut(world, event.entity).add_command_task(task_finished_indicator);
        }
    });
}

fn clone_command(world: &mut World, event: &ExecuteCommand) -> Option<Box<dyn UntypedYarnCommand>> {
    let dialogue_runner = get_dialogue_runner(world, event.entity);
    let command_name = event.command.name.as_str();
    dialogue_runner
        .commands
        .get(command_name)
        .map(|command| command.clone_box())
}

fn get_dialogue_runner(world: &mut World, entity: Entity) -> &DialogueRunner {
    let mut dialogue_runners = world.query::<&DialogueRunner>();

    (dialogue_runners.get(world, entity).unwrap()) as _
}

fn get_dialogue_runner_mut(world: &mut World, entity: Entity) -> Mut<'_, DialogueRunner> {
    let mut dialogue_runners = world.query::<&mut DialogueRunner>();

    dialogue_runners.get_mut(world, entity).unwrap()
}
