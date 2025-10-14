use crate::commands::update_wait;
use crate::dialogue_runner::events::DialogueStarted;
use crate::events::*;
use crate::line_provider::LineProviderSystemSet;
use crate::prelude::*;
use anyhow::{bail, Ok};
use bevy::asset::LoadedUntypedAsset;
use bevy::ecs::system::SystemState;
use bevy::platform::{collections::HashMap, hash::FixedHasher};
use bevy::prelude::*;

pub(crate) fn runtime_interaction_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            continue_runtime
                .pipe(panic_on_err)
                .run_if(resource_exists::<YarnProject>),
        )
            .chain()
            .after(LineProviderSystemSet)
            .after(update_wait)
            .in_set(DialogueExecutionSystemSet)
            .in_set(YarnSpinnerSystemSet),
    );
    
    app.add_observer(accept_line_hints);
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) struct DialogueExecutionSystemSet;

fn continue_runtime(
    world: &mut World,
    mut last_options: Local<HashMap<Entity, Vec<DialogueOption>>>,
) -> SystemResult {
    let mut system_state: SystemState<(
        Query<(Entity, &mut DialogueRunner)>,
        Res<Assets<LoadedUntypedAsset>>,
        Commands,
    )> = SystemState::new(world);

    let (
        mut dialogue_runners,
        loaded_untyped_assets,
        mut commands,
    ) = system_state.get_mut(world);

    let mut dialogues: HashMap<_, _, FixedHasher> = HashMap::default();

    for (source, mut dialogue_runner) in dialogue_runners.iter_mut() {
        let is_sending_missed_events: bool = !dialogue_runner.unsent_events.is_empty();
        if !is_sending_missed_events {
            if dialogue_runner.just_started {
                commands.trigger(DialogueStarted { entity: source });
                dialogue_runner.just_started = false;
            }
            if !dialogue_runner.is_running {
                dialogue_runner.will_continue_in_next_update = false;
                continue;
            }

            if let Some(line_ids) = std::mem::take(&mut dialogue_runner.popped_line_hints) {
                commands.trigger(LineHints { line_ids, entity: source });
            }

            if !(dialogue_runner.will_continue_in_next_update
                && dialogue_runner.poll_tasks_and_check_if_done()
                && dialogue_runner.update_line_availability(&loaded_untyped_assets))
            {
                continue;
            }
            dialogue_runner.will_continue_in_next_update = false;

            if dialogue_runner.run_selected_options_as_lines
                && let Some(option) = dialogue_runner.last_selected_option.take()
            {
                let options = last_options
                        .remove(&source)
                        .expect_or_bug("Failed to get last presented options when trying to run selected option as line.");
                let Some(option) = options.into_iter().find(|o| o.id == option) else {
                    let expected_options = last_options
                        .values()
                        .flat_map(|options| options.iter().map(|option| option.id.to_string()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    bail!(
                        "Dialogue options does not contain selected option. Expected one of [{expected_options}], but found {option}"
                    );
                };
                commands.trigger(PresentLine {
                    line: option.line,
                    entity: source,
                });
                continue;
            }
        }

        let unsent_events = if is_sending_missed_events {
            Some(std::mem::take(&mut dialogue_runner.unsent_events))
        } else {
            None
        };
        dialogues.insert(
            source,
            (
                dialogue_runner.dialogue.take().unwrap(),
                is_sending_missed_events,
                unsent_events,
                None::<Vec<DialogueEvent>>,
            ),
        );
    }

    for (dialogue, _, unsent_events, events) in dialogues.values_mut() {
        let new_events = if let Some(unsent_events) = unsent_events.take() {
            unsent_events
        } else {
            dialogue.continue_with_world(world)?
        };
        events.replace(new_events);
    }
    system_state.apply(world);

    let mut system_state: SystemState<(
        Query<(Entity, &mut DialogueRunner)>,
        Res<YarnProject>,
        Commands,
    )> = SystemState::new(world);

    let (
        mut dialogue_runners,
        project,
        mut commands,
    ) = system_state.get_mut(world);

    for (source, mut dialogue_runner) in dialogue_runners.iter_mut() {
        if let Some((dialogue, is_sending_missed_events, _, Some(events))) =
            dialogues.remove(&source)
        {
            dialogue_runner.dialogue.replace(dialogue);
            for event in events {
                match event {
                    DialogueEvent::Line(line) => {
                        let assets = dialogue_runner.get_assets(&line);
                        let metadata = project.line_metadata(&line.id).unwrap_or_default().to_vec();
                        commands.trigger( PresentLine {
                            line: LocalizedLine::from_yarn_line(line, assets, metadata),
                            entity: source,
                        });
                    }
                    DialogueEvent::Options(options) => {
                        let options: Vec<DialogueOption> = options
                            .into_iter()
                            .map(|option| {
                                let assets = dialogue_runner.get_assets(&option.line);
                                let metadata = project
                                    .line_metadata(&option.line.id)
                                    .unwrap_or_default()
                                    .to_vec();
                                DialogueOption::from_yarn_dialogue_option(option, assets, metadata)
                            })
                            .collect();
                        last_options.insert(source, options.clone());
                        commands.trigger(PresentOptions { options, entity: source });
                    }
                    DialogueEvent::Command(command) => {
                        commands.trigger(ExecuteCommand { command, entity: source });
                        dialogue_runner.continue_in_next_update();
                    }
                    DialogueEvent::NodeComplete(node_name) => {
                        commands.trigger(NodeCompleted { node_name, entity: source });
                    }
                    DialogueEvent::NodeStart(node_name) => {
                        commands.trigger(NodeStarted { node_name, entity: source });
                    }
                    DialogueEvent::LineHints(line_ids) => {
                        commands.trigger(LineHints { line_ids, entity: source });
                    }
                    DialogueEvent::DialogueComplete => {
                        if !is_sending_missed_events {
                            dialogue_runner.is_running = false;
                        }
                        commands.trigger(DialogueCompleted { entity: source });
                    }
                }
            }
        }
    }
    system_state.apply(world);
    Ok(())
}

fn accept_line_hints(
    event: On<LineHints>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
) {
    let mut dialogue_runner = dialogue_runners.get_mut(event.entity).unwrap();
    for asset_provider in dialogue_runner.asset_providers.values_mut() {
        asset_provider.accept_line_hints(&event.line_ids);
    }
}
