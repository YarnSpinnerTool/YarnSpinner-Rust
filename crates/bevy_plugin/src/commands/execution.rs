use crate::commands::UntypedYarnCommand;
use crate::events::ExecuteCommand;
use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn command_execution_plugin(app: &mut App) {
    app.add_observer(execute_commands);
}

fn execute_commands(
        event: On<ExecuteCommand>, 
        dialogue_runners: Query<&DialogueRunner>,
        mut commands: Commands
) {
    let dialogue_runner = dialogue_runners.get(event.entity).unwrap();
    let Some(mut command) = clone_command(dialogue_runner, &event) else {
        return;
    };
    let params = event.command.parameters.clone();
    let entity = event.entity;
    commands.queue(move |world: &mut World| {
        let task_finished_indicator = command.call(params.clone(), world);
        if !task_finished_indicator.is_finished() {
            get_dialogue_runner_mut(world, entity).add_command_task(task_finished_indicator);
        }
    });
}

fn clone_command(
    dialogue_runner: &DialogueRunner,
    event: &ExecuteCommand,
) -> Option<Box<dyn UntypedYarnCommand>> {
    let command_name = event.command.name.as_str();
    dialogue_runner
        .commands
        .get(command_name)
        .map(|command| command.clone_box())
}


fn get_dialogue_runner_mut(world: &mut World, entity: Entity) -> Mut<'_, DialogueRunner> {
    let mut dialogue_runners = world.query::<&mut DialogueRunner>();

    dialogue_runners.get_mut(world, entity).unwrap()
}
