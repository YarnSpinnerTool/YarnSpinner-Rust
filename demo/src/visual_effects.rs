use crate::easing::EasedChange;
use crate::setup::{MainCamera, StageCurtains};
use crate::yarn_slinger_integration::Speaker;
use bevy::prelude::*;
use std::fmt::Debug;
use std::ops::DerefMut;

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
        change: EasedChange<Quat>,
        sprite: Option<Handle<Image>>,
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
        let RotationPhase::ChangingSprite{change, sprite}= rotator.deref_mut() else {
            continue;
        };
        let output = change.elastic(2);

        let rotation_half_way_done = output >= 0.5;
        if rotation_half_way_done {
            if let Some(new_sprite) = sprite.take() {
                let material = materials.get_mut(material).unwrap();
                material.base_color_texture.replace(new_sprite);
            }
        }
        if change.is_done() {
            transform.rotation = change.to;
            *rotator = RotationPhase::None;
        } else {
            transform.rotation = change.from.slerp(change.to, output);
        }
    }
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct FadeCurtainAlpha(pub(crate) EasedChange<f32>);

pub(crate) fn handle_fade(
    mut commands: Commands,
    mut fade: ResMut<FadeCurtainAlpha>,
    mut color: Query<&mut BackgroundColor, With<StageCurtains>>,
) {
    if fade.0.is_done() {
        color.single_mut().0.set_a(fade.0.to);
        commands.remove_resource::<FadeCurtainAlpha>();
        fade.0.set_done();
    } else {
        let scene_becomes_visible = fade.0.from > fade.0.to;
        let output = if scene_becomes_visible {
            fade.0.smooth_start()
        } else {
            fade.0.smooth_end()
        };
        let alpha = fade.0.from + (fade.0.to - fade.0.from) * output;
        color.single_mut().0.set_a(alpha);
    }
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct CameraMovement(pub(crate) EasedChange<Transform>);

pub(crate) fn move_camera(
    mut commands: Commands,
    mut camera_movement: ResMut<CameraMovement>,
    mut transform: Query<&mut Transform, With<MainCamera>>,
) {
    if camera_movement.0.is_done() {
        commands.remove_resource::<CameraMovement>();
        camera_movement.0.set_done();
        *transform.single_mut() = camera_movement.0.to;
    } else {
        let translation_output = camera_movement.0.elastic(1);
        let rotation_output = camera_movement.0.elastic(1);
        let mut transform = transform.single_mut();
        transform.translation = camera_movement
            .0
            .from
            .translation
            .lerp(camera_movement.0.to.translation, translation_output);
        transform.rotation = camera_movement
            .0
            .from
            .rotation
            .slerp(camera_movement.0.to.rotation, rotation_output);
    }
}
