use bevy::ecs::event::ManualEventReader;
use bevy::prelude::*;
use bevy_yarn_slinger::events::*;

#[derive(Debug, Default)]
pub struct EventAsserter {
    pub present_line_reader: ManualEventReader<PresentLineEvent>,
    pub present_options_reader: ManualEventReader<PresentOptionsEvent>,
    pub dialogue_start_reader: ManualEventReader<DialogueStartEvent>,
    pub dialogue_complete_reader: ManualEventReader<DialogueCompleteEvent>,
    pub node_start_reader: ManualEventReader<NodeStartEvent>,
    pub node_complete_reader: ManualEventReader<NodeCompleteEvent>,
    pub line_hints_reader: ManualEventReader<LineHintsEvent>,
    pub execute_command_reader: ManualEventReader<ExecuteCommandEvent>,
}

impl EventAsserter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear_events(&mut self, app: &mut App) {
        self.present_line_reader
            .clear(&mut app.world.resource::<Events<PresentLineEvent>>());
        self.present_options_reader
            .clear(&mut app.world.resource::<Events<PresentOptionsEvent>>());
        self.dialogue_start_reader
            .clear(&mut app.world.resource::<Events<DialogueStartEvent>>());
        self.dialogue_complete_reader
            .clear(&mut app.world.resource::<Events<DialogueCompleteEvent>>());
        self.node_start_reader
            .clear(&mut app.world.resource::<Events<NodeStartEvent>>());
        self.node_complete_reader
            .clear(&mut app.world.resource::<Events<NodeCompleteEvent>>());
        self.line_hints_reader
            .clear(&mut app.world.resource::<Events<LineHintsEvent>>());
        self.execute_command_reader
            .clear(&mut app.world.resource::<Events<ExecuteCommandEvent>>());
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
pub use get_reader;

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
        let events = $app.world.resource::<bevy::prelude::Events<$event>>();
        let reader = crate::prelude::get_reader!($asserter, $event);
        let events: Vec<&$event> = reader.iter(&events).collect();
        assert_eq!($num, events.len(), "Expected {} events of type {}, but found {}: {events:?}", stringify!($num), stringify!($event), events.len());
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

pub use assert_events;
