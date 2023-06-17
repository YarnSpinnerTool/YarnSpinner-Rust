use crate::prelude::*;
use crate::UnderlyingYarnCommand;
use bevy::prelude::*;

pub(crate) fn dialogue_runner_events_plugin(app: &mut App) {
    app.add_event::<PresentLineEvent>()
        .add_event::<PresentOptionsEvent>()
        .add_event::<ExecuteCommandEvent>()
        .add_event::<NodeCompleteEvent>()
        .add_event::<NodeStartEvent>()
        .add_event::<LineHintsEvent>()
        .add_event::<DialogueCompleteEvent>()
        .add_event::<DialogueStartEvent>();
}

/// An event that is fired after a dialogue advances and wishes to present a line to the user.
/// A dialogue view should listen for this event and draw it to the screen.
/// Handling this event is **mandatory** for dialogue views.
#[derive(Debug, Clone, PartialEq)]
pub struct PresentLineEvent {
    /// The line to present to the user.
    pub line: LocalizedLine,
    /// The [`DialogueRunner`] that is presenting this line.
    pub source: Entity,
}

/// An event that is fired after a dialogue advances and wishes to present a set of options to the user.
/// A dialogue view should listen for this event and draw it to the screen.
/// You need to handle this event by calling [`DialogueRunner::select_option`] with the ID found in the provided [`DialogueOption`]s.
/// Handling this event is **mandatory** for dialogue views.
#[derive(Debug, Clone, PartialEq)]
pub struct PresentOptionsEvent {
    /// The options to present to the user.
    pub options: Vec<DialogueOption>,
    /// The [`DialogueRunner`] that is presenting these options.
    pub source: Entity,
}

/// An event that is fired after a dialogue advances and wishes to execute a command.
/// Events are generally handled by looking them up in the [`YarnCommands`] of a [`DialogueRunner`],
/// accessed via [`DialogueRunner::commands`] and [`DialogueRunner::commands_mut`].
/// However, a command is allowed much more freedom in its syntax than one might think, and as such, not all commands are registerable.
/// Thus, you can listen for this event and handle it yourself if you wish to build your own command syntax for e.g. a DSL.
/// Handling this event is optional for dialogue views.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteCommandEvent {
    /// The command to execute.
    pub command: UnderlyingYarnCommand,
    /// The [`DialogueRunner`] that is executing this command.
    pub source: Entity,
}

/// An event that is fired after a node has been completed, i.e. all of its lines, commands, options, etc. have been exhausted.
/// Handling this event is **optional** for dialogue views.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeCompleteEvent {
    /// The name of the node that has been completed.
    pub node_name: String,
    /// The [`DialogueRunner`] that has completed this node.
    pub source: Entity,
}

/// An event that is fired after a node has been started, i.e. the first line, command, option, etc. has been executed.
/// Handling this event is **optional** for dialogue views
#[derive(Debug, Clone, PartialEq)]
pub struct NodeStartEvent {
    /// The name of the node that has been started.
    pub node_name: String,
    /// The [`DialogueRunner`] that has started this node.
    pub source: Entity,
}

/// An event that is fired when a new node has been started. Contains the IDs of all lines in the node as a general hint
/// for asset providing systems to pre-load the lines. The lines are not guaranteed to be presented in the order of the IDs or at all.
/// Handling this event is **optional** for dialogue views.
#[derive(Debug, Clone, PartialEq)]
pub struct LineHintsEvent {
    /// The IDs of all lines in the node.
    pub line_ids: Vec<LineId>,
    /// The [`DialogueRunner`] that has started this node.
    pub source: Entity,
}

/// An event that is fired when a dialogue has been started via [`DialogueRunner::start_node`]/
/// Handling this event is **optional** for dialogue views.
#[derive(Debug, Clone, PartialEq)]
pub struct DialogueStartEvent {
    /// The [`DialogueRunner`] that has started this dialogue.
    pub source: Entity,
}

/// An event that is fired when a dialogue has been completed or stopped via [`DialogueRunner::stop`].
#[derive(Debug, Clone, PartialEq)]
pub struct DialogueCompleteEvent {
    /// The [`DialogueRunner`] that has completed this dialogue.
    pub source: Entity,
}
