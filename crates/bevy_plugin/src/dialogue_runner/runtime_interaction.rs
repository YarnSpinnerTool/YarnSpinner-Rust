use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub(crate) fn runtime_interaction_plugin(app: &mut App) {
    app.add_system(continue_runtime.pipe(panic_on_err));
}

fn continue_runtime(
    mut dialogue_runners: Query<(Entity, &mut DialogueRunner)>,
    mut present_line_events: EventWriter<PresentLineEvent>,
    mut present_options_events: EventWriter<PresentOptionsEvent>,
    mut execute_command_events: EventWriter<ExecuteCommandEvent>,
    mut node_complete_events: EventWriter<NodeCompleteEvent>,
    mut node_start_events: EventWriter<NodeStartEvent>,
    mut line_hints_events: EventWriter<LineHintsEvent>,
    mut dialogue_complete_events: EventWriter<DialogueCompleteEvent>,
    mut last_options: Local<HashMap<Entity, Vec<DialogueOption>>>,
    asset_server: Res<AssetServer>,
) -> SystemResult {
    for (source, mut dialogue_runner) in dialogue_runners.iter_mut() {
        if !dialogue_runner.continue_ {
            continue;
        }
        if dialogue_runner.run_selected_options_as_lines {
            if let Some(option) = dialogue_runner.last_selected_option.take() {
                if let Some(mut options) = last_options.remove(&source) {
                    let Some(index) = options.iter().position(|o| o.id == option) else{
                        bail!("Dialogue options does not contain selected option. Expected one of {:?}, but found {option}", last_options.keys());
                    };
                    let option = options.swap_remove(index);
                    present_line_events.send(PresentLineEvent {
                        line: option.line,
                        source,
                    });
                }
            }
        }
        if let Some(asset_provider) = dialogue_runner.asset_provider.as_mut() {
            asset_provider.set_asset_server(asset_server.clone());
        }
        if let Some(events) = dialogue_runner.dialogue.continue_()? {
            for event in events {
                match event {
                    DialogueEvent::Line(line) => {
                        let assets = dialogue_runner.get_assets(&line);
                        present_line_events.send(PresentLineEvent {
                            line: LocalizedLine::from_yarn_line(line, assets),
                            source,
                        });
                    }
                    DialogueEvent::Options(options) => {
                        let options: Vec<DialogueOption> = options
                            .into_iter()
                            .map(|option| {
                                let assets = dialogue_runner.get_assets(&option.line);
                                DialogueOption::from_yarn_dialogue_option(option, assets)
                            })
                            .collect();
                        last_options.insert(source, options.clone());
                        present_options_events.send(PresentOptionsEvent { options, source });
                    }
                    DialogueEvent::Command(command) => {
                        execute_command_events.send(ExecuteCommandEvent { command, source });
                    }
                    DialogueEvent::NodeComplete(node_name) => {
                        node_complete_events.send(NodeCompleteEvent { node_name, source });
                    }
                    DialogueEvent::NodeStart(node_name) => {
                        node_start_events.send(NodeStartEvent { node_name, source });
                    }
                    DialogueEvent::LineHints(line_ids) => {
                        if let Some(asset_provider) = dialogue_runner.asset_provider.as_mut() {
                            asset_provider.accept_line_hints(&line_ids);
                        }

                        line_hints_events.send(LineHintsEvent { line_ids, source });
                    }
                    DialogueEvent::DialogueComplete => {
                        dialogue_complete_events.send(DialogueCompleteEvent { source });
                    }
                }
            }
        }
    }
    Ok(())
}
