use bevy::ecs::message::MessageCursor;
use bevy::prelude::*;
use bevy_yarnspinner::events::*;

#[derive(Debug, Default)]
pub struct EventAsserter {
    pub present_line_cursor: MessageCursor<PresentLineEvent>,
    pub present_options_cursor: MessageCursor<PresentOptionsEvent>,
    pub dialogue_start_cursor: MessageCursor<DialogueStartEvent>,
    pub dialogue_complete_cursor: MessageCursor<DialogueCompleteEvent>,
    pub node_start_cursor: MessageCursor<NodeStartEvent>,
    pub node_complete_cursor: MessageCursor<NodeCompleteEvent>,
    pub line_hints_cursor: MessageCursor<LineHintsEvent>,
    pub execute_command_cursor: MessageCursor<ExecuteCommandEvent>,
}

impl EventAsserter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_events(&mut self, app: &mut App) {
        self.present_line_cursor
            .clear(app.world().resource::<Messages<PresentLineEvent>>());
        self.present_options_cursor
            .clear(app.world().resource::<Messages<PresentOptionsEvent>>());
        self.dialogue_start_cursor
            .clear(app.world().resource::<Messages<DialogueStartEvent>>());
        self.dialogue_complete_cursor
            .clear(app.world().resource::<Messages<DialogueCompleteEvent>>());
        self.node_start_cursor
            .clear(app.world().resource::<Messages<NodeStartEvent>>());
        self.node_complete_cursor
            .clear(app.world().resource::<Messages<NodeCompleteEvent>>());
        self.line_hints_cursor
            .clear(app.world().resource::<Messages<LineHintsEvent>>());
        self.execute_command_cursor
            .clear(app.world().resource::<Messages<ExecuteCommandEvent>>());
    }
}

#[macro_export]
macro_rules! get_cursor {
    ($asserter:ident, PresentLineEvent) => {
        &mut $asserter.present_line_cursor
    };
    ($asserter:ident, PresentOptionsEvent) => {
        &mut $asserter.present_options_cursor
    };
    ($asserter:ident, DialogueStartEvent) => {
        &mut $asserter.dialogue_start_cursor
    };
    ($asserter:ident, DialogueCompleteEvent) => {
        &mut $asserter.dialogue_complete_cursor
    };
    ($asserter:ident, NodeStartEvent) => {
        &mut $asserter.node_start_cursor
    };
    ($asserter:ident, NodeCompleteEvent) => {
        &mut $asserter.node_complete_cursor
    };
    ($asserter:ident, LineHintsEvent) => {
        &mut $asserter.line_hints_cursor
    };
    ($asserter:ident, ExecuteCommandEvent) => {
        &mut $asserter.execute_command_cursor
    };
}

#[macro_export]
macro_rules! assert_events {
    ($asserter:ident, $app:ident contains [$($event:ident $((n = $num:expr))? $(with $pred:expr)?) ,* $(,)?]) => {
        $(
            { assert_events!($asserter, $app contains $event $((n = $num))? $(with $pred)?); }
        )*
    };
    ($asserter:ident, $app:ident contains $event:ident $(with $pred:expr)?) => {
        assert_events!($asserter, $app contains $event (n = 1) $(with $pred)?);
    };
    ($asserter:ident, $app:ident contains $event:ident (n = $num:expr) $(with $pred:expr)?) => {
        let events = $app.world().resource::<bevy::prelude::Messages<$event>>();
        let cursor = $crate::get_cursor!($asserter, $event);
        let events: Vec<&$event> = cursor.read(&events).collect();
        assert_eq!($num, events.len(), "Expected {} events of type {}, but found {}: {events:#?}", stringify!($num), stringify!($event), events.len());
        $(
            {
                fn get_pred() -> impl Fn(&$event) -> bool {
                    $pred
                }
                let pred = get_pred();
                let actual = events.into_iter().next().unwrap();
                assert!(pred(actual), "Expected event of type {} to fulfill predicate {}, but found {:#?}", stringify!($event), stringify!($pred), actual);
            }
        )?
    };
}
