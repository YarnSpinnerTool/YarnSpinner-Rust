use bevy::ecs::event::EventCursor;
use bevy::prelude::*;
use bevy_yarnspinner::events::*;

#[derive(Debug, Default)]
pub struct EventAsserter {
    pub present_line_reader: EventCursor<PresentLineEvent>,
    pub present_options_reader: EventCursor<PresentOptionsEvent>,
    pub dialogue_start_reader: EventCursor<DialogueStartEvent>,
    pub dialogue_complete_reader: EventCursor<DialogueCompleteEvent>,
    pub node_start_reader: EventCursor<NodeStartEvent>,
    pub node_complete_reader: EventCursor<NodeCompleteEvent>,
    pub line_hints_reader: EventCursor<LineHintsEvent>,
    pub execute_command_reader: EventCursor<ExecuteCommandEvent>,
}

impl EventAsserter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_events(&mut self, app: &mut App) {
        self.present_line_reader
            .clear(app.world().resource::<Events<PresentLineEvent>>());
        self.present_options_reader
            .clear(app.world().resource::<Events<PresentOptionsEvent>>());
        self.dialogue_start_reader
            .clear(app.world().resource::<Events<DialogueStartEvent>>());
        self.dialogue_complete_reader
            .clear(app.world().resource::<Events<DialogueCompleteEvent>>());
        self.node_start_reader
            .clear(app.world().resource::<Events<NodeStartEvent>>());
        self.node_complete_reader
            .clear(app.world().resource::<Events<NodeCompleteEvent>>());
        self.line_hints_reader
            .clear(app.world().resource::<Events<LineHintsEvent>>());
        self.execute_command_reader
            .clear(app.world().resource::<Events<ExecuteCommandEvent>>());
    }
}

#[macro_export]
macro_rules! get_reader {
    ($asserter:ident, PresentLineEvent) => {
        &mut $asserter.present_line_reader
    };
    ($asserter:ident, PresentOptionsEvent) => {
        &mut $asserter.present_options_reader
    };
    ($asserter:ident, DialogueStartEvent) => {
        &mut $asserter.dialogue_start_reader
    };
    ($asserter:ident, DialogueCompleteEvent) => {
        &mut $asserter.dialogue_complete_reader
    };
    ($asserter:ident, NodeStartEvent) => {
        &mut $asserter.node_start_reader
    };
    ($asserter:ident, NodeCompleteEvent) => {
        &mut $asserter.node_complete_reader
    };
    ($asserter:ident, LineHintsEvent) => {
        &mut $asserter.line_hints_reader
    };
    ($asserter:ident, ExecuteCommandEvent) => {
        &mut $asserter.execute_command_reader
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
        let events = $app.world().resource::<bevy::prelude::Events<$event>>();
        let reader = $crate::get_reader!($asserter, $event);
        let events: Vec<&$event> = reader.read(&events).collect();
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
