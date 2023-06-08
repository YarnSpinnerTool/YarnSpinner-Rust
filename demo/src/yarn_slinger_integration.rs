use crate::easing::EasedChange;
use crate::setup::StageCurtains;
use crate::visual_effects::{CameraMovement, FadeCurtainAlpha, RotationPhase};
use crate::{Sprites, CAMERA_TRANSLATION, FERRIS_TRANSLATION, SECOND_ACT_CAMERA_TRANSLATION};
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy_yarn_slinger_example_ui::prelude::*;
use std::f32::consts::PI;
use std::sync::atomic::AtomicBool;
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
    let change = EasedChange::new(
        transform.rotation,
        transform.rotation * Quat::from_rotation_y(PI),
        1.5,
    );
    *rotator = RotationPhase::ChangingSprite {
        change,
        sprite: Some(new_sprite),
    }
}

pub(crate) fn rotate_character(
    In(character): In<&str>,
    mut speakers: Query<(&Speaker, &Transform, &mut RotationPhase)>,
) {
    let (.., transform, mut rotator) = speakers
        .iter_mut()
        .find(|(speaker, ..)| speaker.name.to_lowercase() == character.to_lowercase())
        .unwrap();
    let change = EasedChange::new(
        transform.rotation,
        transform.rotation * Quat::from_rotation_y(PI),
        1.5,
    );
    *rotator = RotationPhase::ChangingSprite {
        change,
        sprite: None,
    }
}

pub(crate) fn fade_in(
    In(seconds): In<f32>,
    mut commands: Commands,
    color: Query<&BackgroundColor, With<StageCurtains>>,
) -> Arc<AtomicBool> {
    let change = EasedChange::new(color.single().0.a(), 0.0, seconds);
    let done = change.done.clone();

    commands.insert_resource(FadeCurtainAlpha(change));
    done
}

pub(crate) fn move_camera_to_clippy(_: In<()>, mut commands: Commands) -> Arc<AtomicBool> {
    let from_translation =
        Transform::from_translation(CAMERA_TRANSLATION).looking_at(FERRIS_TRANSLATION, Vec3::Y);
    let mut to_transform = Transform::from_translation(SECOND_ACT_CAMERA_TRANSLATION)
        .looking_at(FERRIS_TRANSLATION, Vec3::Y);
    to_transform.rotation *= Quat::from_rotation_y(-45.0_f32.to_radians());
    let change = EasedChange::new(from_translation, to_transform, 2.0);
    let done = change.done.clone();
    commands.insert_resource(CameraMovement(change));
    done
}
