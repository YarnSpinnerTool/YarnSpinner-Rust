use crate::example_ui::setup::{DialogueNode, UiRootNode};
use crate::example_ui::typewriter::TypeWrittenText;
use crate::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::utils::Instant;

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        (
            show_dialog.run_if(on_event::<DialogueStartEvent>()),
            hide_dialog.run_if(on_event::<DialogueCompleteEvent>()),
            present_line.run_if(on_event::<PresentLineEvent>()),
        )
            .chain()
            .after(YarnSlingerSystemSet),
    )
    .add_event::<SpeakerChangeEvent>();
}

pub struct SpeakerChangeEvent {
    pub character_name: String,
    pub speaking: bool,
}

fn show_dialog(mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *visibility.single_mut() = Visibility::Visible;
}

fn hide_dialog(mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    *visibility.single_mut() = Visibility::Hidden;
}

fn present_line(
    mut commands: Commands,
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
) {
    for event in line_events.iter() {
        if let Some(name) = event.line.text_without_character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
        }
        commands.insert_resource(TypeWrittenText {
            character_name: event.line.character_name().map(|s| s.to_string()),
            line: event.line.text_without_character_name(),
            start: Instant::now(),
        });
    }
}

fn present_options(
    mut commands: Commands,
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
) {
    for event in line_events.iter() {
        if let Some(name) = event.line.text_without_character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
        }
        commands.insert_resource(TypeWrittenText {
            character_name: event.line.character_name().map(|s| s.to_string()),
            line: event.line.text_without_character_name(),
            start: Instant::now(),
            last_before_options: event.line.attributes,
        });
    }
}

fn continue_dialogue(
    keys: Res<Input<KeyCode>>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    type_written_text: Option<Res<TypeWrittenText>>,
    mut commands: Commands,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            dialogue_runner.continue_in_next_update();
        }
        if let Some(type_written_text) = type_written_text {
            if let Some(name) = type_written_text.character_name.as_ref() {
                speaker_change_events.send(SpeakerChangeEvent {
                    character_name: name.clone(),
                    speaking: false,
                });
            }
            if !type_written_text.last_before_options {
                commands.remove_resource::<TypeWrittenText>();
            }
        }
    }
}
