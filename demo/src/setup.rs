use crate::yarn_slinger_integration::{change_sprite, fade_in, RotationPhase, Speaker};
use crate::{Sprites, CAMERA_TRANSLATION, CLIPPY_TRANSLATION, FERRIS_TRANSLATION};
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use bevy_yarn_slinger::prelude::*;

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(CAMERA_TRANSLATION)
            .looking_at(FERRIS_TRANSLATION, Vec3::Y),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/coffee_shop.glb#Scene0"),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::BISQUE,
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-3., 10., 3.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // Start game with a black background
    commands.spawn((
        NodeBundle {
            background_color: Color::BLACK.into(),
            z_index: ZIndex::Global(-1),
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        StageCurtains,
    ));
    commands.insert_resource(AmbientLight {
        color: Color::rgb(1., 0.75, 0.7),
        brightness: 0.25,
    });
    commands.insert_resource(Sprites {
        ferris_neutral: asset_server.load("sprites/ferris_neutral.png"),
        ferris_happy: asset_server.load("sprites/ferris_happy.png"),
        clippy: asset_server.load("sprites/clippy.png"),
    });
}

pub(crate) fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.default_dialogue_runner().unwrap();
    // Immediately start showing the dialogue
    dialogue_runner.start();
    dialogue_runner
        .command_registrations_mut()
        .register_command("change_sprite", change_sprite)
        .register_command("fade_in", fade_in);
    commands.spawn(dialogue_runner);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct StageCurtains;

pub(crate) fn spawn_sprites(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut sprite_params: Sprite3dParams,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    commands.spawn((
        Sprite3d {
            image: sprites.ferris_neutral.clone(),
            pixels_per_metre: 600.,
            partial_alpha: true,
            unlit: true,
            transform: Transform::from_translation(FERRIS_TRANSLATION)
                .looking_at(CAMERA_TRANSLATION, Vec3::Y),
            ..default()
        }
        .bundle(&mut sprite_params),
        Speaker {
            name: "Ferris".into(),
            initial_translation: FERRIS_TRANSLATION,
            ..default()
        },
        RotationPhase::default(),
    ));
    commands.spawn((
        Sprite3d {
            image: sprites.clippy.clone(),
            pixels_per_metre: 350.,
            partial_alpha: true,
            unlit: true,
            transform: Transform::from_translation(CLIPPY_TRANSLATION)
                .looking_at(CAMERA_TRANSLATION, Vec3::Y),
            ..default()
        }
        .bundle(&mut sprite_params),
        Speaker {
            name: "Clippy".into(),
            initial_translation: CLIPPY_TRANSLATION,
            ..default()
        },
        RotationPhase::default(),
    ));
    *done = true;
}
