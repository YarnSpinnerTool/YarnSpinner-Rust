use crate::option_selection::OptionSelection;
use crate::setup::{DialogueNameNode, UiRootNode};
use crate::typewriter::{self, Typewriter};
use crate::ExampleYarnSlingerDialogueViewSystemSet;
use bevy::prelude::*;
use bevy_yarn_slinger::{events::*, prelude::*};

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            hide_dialog.run_if(on_event::<DialogueCompleteEvent>()),
            show_dialog.run_if(on_event::<DialogueStartEvent>()),
            present_line
                .run_if(resource_exists::<Typewriter>().and_then(on_event::<PresentLineEvent>())),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue.run_if(resource_exists::<Typewriter>()),
        )
            .chain()
            .after(YarnSlingerSystemSet)
            .after(typewriter::spawn)
            .in_set(ExampleYarnSlingerDialogueViewSystemSet),
    )
    .add_event::<SpeakerChangeEvent>()
    .register_type::<SpeakerChangeEvent>();
}

/// Signals that a speaker has changed.
/// A speaker starts speaking when a new line is presented with a [`PresentLineEvent`] which has a character name.
/// A speaker stops speaking when the line is fully displayed on the screen, which happens over the course of a few seconds
#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
#[reflect(Debug, PartialEq, Hash)]
#[non_exhaustive]
pub struct SpeakerChangeEvent {
    /// The name of the character who is or was speaking.
    pub character_name: String,
    /// If `true`, the character just started speaking. Otherwise, they just stopped.
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
    mut name_node: Query<&mut Text, With<DialogueNameNode>>,
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
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.iter() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}

fn continue_dialogue(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut root_visibility: Query<&mut Visibility, With<UiRootNode>>,
) {
    let explicit_continue = keys.just_pressed(KeyCode::Space)
        || mouse_buttons.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();
    if explicit_continue && !typewriter.is_finished() {
        typewriter.fast_forward();
        return;
    }
    if (explicit_continue || typewriter.last_before_options) && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
                *root_visibility.single_mut() = Visibility::Hidden;
            }
        }
    }
}
