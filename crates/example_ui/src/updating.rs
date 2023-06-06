use crate::option_selection::OptionSelection;
use crate::setup::{DialogueNameNode, DialogueNode, UiRootNode};
use crate::typewriter::{self, Typewriter};
use crate::ExampleYarnSlingerUiSystemSet;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        (
            hide_dialog.run_if(on_event::<DialogueCompleteEvent>()),
            show_dialog.run_if(on_event::<DialogueStartEvent>()),
            present_line
                .run_if(resource_exists::<Typewriter>().and_then(on_event::<PresentLineEvent>())),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue.run_if(resource_exists::<Typewriter>()),
            hide_on_wait.run_if(on_event::<ExecuteCommandEvent>()),
        )
            .chain()
            .after(YarnSlingerSystemSet)
            .after(typewriter::spawn)
            .in_set(ExampleYarnSlingerUiSystemSet),
    )
    .add_event::<SpeakerChangeEvent>();
}

pub struct SpeakerChangeEvent {
    pub character_name: String,
    pub speaking: bool,
}

fn show_dialog(mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *visibility.single_mut() = Visibility::Inherited;
}

fn hide_dialog(mut root_visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *root_visibility.single_mut() = Visibility::Hidden;
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
    mut name_node: Query<&mut Text, With<DialogueNameNode>>,
    mut dialogue_node_text: Query<&mut Text, (With<DialogueNode>, Without<DialogueNameNode>)>,
) {
    for event in line_events.iter() {
        let name = if let Some(name) = event.line.character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
            name.to_string()
        } else {
            String::new()
        };
        name_node.single_mut().sections[0].value = name;
        typewriter.set_line(&event.line);
    }
    *root_visibility.single_mut() = Visibility::Inherited;
    *dialogue_node_text.single_mut() = Text::default();
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
