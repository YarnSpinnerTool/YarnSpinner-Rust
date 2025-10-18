use crate::ExampleYarnSpinnerDialogueViewSystemSet;
use crate::option_selection::OptionSelection;
use crate::setup::{DialogueContinueNode, DialogueNameNode, UiRootNode};
use crate::typewriter::{self, Typewriter};
use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        Update,
        continue_dialogue
            .run_if(resource_exists::<Typewriter>)
            .after(YarnSpinnerSystemSet)
            .after(typewriter::spawn)
            .in_set(ExampleYarnSpinnerDialogueViewSystemSet),
    )
    .add_message::<SpeakerChangeEvent>()
    .register_type::<SpeakerChangeEvent>();

    app.add_observer(show_dialog);
    app.add_observer(hide_dialog);
    app.add_observer(present_line);
    app.add_observer(present_options);
}

/// Signals that a speaker has changed.
/// A speaker starts speaking when a new line is presented with a [`PresentLine`] event which has a character name.
/// A speaker stops speaking when the line is fully displayed on the screen, which happens over the course of a few seconds
#[derive(Debug, Eq, PartialEq, Hash, Reflect, Message)]
#[reflect(Debug, PartialEq, Hash)]
#[non_exhaustive]
pub struct SpeakerChangeEvent {
    /// The name of the character who is or was speaking.
    pub character_name: String,
    /// If `true`, the character just started speaking. Otherwise, they just stopped.
    pub speaking: bool,
}

fn show_dialog(_: On<DialogueStarted>, mut visibility: Single<&mut Visibility, With<UiRootNode>>) {
    **visibility = Visibility::Inherited;
}

fn hide_dialog(
    _: On<DialogueCompleted>,
    mut root_visibility: Single<&mut Visibility, With<UiRootNode>>,
) {
    **root_visibility = Visibility::Hidden;
}

fn present_line(
    event: On<PresentLine>,
    mut speaker_change_events: MessageWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
    name_node: Single<Entity, With<DialogueNameNode>>,
    mut text_writer: TextUiWriter,
) {
    let name = if let Some(name) = event.line.character_name() {
        speaker_change_events.write(SpeakerChangeEvent {
            character_name: name.to_string(),
            speaking: true,
        });
        name.to_string()
    } else {
        String::new()
    };
    *text_writer.text(*name_node, 0) = name;
    typewriter.set_line(&event.line);
}

fn present_options(event: On<PresentOptions>, mut commands: Commands) {
    let option_selection = OptionSelection::from_option_set(&event.options);
    commands.insert_resource(option_selection);
}

fn continue_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut root_visibility: Single<&mut Visibility, With<UiRootNode>>,
    mut continue_visibility: Single<
        &mut Visibility,
        (With<DialogueContinueNode>, Without<UiRootNode>),
    >,
) {
    let explicit_continue = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
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
                **root_visibility = Visibility::Hidden;
                **continue_visibility = Visibility::Hidden;
            }
        }
    }
}
