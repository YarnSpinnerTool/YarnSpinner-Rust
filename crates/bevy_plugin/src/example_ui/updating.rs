use crate::example_ui::option_selection::OptionSelection;
use crate::example_ui::setup::UiRootNode;
use crate::example_ui::typewriter::Typewriter;
use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_systems(
        (
            show_dialog.run_if(on_event::<DialogueStartEvent>()),
            hide_dialog.run_if(on_event::<DialogueCompleteEvent>()),
            present_line
                .run_if(resource_exists::<Typewriter>().and_then(on_event::<PresentLineEvent>())),
            present_options.run_if(on_event::<PresentOptionsEvent>()),
            continue_dialogue.run_if(
                resource_exists::<Typewriter>().and_then(not(resource_exists::<OptionSelection>())),
            ),
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

fn show_dialog(mut commands: Commands, mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    commands.init_resource::<Typewriter>();
    *visibility.single_mut() = Visibility::Visible;
}

fn hide_dialog(mut commands: Commands, mut visibility: Query<&mut Visibility, With<UiRootNode>>) {
    commands.remove_resource::<Typewriter>();
    *visibility.single_mut() = Visibility::Hidden;
}

fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
) {
    for event in line_events.iter() {
        if let Some(name) = event.line.character_name() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
        }
        typewriter.set_line(&event.line);
    }
}

fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for _event in events.iter() {
        commands.insert_resource(OptionSelection {});
    }
}

fn continue_dialogue(
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter: ResMut<Typewriter>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
) {
    if keys.just_pressed(KeyCode::Space) || mouse_buttons.just_pressed(MouseButton::Left) {
        if !typewriter.is_finished() {
            typewriter.fast_forward();
            return;
        }
        if let Some(name) = typewriter.character_name.as_ref() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.clone(),
                speaking: false,
            });
        }
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            dialogue_runner.continue_in_next_update();
        }
    }
}
