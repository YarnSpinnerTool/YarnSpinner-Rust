use crate::ExampleYarnSpinnerDialogueViewSystemSet;
use crate::option_selection::OptionSelection;
use crate::setup::{DialogueContinueNode, DialogueNode, UiRootNode, create_dialog_text};
use crate::updating::SpeakerChangeEvent;
use bevy::platform::time::Instant;
use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn typewriter_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            send_finished_event.run_if(resource_exists::<Typewriter>),
            write_text.run_if(resource_exists::<Typewriter>),
            show_continue.run_if(resource_exists::<Typewriter>),
            bob_continue,
        )
            .chain()
            .after(YarnSpinnerSystemSet)
            .in_set(ExampleYarnSpinnerDialogueViewSystemSet),
    )
    .add_message::<TypewriterFinishedEvent>();

    app.add_observer(despawn);
    app.add_observer(spawn);
}

#[derive(Debug, Eq, PartialEq, Hash, Reflect, Message)]
pub(crate) struct TypewriterFinishedEvent;

#[derive(Debug, Clone, PartialEq, Resource)]
pub(crate) struct Typewriter {
    pub(crate) character_name: Option<String>,
    pub(crate) current_text: String,
    pub(crate) graphemes_left: Vec<String>,
    pub(crate) last_before_options: bool,
    elapsed: f32,
    start: Instant,
    fast_typing: bool,
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            character_name: default(),
            current_text: default(),
            graphemes_left: default(),
            last_before_options: default(),
            elapsed: default(),
            start: Instant::now(),
            fast_typing: default(),
        }
    }
}

impl Typewriter {
    pub(crate) fn set_line(&mut self, line: &LocalizedLine) {
        *self = Self {
            character_name: line.character_name().map(|s| s.to_string()),
            current_text: String::new(),
            graphemes_left: line
                .text_without_character_name()
                .graphemes(true)
                .map(|s| s.to_string())
                .collect(),
            last_before_options: line.is_last_line_before_options(),
            ..default()
        };
    }

    pub(crate) fn is_finished(&self) -> bool {
        self.graphemes_left.is_empty() && !self.current_text.is_empty()
    }

    pub(crate) fn fast_forward(&mut self) {
        self.fast_typing = true;
    }

    fn update_current_text(&mut self) {
        if self.is_finished() {
            return;
        }
        self.elapsed += self.start.elapsed().as_secs_f32();
        self.start = Instant::now();
        let calculated_graphemes = (self.graphemes_per_second() * self.elapsed).floor() as usize;
        let graphemes_left = self.graphemes_left.len();
        let grapheme_length_to_take = (calculated_graphemes).min(graphemes_left);
        self.elapsed -= grapheme_length_to_take as f32 / self.graphemes_per_second();
        let graphemes_to_take = self.graphemes_left.drain(..grapheme_length_to_take);
        self.current_text.extend(graphemes_to_take);
    }

    fn graphemes_per_second(&self) -> f32 {
        if self.fast_typing { 120.0 } else { 40.0 }
    }
}

fn write_text(
    mut commands: Commands,
    text: Single<Entity, With<DialogueNode>>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut speaker_change_events: MessageWriter<SpeakerChangeEvent>,
    mut root_visibility: Single<&mut Visibility, With<UiRootNode>>,
) {
    let mut text_entity = commands.entity(*text);
    if typewriter.last_before_options && option_selection.is_none() {
        text_entity.despawn_related::<Children>();
        return;
    }
    if typewriter.is_finished() {
        return;
    }
    if !typewriter.last_before_options {
        **root_visibility = Visibility::Inherited;
        // If this is last before options, the `OptionSelection` will make the visibility inherited as soon as it's ready instead
    }
    typewriter.update_current_text();
    if typewriter.is_finished()
        && let Some(name) = typewriter.character_name.as_deref()
    {
        speaker_change_events.write(SpeakerChangeEvent {
            character_name: name.to_string(),
            speaking: false,
        });
    }

    let current_text = &typewriter.current_text;
    let rest = typewriter.graphemes_left.join("");
    let spans = create_dialog_text(current_text, rest);
    text_entity
        .despawn_related::<Children>()
        .with_children(|parent| {
            parent.spawn(spans[0].clone());
            parent.spawn(spans[1].clone());
        });
}

fn show_continue(
    typewriter: Res<Typewriter>,
    mut visibility: Single<&mut Visibility, With<DialogueContinueNode>>,
    mut typewriter_finished_event: MessageReader<TypewriterFinishedEvent>,
) {
    for _event in typewriter_finished_event.read() {
        if !typewriter.last_before_options {
            **visibility = Visibility::Inherited;
        }
    }
}

pub(crate) fn despawn(_ : On<DialogueCompleted>, mut commands: Commands) {
    commands.remove_resource::<Typewriter>();
}

pub(crate) fn spawn(_: On<DialogueStarted>, mut commands: Commands) {
    commands.init_resource::<Typewriter>();
}

fn bob_continue(
    time: Res<Time>,
    visibility: Single<&Visibility, With<DialogueContinueNode>>,
    mut style: Single<&mut Node, With<DialogueContinueNode>>,
) {
    if *visibility == Visibility::Hidden {
        return;
    }
    let pixels = (time.elapsed_secs() * 3.0).sin().powi(2) * 5.0;
    style.bottom = Val::Px(pixels);
}

fn send_finished_event(
    mut events: MessageWriter<TypewriterFinishedEvent>,
    typewriter: Res<Typewriter>,
    mut last_finished: Local<bool>,
) {
    if !typewriter.is_finished() {
        *last_finished = false;
    } else if !*last_finished {
        events.write(TypewriterFinishedEvent);
        *last_finished = true;
    }
}
