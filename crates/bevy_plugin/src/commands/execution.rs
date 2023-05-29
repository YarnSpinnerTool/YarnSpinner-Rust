use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn command_execution_plugin(_app: &mut App) {}


fn execute_commands(
    mut events: EventReader<ExecuteCommandEvent>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
) {
    for event in events.iter() {
        let dialogue_runner = dialogue_runners.get_mut(event.source).unwrap();
        todo!()
    }
}