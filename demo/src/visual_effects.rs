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
    ChangingSprite(SpriteChange),
}

pub(crate) struct SpriteChange {
    pub(crate) new_sprite: Option<Handle<Image>>,
    pub(crate) initial_transform: Transform,
    pub(crate) duration: f32,
    pub(crate) start: Instant,
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
        let RotationPhase::ChangingSprite(sprite_change)= rotator.deref_mut() else {
            continue;
        };
        let input = (sprite_change.start.elapsed().as_secs_f32() / sprite_change.duration).min(1.0);
        let output = ease_out_elastic(input);

        let target_y_rotation = 180.0_f32.to_radians();
        let y_rotation = target_y_rotation * output;
        transform.rotation =
            sprite_change.initial_transform.rotation * Quat::from_rotation_y(y_rotation);

        let rotation_half_way_done = output >= 0.5;
        if rotation_half_way_done {
            if let Some(new_sprite) = sprite_change.new_sprite.take() {
                let material = materials.get_mut(material).unwrap();
                material.base_color_texture.replace(new_sprite);
            }
        }
        let done = input >= 0.99;
        if done {
            *rotator = RotationPhase::None;
        }
    }
}

/// Source: <https://easings.net/#easeOutElastic>
fn ease_out_elastic(x: f32) -> f32 {
    const C4: f32 = (2.0 * std::f32::consts::PI) / 3.0;

    (2.0_f32).powf(-10.0 * x) * ((x * 10.0 - 0.75) * C4).sin() + 1.0
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
