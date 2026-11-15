use bevy::app::App;

pub mod yarn_file;
pub mod yarnc;

pub fn plugin(app: &mut App) {
    app.add_plugins((yarn_file::plugin, yarnc::plugin));
}
