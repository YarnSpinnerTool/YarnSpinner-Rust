use bevy::asset::LoadState;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy::window::PresentMode;
use bevy_editor_pls::EditorPlugin;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger_example_ui::prelude::*;
use std::ops::DerefMut;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
                watch_for_changes: true,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Yarn Slinger Story Demo".into(),
                    resolution: (800., 600.).into(),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugin(EditorPlugin::new())
    .add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["story.yarn"]).with_localizations(Localizations {
            base_localization: "en-US".into(),
            translations: vec!["de-CH".into()],
            file_generation_mode: FileGenerationMode::DEVELOPMENT_ON_SUPPORTED_PLATFORMS,
        }),
    )
    .add_plugin(ExampleYarnSlingerUiPlugin::new())
    .add_plugin(Sprite3dPlugin)
    .add_systems((
        setup.on_startup(),
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
        spawn_sprites.run_if(sprites_have_loaded),
        rotate_sprite,
    ))
    .add_systems(
        (change_speaker, bob_speaker)
            .chain()
            .after(ExampleYarnSlingerUiSystemSet),
    )
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    commands.insert_resource(AmbientLight {
        color: Color::rgb(1., 0.75, 0.7),
        brightness: 0.25,
    });
    commands.insert_resource(Sprites {
        ferris_neutral: asset_server.load("sprites/ferris_neutral.png"),
        ferris_happy: asset_server.load("sprites/ferris_happy.png"),
    });
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.default_dialogue_runner().unwrap();
    // Immediately start showing the dialogue
    dialogue_runner.start();
    dialogue_runner
        .command_registrations_mut()
        .register_command("change_sprite", change_sprite);
    commands.spawn(dialogue_runner);
}

fn spawn_sprites(
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
    *done = true;
}

const FERRIS_TRANSLATION: Vec3 = Vec3::new(-1.3, 0.9, 0.3);
const CAMERA_TRANSLATION: Vec3 = Vec3::new(-1.7, 1.4, 1.8);

#[derive(Resource)]
struct Sprites {
    ferris_neutral: Handle<Image>,
    ferris_happy: Handle<Image>,
}

#[derive(Component)]
struct Speaker {
    name: String,
    active: bool,
    last_active: Instant,
    initial_translation: Vec3,
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

fn sprites_have_loaded(sprites: Res<Sprites>, asset_server: Res<AssetServer>) -> bool {
    asset_server.get_load_state(&sprites.ferris_neutral) == LoadState::Loaded
        && asset_server.get_load_state(&sprites.ferris_happy) == LoadState::Loaded
}

fn change_speaker(
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

fn bob_speaker(mut speakers: Query<(&Speaker, &mut Transform)>) {
    for (speaker, mut transform) in speakers.iter_mut() {
        let is_back_at_initial_position =
            (transform.translation.y - speaker.initial_translation.y).powi(2) < 0.001;

        if !speaker.active && is_back_at_initial_position {
            continue;
        }
        transform.translation.y = speaker.initial_translation.y
            + (speaker.last_active.elapsed().as_secs_f32() * 10.0)
                .sin()
                .powi(2)
                * 0.05;
    }
}

fn change_sprite(
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

fn rotate_sprite(
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

#[derive(Component, Default)]
enum RotationPhase {
    #[default]
    None,
    ChangingSprite {
        new_sprite: Option<Handle<Image>>,
        target_transform: Transform,
    },
}