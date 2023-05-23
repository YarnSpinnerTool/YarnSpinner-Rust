use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn runtime_interaction_plugin(app: &mut App) {
    app.add_system(continue_runtime.pipe(panic_on_err));
}

fn continue_runtime(mut dialogue_runners: Query<(Entity, &mut DialogueRunner)>) -> SystemResult {
    for (_entity, mut dialogue_runner) in dialogue_runners.iter_mut() {
        if !dialogue_runner.continue_ {
            continue;
        }
        if let Some(events) = dialogue_runner.dialogue.continue_()? {
            for event in events {
                match event {
                    DialogueEvent::Line(_line) => {}
                    DialogueEvent::Options(_options) => {}
                    DialogueEvent::Command(_command) => {}
                    DialogueEvent::NodeComplete(_node_name) => {}
                    DialogueEvent::NodeStart(_node_name) => {}
                    DialogueEvent::LineHints(_line_ids) => {}
                    DialogueEvent::DialogueComplete => {}
                }
                todo!()
            }
        }
    }
    Ok(())
}
