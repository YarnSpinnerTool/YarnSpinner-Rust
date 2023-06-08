use crate::setup::StageCurtains;
use crate::yarn_slinger_integration::Speaker;
use bevy::prelude::*;
use bevy::utils::Instant;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub(crate) fn bob_speaker(mut speakers: Query<(&Speaker, &mut Transform)>) {
    for (speaker, mut transform) in speakers.iter_mut() {
        let is_back_at_initial_position =
            (transform.translation.y - speaker.initial_translation.y).powi(2) < 1e-5;

        if !speaker.active && is_back_at_initial_position {
            continue;
        }
        transform.translation.y = speaker.initial_translation.y
            + (speaker.last_active.elapsed().as_secs_f32() * 10.0)
                .sin()
                .powi(2)
                * 0.04;
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

pub(crate) fn rotate_sprite(
    mut rotators: Query<(
        &mut Transform,
        &Handle<StandardMaterial>,
        &mut RotationPhase,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut transform, material, mut rotator) in rotators.iter_mut() {
        let RotationPhase::ChangingSprite { target_transform, new_sprite } = rotator.deref_mut() else {
            continue;
        };
        transform.rotation = transform.rotation.slerp(target_transform.rotation, 0.12);

        let dot_to_target = transform.rotation.dot(target_transform.rotation).abs();
        let halfway_done = dot_to_target >= 0.5;
        if halfway_done {
            if let Some(new_sprite) = new_sprite.take() {
                let material = materials.get_mut(material).unwrap();
                material.base_color_texture.replace(new_sprite);
            }
        }
        let done = dot_to_target >= 0.9;
        if done {
            transform.rotation = target_transform.rotation;
            *rotator = RotationPhase::None;
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct Fade {
    pub(crate) duration: f32,
    pub(crate) start_alpha: f32,
    pub(crate) end_alpha: f32,
    pub(crate) done: Arc<AtomicBool>,
    pub(crate) start: Instant,
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
