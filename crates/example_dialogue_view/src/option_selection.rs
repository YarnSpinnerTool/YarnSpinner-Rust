use crate::setup::{spawn_options, DialogueNode, OptionButton, OptionsNode, UiRootNode};
use crate::typewriter::{self, Typewriter};
use crate::ExampleYarnSlingerDialogueViewSystemSet;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_yarn_slinger::{events::*, prelude::*};

pub(crate) fn option_selection_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_options.run_if(
                resource_exists::<Typewriter>().and_then(resource_exists::<OptionSelection>()),
            ),
            select_option
                .run_if(
                    resource_exists::<Typewriter>().and_then(resource_exists::<OptionSelection>()),
                )
                .before(typewriter::despawn),
        )
            .chain()
            .after(YarnSlingerSystemSet)
            .in_set(ExampleYarnSlingerDialogueViewSystemSet),
    );
}

#[derive(Debug, Clone, PartialEq, Default, Resource)]
pub(crate) struct OptionSelection {
    options: Vec<DialogueOption>,
}

impl OptionSelection {
    pub fn from_option_set<'a>(options: impl IntoIterator<Item = &'a DialogueOption>) -> Self {
        let options = options
            .into_iter()
            .filter(|o| o.is_available)
            .cloned()
            .collect();
        Self { options }
    }
}

fn show_options(
    typewriter: Res<Typewriter>,
    option_selection: Res<OptionSelection>,
    mut commands: Commands,
    children: Query<&Children>,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut root_visibility: Query<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
) {
    let (entity, mut style, mut visibility) = options_node.single_mut();
    style.display = Display::Flex;
    if typewriter.is_finished() {
        *visibility = Visibility::Inherited;
    } else {
        *visibility = Visibility::Hidden;
    }
    if children.iter_descendants(entity).next().is_none() {
        *root_visibility.single_mut() = Visibility::Inherited;
        let mut entity_commands = commands.entity(entity);
        spawn_options(&mut entity_commands, &option_selection.options);
    }
}

fn select_option(
    line_events: EventReader<PresentLineEvent>,
    dialogue_complete_events: EventReader<DialogueCompleteEvent>,
    keys: Res<Input<KeyCode>>,
    typewriter: Res<Typewriter>,
    mut commands: Commands,
    mut buttons: Query<
        (&Interaction, &OptionButton, &Children),
        (With<Button>, Changed<Interaction>),
    >,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut dialogue_node_text: Query<&mut Text, With<DialogueNode>>,
    mut text: Query<&mut Text, Without<DialogueNode>>,
    option_selection: Res<OptionSelection>,
    mut root_visibility: Query<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !typewriter.is_finished() {
        return;
    }

    let mut selection = None;
    let key_to_option: HashMap<_, _> = NUMBER_KEYS
        .into_iter()
        .zip(NUMPAD_KEYS.into_iter())
        .zip(option_selection.options.iter().map(|option| option.id))
        .collect();
    for ((num_key, numpad_key), option) in key_to_option {
        if keys.just_pressed(num_key) || keys.just_pressed(numpad_key) {
            selection = Some(option);
            break;
        }
    }
    let mut window = windows.single_mut();
    for (interaction, button, children) in buttons.iter_mut() {
        let (color, icon) = match *interaction {
            Interaction::Pressed if selection.is_none() => {
                selection = Some(button.0);
                (Color::TOMATO, CursorIcon::Default)
            }
            Interaction::Hovered => (Color::WHITE, CursorIcon::Hand),
            _ => (Color::TOMATO, CursorIcon::Default),
        };
        window.cursor.icon = icon;
        let text_entity = children.iter().find(|&e| text.contains(*e)).unwrap();
        let mut text = text.get_mut(*text_entity).unwrap();
        text.sections[1].style.color = color;
    }
    let has_selected_id = selection.is_some();
    if let Some(id) = selection {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            dialogue_runner.select_option(id).unwrap();
        }
    }
    let should_despawn =
        has_selected_id || !line_events.is_empty() || !dialogue_complete_events.is_empty();
    if should_despawn {
        commands.remove_resource::<OptionSelection>();
        let (entity, mut style, mut visibility) = options_node.single_mut();
        commands.entity(entity).despawn_descendants();
        style.display = Display::None;
        *visibility = Visibility::Hidden;
        *dialogue_node_text.single_mut() = Text::default();
        *root_visibility.single_mut() = Visibility::Hidden;
    }
}

const NUMBER_KEYS: [KeyCode; 9] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
];

const NUMPAD_KEYS: [KeyCode; 9] = [
    KeyCode::Numpad1,
    KeyCode::Numpad2,
    KeyCode::Numpad3,
    KeyCode::Numpad4,
    KeyCode::Numpad5,
    KeyCode::Numpad6,
    KeyCode::Numpad7,
    KeyCode::Numpad8,
    KeyCode::Numpad9,
];
