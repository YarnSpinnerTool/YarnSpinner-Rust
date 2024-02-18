use crate::visual_effects::RotationPhase;
use crate::yarnspinner_integration::{
    change_sprite, fade_in, fade_out, move_camera_to_clippy, quit, rotate_character, show_bang,
    Speaker,
};
use crate::{Sprites, CAMERA_TRANSLATION, CLIPPY_TRANSLATION, FERRIS_TRANSLATION};
#[cfg(not(target_arch = "wasm32"))]
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::gltf::Gltf;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::camera::Exposure;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use bevy_yarnspinner::prelude::*;

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Unfortunately, MSAA and HDR are not supported simultaneously under WebGL.
    // Since this example uses HDR, we must disable MSAA for WASM builds, at least
    // until WebGPU is ready and no longer behind a feature flag in Web browsers.
    #[cfg(target_arch = "wasm32")]
    commands.insert_resource(Msaa::Off);
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(CAMERA_TRANSLATION)
                .looking_at(FERRIS_TRANSLATION, Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            exposure: Exposure::INDOOR,
            ..default()
        },
        #[cfg(not(target_arch = "wasm32"))]
        BloomSettings {
            intensity: 0.07,
            ..default()
        },
        MainCamera,
    ));
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/coffee_shop.glb#Scene0"),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::BISQUE,
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 20.0,
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(-3.5, 2.3, 1.15).looking_at(FERRIS_TRANSLATION, Vec3::Y),
        ..default()
    });

    for (x, y, z) in [
        (-1.0, 2.5, 0.75),
        (-1.0, 2.5, -1.6),
        (3.0, 2.5, 0.75),
        (3.0, 2.5, -1.6),
    ] {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                color: Color::rgb(1.0, 0.78, 0.45),
                intensity: 10_000.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(x, y, z),
            ..default()
        });
    }

    // Start game with a black background
    commands.spawn((
        NodeBundle {
            background_color: Color::BLACK.into(),
            z_index: ZIndex::Global(-1),
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
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
        bang: asset_server.load("sprites/bang.png"),
    });
}

pub(crate) fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    dialogue_runner
        .commands_mut()
        .add_command("change_sprite", change_sprite)
        .add_command("fade_in", fade_in)
        .add_command("fade_out", fade_out)
        .add_command("quit", quit)
        .add_command("rotate", rotate_character)
        .add_command("move_camera_to_clippy", move_camera_to_clippy)
        .add_command("show_bang", show_bang);
    // Immediately start showing the dialogue
    dialogue_runner.start_node("Start");
    commands.spawn(dialogue_runner);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct StageCurtains;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct MainCamera;

pub(crate) fn adapt_materials(
    gltfs: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    let Some(gltf) = gltfs.get(&asset_server.load("models/coffee_shop.glb")) else {
        return;
    };
    let glass_handle = gltf.named_materials.get("Glass").unwrap();
    let glass_material = materials.get_mut(glass_handle).unwrap();
    // No way to export this from Blender, unfortunately
    glass_material.alpha_mode = AlphaMode::Add;
    *done = true;
}

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
            alpha_mode: AlphaMode::Blend,
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
            alpha_mode: AlphaMode::Blend,
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
