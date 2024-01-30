// Disable windows console in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use self::{setup::*, visual_effects::*, yarnspinner_integration::*};
use bevy::asset::{AssetMetaCheck, LoadState};
use bevy::prelude::*;
use bevy::scene::SceneInstance;
use bevy::window::PresentMode;
#[cfg(feature = "editor")]
use bevy_editor_pls::EditorPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

mod easing;
mod setup;
mod visual_effects;
mod yarnspinner_integration;

fn main() {
    let mut app = App::new();
    app.insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Yarn Spinner Story Demo".into(),
                    resolution: (800., 600.).into(),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogue/story.yarn")),
            ExampleYarnSpinnerDialogueViewPlugin::new(),
            Sprite3dPlugin,
            #[cfg(feature = "editor")]
            EditorPlugin::new(),
        ))
        .insert_resource(ClearColor(Color::CYAN))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
                adapt_materials.run_if(any_with_component::<SceneInstance>()),
                spawn_sprites.run_if(sprites_have_loaded),
            ),
        )
        .add_systems(
            Update,
            (
                handle_fade.run_if(resource_exists::<FadeCurtainAlpha>()),
                move_camera.run_if(resource_exists::<CameraMovement>()),
                change_speaker,
                bob_speaker,
                rotate_sprite,
                ease_bang.run_if(any_with_component::<Bang>()),
            )
                .chain()
                .after(ExampleYarnSpinnerDialogueViewSystemSet),
        )
        .run();
}

#[derive(Resource)]
struct Sprites {
    ferris_neutral: Handle<Image>,
    ferris_happy: Handle<Image>,
    clippy: Handle<Image>,
    bang: Handle<Image>,
}

fn sprites_have_loaded(sprites: Res<Sprites>, asset_server: Res<AssetServer>) -> bool {
    asset_server.get_load_state(&sprites.ferris_neutral) == Some(LoadState::Loaded)
        && asset_server.get_load_state(&sprites.ferris_happy) == Some(LoadState::Loaded)
        && asset_server.get_load_state(&sprites.clippy) == Some(LoadState::Loaded)
        && asset_server.get_load_state(&sprites.bang) == Some(LoadState::Loaded)
}

const FERRIS_TRANSLATION: Vec3 = Vec3::new(-1.3, 0.9, 0.35);
const CLIPPY_TRANSLATION: Vec3 = Vec3::new(1.5, 0.94, -0.4);
const CAMERA_TRANSLATION: Vec3 = Vec3::new(-1.7, 1.4, 1.8);
const SECOND_ACT_CAMERA_TRANSLATION: Vec3 = Vec3::new(-2.0, 1.4, 1.8);
