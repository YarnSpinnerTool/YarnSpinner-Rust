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
            base_localization: "en-US".into(),
            translations: vec!["de-CH".into()],
            file_generation_mode: FileGenerationMode::DEVELOPMENT_ON_SUPPORTED_PLATFORMS,
        }),
    )
    .add_plugin(ExampleYarnSlingerUiPlugin::new())
    .add_systems((
        setup.on_startup(),
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
    ))
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.default_dialogue_runner().unwrap();
    // Immediately start showing the dialogue
    dialogue_runner.start();
    commands.spawn(dialogue_runner);
}
