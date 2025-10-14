use crate::easing::EasedChange;
use crate::setup::{MainCamera, StageCurtains};
use crate::visual_effects::{Bang, CameraMovement, FadeCurtainAlpha, RotationPhase};
use crate::{
    CAMERA_TRANSLATION, CLIPPY_TRANSLATION, FERRIS_TRANSLATION, SECOND_ACT_CAMERA_TRANSLATION,
    Sprites,
};
use bevy::app::AppExit;
use bevy::light::NotShadowCaster;
use bevy::platform::time::Instant;
use bevy::prelude::*;
use bevy_sprite3d::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;
use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

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
    mut speaker_change_events: MessageReader<SpeakerChangeEvent>,
    mut speakers: Query<&mut Speaker>,
) {
    for event in speaker_change_events.read() {
        let event_name = event.character_name.to_lowercase();
        let everyone_is_speaking = event_name == "everyone";
        let speaker_names: Vec<_> = event_name.split(' ').collect();
        for mut speaker in &mut speakers {
            if everyone_is_speaking || speaker_names.contains(&speaker.name.to_lowercase().as_str())
            {
                if event.speaking {
                    speaker.last_active = Instant::now();
                    speaker.active = true;
                } else {
                    speaker.active = false;
                }
            }
        }
    }
}

pub(crate) fn change_sprite(
    In((character, sprite)): In<(String, String)>,
    mut speakers: Query<(&Speaker, &Transform, &mut RotationPhase)>,
    camera_transform: Single<&Transform, (With<MainCamera>, Without<RotationPhase>)>,
    sprites: Res<Sprites>,
) {
    let (.., transform, mut rotator) = speakers
        .iter_mut()
        .find(|(speaker, ..)| speaker.name.to_lowercase() == character.to_lowercase())
        .unwrap();
    let new_sprite = match sprite.as_str() {
        "ferris_neutral" => Some(sprites.ferris_neutral.clone()),
        "ferris_happy" => Some(sprites.ferris_happy.clone()),
        "clippy" => Some(sprites.clippy.clone()),
        "" => None,
        _ => panic!("Unknown sprite {sprite}"),
    };

    // Not using the current rotation because we might be mid-rotation from the last sprite change.
    let original_rotation = transform
        .looking_at(camera_transform.translation, Vec3::Y)
        .rotation;
    let change = EasedChange::new(
        original_rotation,
        original_rotation * Quat::from_rotation_y(PI),
        0.7,
    );
    *rotator = RotationPhase::ChangingSprite {
        change,
        sprite: new_sprite,
    }
}

pub(crate) fn rotate_character(
    In(character): In<String>,
    speakers: Query<(&Speaker, &Transform, &mut RotationPhase)>,
    camera: Single<&Transform, (With<MainCamera>, Without<RotationPhase>)>,
    sprites: Res<Sprites>,
) {
    change_sprite(In((character, "".to_string())), speakers, camera, sprites);
}

pub(crate) fn fade_in(
    In(seconds): In<f32>,
    mut commands: Commands,
    color: Single<&BackgroundColor, With<StageCurtains>>,
) -> Arc<AtomicBool> {
    let change = EasedChange::new(color.0.alpha(), 0.0, seconds);
    let done = change.done.clone();

    commands.insert_resource(FadeCurtainAlpha(change));
    done
}

pub(crate) fn fade_out(
    In(seconds): In<f32>,
    mut commands: Commands,
    color: Single<&BackgroundColor, With<StageCurtains>>,
) -> Arc<AtomicBool> {
    let change = EasedChange::new(color.0.alpha(), 1.0, seconds);
    let done = change.done.clone();

    commands.insert_resource(FadeCurtainAlpha(change));
    done
}

pub(crate) fn quit(_: In<()>, mut app_exit_events: MessageWriter<AppExit>) {
    app_exit_events.write(AppExit::Success);
}

pub(crate) fn move_camera_to_clippy(_: In<()>, mut commands: Commands) -> Arc<AtomicBool> {
    let from_translation =
        Transform::from_translation(CAMERA_TRANSLATION).looking_at(FERRIS_TRANSLATION, Vec3::Y);
    let vision_target = (FERRIS_TRANSLATION
        + Vec3::new(0.0, CLIPPY_TRANSLATION.y * 0.8, CLIPPY_TRANSLATION.z * 0.1))
        / 2.0;
    let to_transform = Transform::from_translation(SECOND_ACT_CAMERA_TRANSLATION)
        .looking_at(vision_target, Vec3::Y);

    let change = EasedChange::new(from_translation, to_transform, 1.2);
    let done = change.done.clone();
    commands.insert_resource(CameraMovement(change));
    done
}

pub(crate) fn show_bang(
    In((character, duration)): In<(String, f32)>,
    speakers: Query<&Speaker>,
    mut commands: Commands,
    camera_transform: Single<&Transform, With<MainCamera>>,
    sprites: Res<Sprites>,
) {
    let speaker = speakers
        .iter()
        .find(|speaker| speaker.name.to_lowercase() == character.to_lowercase())
        .unwrap();
    let speaker_back =
        (speaker.initial_translation - camera_transform.translation).normalize() * 0.1;
    let change = EasedChange::new(
        (speaker.initial_translation + speaker_back, 0.0),
        (
            speaker.initial_translation + Vec3::Y * 0.45 + speaker_back,
            1.0,
        ),
        // One third is transition, two thirds is just showing the bang.
        duration / 3.0,
    );
    commands.spawn((
        Sprite3d {
            pixels_per_metre: 900.,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        },
        Sprite {
            image: sprites.bang.clone(),
            ..default()
        },
        Transform::from_translation(speaker.initial_translation + speaker_back)
            .looking_at(CAMERA_TRANSLATION, Vec3::Y),
        NotShadowCaster,
        Bang(change),
    ));
}
