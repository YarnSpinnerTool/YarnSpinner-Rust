use crate::ExampleYarnSpinnerDialogueViewSystemSet;
use crate::setup::{DialogueNode, OptionButton, OptionsNode, UiRootNode, spawn_options};
use crate::typewriter::{self, Typewriter, TypewriterFinishedEvent};
use bevy::color::palettes::css;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};
use bevy_yarnspinner::{events::*, prelude::*};

pub(crate) fn option_selection_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            create_options.run_if(resource_added::<OptionSelection>),
            show_options,
            select_option
                .run_if(resource_exists::<OptionSelection>.and(any_with_component::<PrimaryWindow>))
                .before(typewriter::despawn),
        )
            .chain()
            .after(YarnSpinnerSystemSet)
            .in_set(ExampleYarnSpinnerDialogueViewSystemSet),
    );

    app.add_observer(despawn_options::<DialogueCompleted>);
    app.add_observer(despawn_options::<HasSelectedOptionEvent>);
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
    options_node: Single<(Entity, &mut Node, &mut Visibility), With<OptionsNode>>,
    mut root_visibility: Single<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
    typewriter: Res<Typewriter>,
) {
    let (entity, mut node, mut visibility) = options_node.into_inner();
    node.display = Display::Flex;
    if !typewriter.is_finished() {
        *visibility = Visibility::Hidden;
    }
    if children.iter_descendants(entity).next().is_none() {
        **root_visibility = Visibility::Inherited;
        let mut entity_commands = commands.entity(entity);
        spawn_options(&mut entity_commands, &option_selection.options);
    }
}

fn show_options(
    mut typewriter_finished_event: MessageReader<TypewriterFinishedEvent>,
    mut options_node: Single<&mut Visibility, With<OptionsNode>>,
) {
    for _event in typewriter_finished_event.read() {
        **options_node = Visibility::Inherited;
    }
}

fn select_option(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    typewriter: Res<Typewriter>,
    mut buttons: Query<(Entity, &Interaction, &OptionButton), (With<Button>, Changed<Interaction>)>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut text_writer: TextUiWriter,
    option_selection: Res<OptionSelection>,
    window: Single<Entity, With<PrimaryWindow>>,
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

    for (entity, interaction, button) in buttons.iter_mut() {
        let (color, icon) = match *interaction {
            Interaction::Pressed if selection.is_none() => {
                selection = Some(button.0);
                (css::TOMATO.into(), SystemCursorIcon::Default)
            }
            Interaction::Hovered => (Color::WHITE, SystemCursorIcon::Pointer),
            _ => (css::TOMATO.into(), SystemCursorIcon::Default),
        };
        commands.entity(*window).insert(CursorIcon::System(icon));
        *text_writer.color(entity, 2) = TextColor(color);
    }
    let has_selected_id = selection.is_some();
    if let Some(id) = selection {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            dialogue_runner.select_option(id).unwrap();
        }
    }
    if has_selected_id {
        commands.trigger(HasSelectedOptionEvent);
    }
}

fn despawn_options<T: Event>
(
    _ : On<T>,
    mut commands: Commands,
    options_node: Single<(Entity, &mut Node, &mut Visibility), With<OptionsNode>>,
    mut dialogue_node_text: Single<&mut Text, With<DialogueNode>>,
    mut root_visibility: Single<&mut Visibility, (With<UiRootNode>, Without<OptionsNode>)>,
) {
    commands.remove_resource::<OptionSelection>();
    let (entity, mut node, mut visibility) = options_node.into_inner();
    commands.entity(entity).despawn_related::<Children>();
    node.display = Display::None;
    *visibility = Visibility::Hidden;
    **dialogue_node_text = Text::default();
    **root_visibility = Visibility::Hidden;
}

const NUMBER_KEYS: [KeyCode; 9] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
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
