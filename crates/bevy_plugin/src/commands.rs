use bevy::prelude::*;
pub(crate) use command_registry::wait::update_wait;
pub use command_registry::YarnCommands;
pub use command_wrapping::{TaskFinishedIndicator, UntypedYarnCommand, YarnCommand};
use seldom_fn_plugin::FnPluginExt;

mod command_registry;
mod command_wrapping;
mod execution;

pub(crate) fn commands_plugin(app: &mut App) {
    app.fn_plugin(command_wrapping::command_wrapping_plugin)
        .fn_plugin(command_registry::command_registry_plugin)
        .fn_plugin(execution::command_execution_plugin);
}
