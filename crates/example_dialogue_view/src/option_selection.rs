use crate::ExampleYarnSpinnerDialogueViewSystemSet;
use crate::setup::{DialogueNode, OptionButton, OptionsNode, UiRootNode, spawn_options};
use crate::typewriter::{Typewriter, TypewriterFinishedEvent};
use bevy::color::palettes::css;
use bevy::picking::hover::Hovered;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::text::TextLayoutInfo;
use bevy::ui::Pressed;
use bevy::ui_widgets::{Activate, Button, ButtonPlugin};
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};
use bevy_yarnspinner::{events::*, prelude::*};

pub(crate) fn option_selection_plugin(app: &mut App) {
    if !app.is_plugin_added::<ButtonPlugin>() {
        app.add_plugins(ButtonPlugin);
    }
    app.add_observer(activate_option);
    app.add_systems(
        Update,
        (
            create_options.run_if(resource_added::<OptionSelection>),
            hack_sync_option_text_min_height.run_if(resource_exists::<OptionSelection>),
            show_options,
            select_option.run_if(
                resource_exists::<OptionSelection>.and_then(any_with_component::<PrimaryWindow>),
            ),
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
    selected: Option<OptionId>,
}

impl OptionSelection {
    pub fn from_option_set<'a>(options: impl IntoIterator<Item = &'a DialogueOption>) -> Self {
        let options = options
            .into_iter()
            .filter(|o| o.is_available)
            .cloned()
            .collect();
        Self {
            options,
            selected: None,
        }
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

/// Looks like something is layouting the box for just one line instead of multiple ones
/// when text is wrapped. So let's force sync stuff.
fn hack_sync_option_text_min_height(
    mut option_texts: Query<(&TextLayoutInfo, &mut Node), With<OptionButton>>,
) {
    for (layout_info, mut node) in &mut option_texts {
        let min_height = Val::Px(layout_info.size.y);
        if node.min_height != min_height {
            node.min_height = min_height;
        }
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
    buttons: Query<(Entity, Option<&Hovered>, Has<Pressed>), With<OptionButton>>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut text_writer: TextUiWriter,
    mut option_selection: ResMut<OptionSelection>,
    window: Single<Entity, With<PrimaryWindow>>,
) {
    if !typewriter.is_finished() {
        return;
    }

    let mut selection = option_selection.selected.take();
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

    let mut cursor = SystemCursorIcon::Default;
    for (entity, hovered, pressed) in &buttons {
        let is_hovered = hovered.is_some_and(|hovered| hovered.0);
        let color = if is_hovered && !pressed {
            Color::WHITE
        } else {
            css::TOMATO.into()
        };
        if is_hovered {
            cursor = SystemCursorIcon::Pointer;
        }
        *text_writer.color(entity, 2) = TextColor(color);
    }
    commands.entity(*window).insert(CursorIcon::System(cursor));
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

fn activate_option(
    event: On<Activate>,
    buttons: Query<&OptionButton, With<Button>>,
    selection: Option<ResMut<OptionSelection>>,
    typewriter: Option<Res<Typewriter>>,
) {
    // Activate is global: title/phone widgets can fire it outside a dialogue.
    // Typewriter is created on DialogueStarted and removed on DialogueCompleted.
    if let Ok(button) = buttons.get(event.entity)
        && let Some(mut selection) = selection
        && let Some(typewriter) = typewriter
        && typewriter.is_finished()
    {
        selection.selected.get_or_insert(button.0);
    }
}

fn despawn_options<T: Event>(
    _: On<T>,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn app(finished: bool) -> (App, Entity) {
        let mut app = App::new();
        let mut typewriter = Typewriter::default();
        if finished {
            typewriter.current_text = "Finished line".into();
        }
        app.insert_resource(typewriter)
            .init_resource::<OptionSelection>()
            .add_observer(activate_option);
        let button = app
            .world_mut()
            .spawn((Button, OptionButton(OptionId(1))))
            .id();
        (app, button)
    }

    #[test]
    fn activation_waits_for_finished_text_and_ignores_other_widgets() {
        let (mut app, button) = app(false);
        app.world_mut().trigger(Activate { entity: button });
        assert_eq!(app.world().resource::<OptionSelection>().selected, None);
        app.world_mut().resource_mut::<Typewriter>().current_text = "Finished line".into();
        let unrelated = app.world_mut().spawn(Button).id();
        app.world_mut().trigger(Activate { entity: unrelated });
        assert_eq!(app.world().resource::<OptionSelection>().selected, None);
        app.world_mut().trigger(Activate { entity: button });
        assert_eq!(
            app.world().resource::<OptionSelection>().selected,
            Some(OptionId(1))
        );
    }

    #[test]
    fn activation_outside_a_conversation_does_not_require_a_typewriter() {
        let (mut app, option) = app(true);
        let unrelated = app.world_mut().spawn(Button).id();
        // Typewriter exists only between DialogueStarted and DialogueCompleted.
        // Our global Activate observer also receives title/phone button clicks.
        app.world_mut().remove_resource::<Typewriter>();
        app.world_mut().trigger(Activate { entity: unrelated });
        app.world_mut().trigger(Activate { entity: option });
        assert_eq!(app.world().resource::<OptionSelection>().selected, None);
        app.world_mut().remove_resource::<OptionSelection>();
        app.world_mut().trigger(Activate { entity: unrelated });
    }

    #[test]
    fn duplicate_activation_keeps_one_selection_and_late_activation_is_safe() {
        let (mut app, button) = app(true);
        let second = app
            .world_mut()
            .spawn((Button, OptionButton(OptionId(2))))
            .id();
        app.world_mut().trigger(Activate { entity: button });
        app.world_mut().trigger(Activate { entity: second });
        assert_eq!(
            app.world().resource::<OptionSelection>().selected,
            Some(OptionId(1))
        );
        app.world_mut().remove_resource::<OptionSelection>();
        app.world_mut().trigger(Activate { entity: button });
    }

    #[test]
    fn pointer_click_bubbles_from_label_and_activates_the_option() {
        use bevy::camera::NormalizedRenderTarget;
        use bevy::picking::{
            backend::HitData,
            events::{Pointer, PointerClick, PointerPress, PointerRelease},
            pointer::{Location, PointerButton, PointerId},
        };

        let (mut app, button) = app(true);
        app.add_plugins(ButtonPlugin);
        // PointerTraversal queries Window even without a windowing plugin.
        app.world_mut().spawn(Window::default());
        let label = app.world_mut().spawn(ChildOf(button)).id();
        app.update();
        let pointer = Pointer::new(
            PointerId::Mouse,
            Location {
                target: NormalizedRenderTarget::None {
                    width: 1,
                    height: 1,
                },
                position: Vec2::ZERO,
            },
        );
        let hit = HitData::new(Entity::PLACEHOLDER, 0.0, None, None);
        app.world_mut().trigger(PointerPress {
            entity: label,
            pointer: pointer.clone(),
            button: PointerButton::Primary,
            hit: hit.clone(),
            count: 1,
        });
        app.update();
        assert!(app.world().get::<Pressed>(button).is_some());
        assert_eq!(app.world().resource::<OptionSelection>().selected, None);
        // Bevy dispatches click before release; ButtonPlugin emits Activate.
        app.world_mut().trigger(PointerClick {
            entity: label,
            pointer: pointer.clone(),
            button: PointerButton::Primary,
            hit: hit.clone(),
            duration: std::time::Duration::from_millis(50),
            count: 1,
        });
        app.update();
        assert_eq!(
            app.world().resource::<OptionSelection>().selected,
            Some(OptionId(1))
        );
        app.world_mut().trigger(PointerRelease {
            entity: label,
            pointer,
            button: PointerButton::Primary,
            hit,
        });
        app.update();
        assert!(app.world().get::<Pressed>(button).is_none());
    }
}
