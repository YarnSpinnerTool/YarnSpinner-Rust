use crate::yarn_slinger_integration::{RotationPhase, Speaker};
use bevy::prelude::*;
use std::ops::DerefMut;

pub(crate) fn bob_speaker(mut speakers: Query<(&Speaker, &mut Transform)>) {
    for (speaker, mut transform) in speakers.iter_mut() {
        let is_back_at_initial_position =
            (transform.translation.y - speaker.initial_translation.y).powi(2) < 1e-5;

        if !speaker.active && is_back_at_initial_position {
            continue;
        }
        transform.translation.y = speaker.initial_translation.y
            + (speaker.last_active.elapsed().as_secs_f32() * 12.0)
                .sin()
                .powi(2)
                * 0.04;
    }
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
