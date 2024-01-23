use crate::commands::update_wait;
use crate::dialogue_runner::events::DialogueStartEvent;
use crate::events::*;
use crate::line_provider::LineProviderSystemSet;
use crate::prelude::*;
use anyhow::bail;
use bevy::asset::LoadedUntypedAsset;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub(crate) fn runtime_interaction_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            continue_runtime
                .pipe(panic_on_err)
                .run_if(resource_exists::<YarnProject>()),
            accept_line_hints,
        )
            .chain()
            .after(LineProviderSystemSet)
            .after(update_wait)
            .in_set(DialogueExecutionSystemSet)
            .in_set(YarnSlingerSystemSet),
    );
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) struct DialogueExecutionSystemSet;

fn continue_runtime(
    mut dialogue_runners: Query<(Entity, &mut DialogueRunner)>,
    mut present_line_events: EventWriter<PresentLineEvent>,
    mut present_options_events: EventWriter<PresentOptionsEvent>,
    mut execute_command_events: EventWriter<ExecuteCommandEvent>,
    mut node_complete_events: EventWriter<NodeCompleteEvent>,
    mut node_start_events: EventWriter<NodeStartEvent>,
    mut line_hints_events: EventWriter<LineHintsEvent>,
    mut dialogue_complete_events: EventWriter<DialogueCompleteEvent>,
    mut dialogue_start_events: EventWriter<DialogueStartEvent>,
    mut last_options: Local<HashMap<Entity, Vec<DialogueOption>>>,
    loaded_untyped_assets: Res<Assets<LoadedUntypedAsset>>,
    project: Res<YarnProject>,
) -> SystemResult {
    for (source, mut dialogue_runner) in dialogue_runners.iter_mut() {
        let is_sending_missed_events = !dialogue_runner.unsent_events.is_empty();
        if !is_sending_missed_events {
            if dialogue_runner.just_started {
                dialogue_start_events.send(DialogueStartEvent { source });
                dialogue_runner.just_started = false;
            }
            if !dialogue_runner.is_running {
                dialogue_runner.will_continue_in_next_update = false;
                continue;
            }

            if let Some(line_ids) = std::mem::take(&mut dialogue_runner.popped_line_hints) {
                line_hints_events.send(LineHintsEvent { line_ids, source });
            }

            if !(dialogue_runner.will_continue_in_next_update
                && dialogue_runner.poll_tasks_and_check_if_done()
                && dialogue_runner.are_lines_available(&loaded_untyped_assets))
            {
                continue;
            }
            dialogue_runner.will_continue_in_next_update = false;

            if dialogue_runner.run_selected_options_as_lines {
                if let Some(option) = dialogue_runner.last_selected_option.take() {
                    let options = last_options
                        .remove(&source)
                        .expect("Failed to get last presented options when trying to run selected option as line. \
                                  This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new");
                    let Some(option) = options.into_iter().find(|o| o.id == option) else {
                        let expected_options = last_options
                            .values()
                            .flat_map(|options| options.iter().map(|option| option.id.to_string()))
                            .collect::<Vec<_>>()
                            .join(", ");
                        bail!("Dialogue options does not contain selected option. Expected one of [{expected_options}], but found {option}");
                    };
                    present_line_events.send(PresentLineEvent {
                        line: option.line,
                        source,
                    });
                    continue;
                }
            }
        }
        let events = if is_sending_missed_events {
            std::mem::take(&mut dialogue_runner.unsent_events)
        } else {
            dialogue_runner.dialogue.continue_()?
        };

        for event in events {
            match event {
                DialogueEvent::Line(line) => {
                    let assets = dialogue_runner.get_assets(&line, &loaded_untyped_assets);
                    let metadata = project.line_metadata(&line.id).unwrap_or_default().to_vec();
                    present_line_events.send(PresentLineEvent {
                        line: LocalizedLine::from_yarn_line(line, assets, metadata),
                        source,
                    });
                }
                DialogueEvent::Options(options) => {
                    let options: Vec<DialogueOption> = options
                        .into_iter()
                        .map(|option| {
                            let assets =
                                dialogue_runner.get_assets(&option.line, &loaded_untyped_assets);
                            let metadata = project
                                .line_metadata(&option.line.id)
                                .unwrap_or_default()
                                .to_vec();
                            DialogueOption::from_yarn_dialogue_option(option, assets, metadata)
                        })
                        .collect();
                    last_options.insert(source, options.clone());
                    present_options_events.send(PresentOptionsEvent { options, source });
                }
                DialogueEvent::Command(command) => {
                    execute_command_events.send(ExecuteCommandEvent { command, source });
                    dialogue_runner.continue_in_next_update();
                }
                DialogueEvent::NodeComplete(node_name) => {
                    node_complete_events.send(NodeCompleteEvent { node_name, source });
                }
                DialogueEvent::NodeStart(node_name) => {
                    node_start_events.send(NodeStartEvent { node_name, source });
                }
                DialogueEvent::LineHints(line_ids) => {
                    line_hints_events.send(LineHintsEvent { line_ids, source });
                }
                DialogueEvent::DialogueComplete => {
                    if !is_sending_missed_events {
                        dialogue_runner.is_running = false;
                    }
                    dialogue_complete_events.send(DialogueCompleteEvent { source });
                }
            }
        }
    }
    Ok(())
}

fn accept_line_hints(
    mut events: EventReader<LineHintsEvent>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
) {
    for event in events.read() {
        let mut dialogue_runner = dialogue_runners.get_mut(event.source).unwrap();
        for asset_provider in dialogue_runner.asset_providers.values_mut() {
            asset_provider.accept_line_hints(&event.line_ids);
        }
    }
}
