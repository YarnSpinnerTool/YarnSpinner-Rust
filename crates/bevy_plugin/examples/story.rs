use bevy::prelude::*;
use bevy::window::PresentMode;
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
    .add_plugin(
        YarnSlingerPlugin::with_yarn_files(vec!["story.yarn"]).with_localizations(Localizations {
            base_language: "en-US".into(),
            translations: vec!["de-CH".into()],
            #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
            file_generation_mode: FileGenerationMode::Development,
            #[cfg(any(target_arch = "wasm32", target_os = "android"))]
            file_generation_mode: FileGenerationMode::Production,
        }),
    )
    .add_plugin(ExampleYarnSlingerUiPlugin::new())
    .add_system(setup.on_startup())
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
