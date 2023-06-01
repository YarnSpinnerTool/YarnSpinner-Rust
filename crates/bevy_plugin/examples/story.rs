use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_yarn_slinger::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
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
            file_generation_mode: FileGenerationMode::Development,
        }),
    )
    .run();
}
