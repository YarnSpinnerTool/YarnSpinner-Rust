use crate::example_ui::setup::{spawn_options, DialogueNode, OptionButton, OptionsNode};
use crate::example_ui::typewriter::Typewriter;
use crate::example_ui::updating::SpeakerChangeEvent;
use crate::prelude::{DialogueOption, DialogueRunner};
use bevy::prelude::*;

pub(crate) fn option_selection_plugin(app: &mut App) {
    app.add_systems((
        show_options
            .run_if(resource_exists::<Typewriter>().and_then(resource_exists::<OptionSelection>())),
        select_option
            .run_if(resource_exists::<Typewriter>().and_then(resource_exists::<OptionSelection>())),
    ));
}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub(crate) struct OptionSelection {
    options: Vec<DialogueOption>,
}

impl OptionSelection {
    pub fn from_option_set(options: Vec<DialogueOption>) -> Self {
        Self { options }
    }
}

fn show_options(
    typewriter: Res<Typewriter>,
    option_selection: Res<OptionSelection>,
    mut commands: Commands,
    children: Query<&Children>,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
) {
    let (entity, mut style, mut visibility) = options_node.single_mut();
    style.display = Display::Flex;
    if typewriter.is_finished() {
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }
    if children.iter_descendants(entity).next().is_none() {
        let mut entity_commands = commands.entity(entity);
        spawn_options(&mut entity_commands, &option_selection.options);

        if let Some(name) = typewriter.character_name.as_ref() {
            speaker_change_events.send(SpeakerChangeEvent {
                character_name: name.clone(),
                speaking: false,
            });
        }
    }
}

fn select_option(
    typewriter: Res<Typewriter>,
    mut commands: Commands,
    mut buttons: Query<
        (&Interaction, &OptionButton, &mut BackgroundColor),
        (With<Button>, Changed<Interaction>),
    >,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut text: Query<&mut Text, With<DialogueNode>>,
) {
    if !typewriter.is_finished() {
        return;
    }
    for (interaction, button, mut color) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                for mut dialogue_runner in dialogue_runners.iter_mut() {
                    dialogue_runner.select_option(button.0).unwrap();
                }
                commands.remove_resource::<OptionSelection>();
                *color = Color::NONE.into();
                let (entity, mut style, mut visibility) = options_node.single_mut();
                commands.entity(entity).despawn_descendants();
                style.display = Display::None;
                *visibility = Visibility::Hidden;
                *text.single_mut() = Text::default();
            }
            Interaction::Hovered => {
                *color = Color::ALICE_BLUE.into();
            }
            Interaction::None => {
                *color = Color::NONE.into();
            }
        }
    }
}
