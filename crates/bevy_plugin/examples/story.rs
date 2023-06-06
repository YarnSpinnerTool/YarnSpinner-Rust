use bevy::asset::LoadState;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_editor_pls::EditorPlugin;
use bevy_sprite3d::{Sprite3d, Sprite3dParams, Sprite3dPlugin};
use bevy_yarn_slinger::prelude::*;

#[cfg(not(feature = "example_ui"))]
compile_error!("This example requires the `example_ui` feature to be enabled");

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
                    fit_canvas_to_parent: true,
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
    ))
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
    });
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.default_dialogue_runner().unwrap();
    // Immediately start showing the dialogue
    dialogue_runner.start();
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
    commands.spawn(
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
    );
    *done = true;
}

const FERRIS_TRANSLATION: Vec3 = Vec3::new(-1.3, 0.9, 0.3);
const CAMERA_TRANSLATION: Vec3 = Vec3::new(-1.7, 1.4, 1.8);

#[derive(Resource)]
struct Sprites {
    ferris_neutral: Handle<Image>,
}

fn sprites_have_loaded(sprites: Res<Sprites>, asset_server: Res<AssetServer>) -> bool {
    asset_server.get_load_state(&sprites.ferris_neutral) == LoadState::Loaded
}
