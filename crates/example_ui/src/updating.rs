use crate::option_selection::OptionSelection;
use crate::setup::{DialogueNameNode, UiRootNode};
use crate::typewriter::Typewriter;
use crate::ExampleYarnSlingerUiSystemSet;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        (
            show_dialog.run_if(on_event::<DialogueStartEvent>()),
            hide_dialog.run_if(on_event::<DialogueCompleteEvent>()),
            present_line
                .run_if(resource_exists::<Typewriter>().and_then(on_event::<PresentLineEvent>())),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue.run_if(resource_exists::<Typewriter>()),
            hide_on_wait.run_if(on_event::<ExecuteCommandEvent>()),
        )
            .chain()
            .after(YarnSlingerSystemSet)
            .in_set(ExampleYarnSlingerUiSystemSet),
    )
    .add_event::<SpeakerChangeEvent>();
}

pub struct SpeakerChangeEvent {
    pub character_name: String,
    pub speaking: bool,
}

fn show_dialog(mut commands: Commands, mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    commands.init_resource::<Typewriter>();
    *visibility.single_mut() = Visibility::Inherited;
}

fn hide_dialog(
    mut commands: Commands,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
) {
    commands.remove_resource::<Typewriter>();
    *root_visibility.single_mut() = Visibility::Hidden;
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
    mut name_node: Query<&mut Text, With<DialogueNameNode>>,
) {
    for event in line_events.iter() {
        if let Some(name) = event.line.character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
            name_node.single_mut().sections[0].value = name.to_string();
        } else {
            name_node.single_mut().sections[0].value = String::new();
        }
        typewriter.set_line(&event.line);
    }
    *root_visibility.single_mut() = Visibility::Inherited;
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.iter() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

fn hide_on_wait(
    mut events: EventReader<ExecuteCommandEvent>,
    mut visibility: Query<&mut Visibility, With<UiRootNode>>,
) {
    if events.iter().any(|event| event.command.name == "wait") {
        *visibility.single_mut() = Visibility::Hidden;
    }
}

fn continue_dialogue(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
) {
    let explicit_continue =
        keys.just_pressed(KeyCode::Space) || mouse_buttons.just_pressed(MouseButton::Left);
    if explicit_continue && !typewriter.is_finished() {
        typewriter.fast_forward();
        return;
    }
    if (explicit_continue || typewriter.last_before_options) && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
            }
        }
    }
}
