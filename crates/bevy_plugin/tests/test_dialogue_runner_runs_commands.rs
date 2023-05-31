use anyhow::Result;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use utils::prelude::*;

mod utils;

#[test]
fn errs_on_selection_without_start() -> Result<()> {
    let mut app = App::new();
    setup_dialogue_runner(&mut app)
        .select_option(OptionId(0))
        .unwrap_err();

    Ok(())
}

fn setup_dialogue_runner(app: &mut App) -> Mut<DialogueRunner> {
    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["commands.yarn"]))
        .dialogue_runner_mut()
}
