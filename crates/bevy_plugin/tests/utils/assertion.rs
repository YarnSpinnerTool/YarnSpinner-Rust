use bevy::prelude::*;
use bevy_yarnspinner::events::*;
use std::fmt::Debug;

pub struct AssertionPlugin;
impl Plugin for AssertionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(event_assertion::<PresentLine>);
        app.add_observer(event_assertion::<PresentOptions>);
        app.add_observer(event_assertion::<ExecuteCommand>);
        app.add_observer(event_assertion::<NodeCompleted>);
        app.add_observer(event_assertion::<NodeStarted>);
        app.add_observer(event_assertion::<LineHints>);
        app.add_observer(event_assertion::<DialogueStarted>);
        app.add_observer(event_assertion::<DialogueCompleted>);
    }
}

#[derive(Resource)]
pub struct EventAsserter<T: Event + Debug + Clone> {
    pub predicate: Option<Box<dyn Fn(T) -> bool + Send + Sync>>,
    pub expected_calls: usize,
    pub actual_calls: usize,
}
impl<T: Event + Debug + Clone> Default for EventAsserter<T> {
    fn default() -> Self {
        Self {
            predicate: None,
            expected_calls: 0,
            actual_calls: 0,
        }
    }
}
pub fn event_assertion<T: Event + Debug + Clone>(
    event: On<T>,
    mut event_asserter: If<ResMut<EventAsserter<T>>>,
) {
    event_asserter.actual_calls += 1;
    if let Some(predicate) = &event_asserter.predicate {
        assert!(
            predicate(event.event().clone()),
            "Expected event of type {} to fulfill predicate, but found {:#?}",
            stringify!(T),
            event.event()
        );
    }
}

#[macro_export]
macro_rules! assert_events {
    ($app:ident contains [$($event:ident $((n = $num:expr))? $(with $pred:expr)?) ,* $(,)?]) => {
        $(
            { assert_events!($app contains $event $((n = $num))? $(with $pred)?); }
        )*
        $app.update();
        $(
            $app.clear_and_assert_event::<$event>();
        )*
    };
    ($app:ident contains $event:ident $(with $pred:expr)?) => {
        assert_events!($app contains $event (n = 1) $(with $pred)?);
    };
    ($app:ident contains $event:ident (n = $num:expr) $(with $pred:expr)?) => {
        let mut event_asserter = EventAsserter::<$event>::default();
        event_asserter.expected_calls = $num;
        $(
            {
                event_asserter.predicate = Some(Box::new($pred));
            }
        )?
        $app.world_mut().insert_resource(event_asserter);
    };
}
