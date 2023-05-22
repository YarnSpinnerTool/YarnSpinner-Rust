use crate::prelude::*;
use crate::project::*;
use bevy::prelude::*;

pub(crate) fn runtime_interaction_plugin(app: &mut App) {
    app.add_system(set_dialogue_programs.run_if(resource_exists::<YarnCompilation>()));
}

fn continue_runtime(mut dialogue_runners: Query<&mut DialogueRunner>) -> SystemResult {
    for dialogue_runner in dialogue_runners.iter_mut() {
        if !dialogue_runner.continue_ {
            continue;
        }
        if let Some(dialogue) = &mut dialogue_runner.dialogue {
            if let Some(events) = dialogue.continue_()? {
                for event in events {
                    match event {
                        DialogueEvent::Line(line) => {}
                        DialogueEvent::Options(options) => {}
                        DialogueEvent::Command(command) => {}
                        DialogueEvent::NodeComplete(node_name) => {}
                        DialogueEvent::NodeStart(node_name) => {}
                        DialogueEvent::LineHints(line_ids) => {}
                        DialogueEvent::DialogueComplete => {}
                    }
                    todo!()
                }
            }
        }
    }
    Ok(())
}

fn set_dialogue_programs(
    mut dialogue_runners: Query<&mut DialogueRunner>,
    global_variable_storage: Res<GlobalVariableStorage>,
    global_text_provider: Res<GlobalTextProvider>,
    global_line_asset_provider: Option<Res<GlobalLineAssetProvider>>,
    global_library: Res<GlobalYarnFnLibrary>,
    yarn_compilation: Res<YarnCompilation>,
) {
    let compilation_changed = yarn_compilation.is_changed();
    let dialogue_runners = dialogue_runners
        .iter_mut()
        .filter(|runner| compilation_changed || runner.dialogue.is_none());
    for mut dialogue_runner in dialogue_runners {
        let local_library = dialogue_runner.library_buffer.take();

        let dialogue = if let Some(dialogue) = &mut dialogue_runner.dialogue {
            dialogue
        } else {
            let text_provider = dialogue_runner
                .text_provider_override
                .as_ref()
                .map(|provider| provider.clone_shallow())
                .unwrap_or_else(|| global_text_provider.0.clone_shallow());
            let variable_storage = dialogue_runner
                .variable_storage_override
                .as_ref()
                .map(|storage| storage.clone_shallow())
                .unwrap_or_else(|| global_variable_storage.0.clone_shallow());
            if dialogue_runner.line_asset_provider_override.is_none() {
                let line_asset_provider = global_line_asset_provider
                    .as_ref()
                    .map(|provider| provider.0.clone_shallow());
                dialogue_runner.line_asset_provider_override = Some(line_asset_provider);
            }
            dialogue_runner.dialogue = Some(Dialogue::new(variable_storage, text_provider));
            dialogue_runner.dialogue.as_mut().unwrap()
        };
        dialogue.replace_program(yarn_compilation.0.program.clone().unwrap());

        dialogue.library_mut().extend(global_library.0.clone());
        if let Some(library) = local_library {
            dialogue.library_mut().extend(library);
        }
    }
}
