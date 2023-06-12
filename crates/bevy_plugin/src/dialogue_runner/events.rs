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
/// A UI should listen for this event and draw it to the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct PresentLineEvent {
    /// The line to present to the user.
    pub line: LocalizedLine,
    /// The [`DialogueRunner`] that is presenting this line.
    pub source: Entity,
}

/// An event that is fired after a dialogue advances and wishes to present a set of options to the user.
/// A UI should listen for this event and draw it to the screen.
/// You need to handle this event by calling [`DialogueRunner::select_option`] with the ID found in the provided [`DialogueOption`]s.
#[derive(Debug, Clone, PartialEq)]
pub struct PresentOptionsEvent {
    /// The options to present to the user.
    pub options: Vec<DialogueOption>,
    /// The [`DialogueRunner`] that is presenting these options.
    pub source: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteCommandEvent {
    pub command: UnderlyingYarnCommand,
    pub source: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeCompleteEvent {
    pub node_name: String,
    pub source: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeStartEvent {
    pub node_name: String,
    pub source: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineHintsEvent {
    pub line_ids: Vec<LineId>,
    pub source: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogueStartEvent {
    pub source: Entity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogueCompleteEvent {
    pub source: Entity,
}
