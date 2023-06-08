use crate::setup::StageCurtains;
use crate::Sprites;
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarn_slinger_example_ui::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Component)]
pub(crate) struct Speaker {
    pub(crate) name: String,
    pub(crate) active: bool,
    pub(crate) last_active: Instant,
    pub(crate) initial_translation: Vec3,
}
impl Default for Speaker {
    fn default() -> Self {
        Self {
            name: "Unknown".into(),
            active: false,
            last_active: Instant::now(),
            initial_translation: Vec3::ZERO,
        }
    }
}

pub(crate) fn change_speaker(
    mut speaker_change_events: EventReader<SpeakerChangeEvent>,
    mut speakers: Query<(&mut Speaker, &Transform)>,
) {
    for event in speaker_change_events.iter() {
        let Some((mut speaker, transform)) = speakers
            .iter_mut()
            .find(|(speaker, ..)| speaker.name.to_lowercase() == event.character_name.to_lowercase()) else {
            continue;
        };
        if event.speaking {
            speaker.last_active = Instant::now();
            speaker.active = true;
            speaker.initial_translation = transform.translation;
        } else {
            speaker.active = false;
        }
    }
}

pub(crate) fn change_sprite(
    In((character, sprite)): In<(&str, &str)>,
    mut speakers: Query<(&Speaker, &Transform, &mut RotationPhase)>,
    sprites: Res<Sprites>,
) {
    let (.., transform, mut rotator) = speakers
        .iter_mut()
        .find(|(speaker, ..)| speaker.name.to_lowercase() == character.to_lowercase())
        .unwrap();
    let new_sprite = match sprite {
        "ferris_neutral" => sprites.ferris_neutral.clone(),
        "ferris_happy" => sprites.ferris_happy.clone(),
        "clippy" => sprites.clippy.clone(),
        _ => panic!("Unknown sprite {sprite}"),
    };
    *rotator = RotationPhase::ChangingSprite {
        target_transform: {
            let mut target_transform = *transform;
            target_transform.rotate_local_y(-180.0_f32.to_radians());
            target_transform
        },
        new_sprite: Some(new_sprite),
    }
}

#[derive(Component, Default)]
pub(crate) enum RotationPhase {
    #[default]
    None,
    ChangingSprite {
        new_sprite: Option<Handle<Image>>,
        target_transform: Transform,
    },
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct Fade {
    duration: f32,
    start_alpha: f32,
    end_alpha: f32,
    done: Arc<AtomicBool>,
    start: Instant,
}

pub(crate) fn fade_in(
    In(seconds): In<f32>,
    mut commands: Commands,
    color: Query<&BackgroundColor, With<StageCurtains>>,
) -> Arc<AtomicBool> {
    let done = Arc::new(AtomicBool::new(false));
    commands.insert_resource(Fade {
        duration: seconds,
        start_alpha: color.single().0.a(),
        end_alpha: 0.0,
        done: done.clone(),
        start: Instant::now(),
    });
    done
}

pub(crate) fn handle_fade(
    mut commands: Commands,
    fade: ResMut<Fade>,
    mut color: Query<&mut BackgroundColor, With<StageCurtains>>,
) {
    let input = (fade.start.elapsed().as_secs_f32() / fade.duration).min(1.0);

    let smooth_start = |input: f32| input.powi(3);
    let smooth_end = |input: f32| 1.0 - (1.0 - input).powi(2);
    let scene_becomes_visible = fade.start_alpha > fade.end_alpha;
    let easing_fn = if scene_becomes_visible {
        smooth_start
    } else {
        smooth_end
    };
    let output = easing_fn(input);

    let alpha = fade.start_alpha + (fade.end_alpha - fade.start_alpha) * output;
    color.single_mut().0.set_a(alpha);
    if input >= 0.99 {
        commands.remove_resource::<Fade>();
        fade.done.store(true, Ordering::Relaxed);
    }
}
