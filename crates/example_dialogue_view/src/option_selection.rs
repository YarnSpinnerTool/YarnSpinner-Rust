use crate::setup::{spawn_options, DialogueNode, OptionButton, OptionsNode, UiRootNode};
use crate::typewriter::{self, Typewriter, TypewriterFinishedEvent};
use crate::ExampleYarnSlingerDialogueViewSystemSet;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use bevy_yarn_slinger::{events::*, prelude::*};

pub(crate) fn option_selection_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            create_options.run_if(resource_added::<OptionSelection>()),
            show_options,
            select_option
                .run_if(resource_exists::<OptionSelection>())
                .before(typewriter::despawn),
            despawn_options,
        )
            .chain()
            .after(YarnSlingerSystemSet)
            .in_set(ExampleYarnSlingerDialogueViewSystemSet),
    )
    .add_event::<HasSelectedOptionEvent>();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Event)]
struct HasSelectedOptionEvent;

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

fn create_options(
    option_selection: Res<OptionSelection>,
    mut commands: Commands,
    children: Query<&Children>,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut root_visibility: Query<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
) {
    let (entity, mut style, mut visibility) = options_node.single_mut();
    style.display = Display::Flex;
    *visibility = Visibility::Hidden;
    if children.iter_descendants(entity).next().is_none() {
        *root_visibility.single_mut() = Visibility::Inherited;
        let mut entity_commands = commands.entity(entity);
        spawn_options(&mut entity_commands, &option_selection.options);
    }
}

fn show_options(
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>,
    mut options_node: Query<&mut Visibility, With<OptionsNode>>,
) {
    for _event in typewriter_finished_event.read() {
        let mut visibility = options_node.single_mut();
        *visibility = Visibility::Inherited;
    }
}

fn select_option(
    keys: Res<Input<KeyCode>>,
    typewriter: Res<Typewriter>,
    mut buttons: Query<
        (&Interaction, &OptionButton, &Children),
        (With<Button>, Changed<Interaction>),
    >,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut text: Query<&mut Text, Without<DialogueNode>>,
    option_selection: Res<OptionSelection>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut selected_option_event: EventWriter<HasSelectedOptionEvent>,
) {
    if !typewriter.is_finished() {
        return;
    }

    let mut selection = None;
    let key_to_option: HashMap<_, _> = NUMBER_KEYS
        .into_iter()
        .zip(NUMPAD_KEYS)
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
    if has_selected_id {
        selected_option_event.send(HasSelectedOptionEvent);
    }
}

fn despawn_options(
    mut has_selected_option_event: EventReader<HasSelectedOptionEvent>,
    mut dialogue_complete_event: EventReader<DialogueCompleteEvent>,
    mut commands: Commands,
    mut options_node: Query<(Entity, &mut Style, &mut Visibility), With<OptionsNode>>,
    mut dialogue_node_text: Query<&mut Text, With<DialogueNode>>,
    mut root_visibility: Query<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
) {
    let should_despawn =
        !has_selected_option_event.is_empty() || !dialogue_complete_event.is_empty();
    if !should_despawn {
        return;
    }
    has_selected_option_event.clear();
    dialogue_complete_event.clear();
    commands.remove_resource::<OptionSelection>();
    let (entity, mut style, mut visibility) = options_node.single_mut();
    commands.entity(entity).despawn_descendants();
    style.display = Display::None;
    *visibility = Visibility::Hidden;
    *dialogue_node_text.single_mut() = Text::default();
    *root_visibility.single_mut() = Visibility::Hidden;
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
