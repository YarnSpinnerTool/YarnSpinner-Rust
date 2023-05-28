use bevy::prelude::*;
pub use command_registry::YarnCommands;
pub use command_wrapping::{UntypedYarnCommandFn, YarnCommandFn};
use seldom_fn_plugin::FnPluginExt;

mod command_registry;
mod command_wrapping;

pub(crate) fn commands_plugin(app: &mut App) {
    app.fn_plugin(command_wrapping::command_wrapping_plugin)
        .fn_plugin(command_registry::command_registry_plugin);
}
