use bevy::prelude::*;
pub(crate) use command_registry::wait::update_wait;
pub use command_registry::YarnCommands;
pub use command_wrapping::{TaskFinishedIndicator, UntypedYarnCommand, YarnCommand};

mod command_registry;
mod command_wrapping;
mod execution;

pub(crate) fn commands_plugin(app: &mut App) {
    app.add_plugins(command_wrapping::command_wrapping_plugin)
        .add_plugins(command_registry::command_registry_plugin)
        .add_plugins(execution::command_execution_plugin);
}
