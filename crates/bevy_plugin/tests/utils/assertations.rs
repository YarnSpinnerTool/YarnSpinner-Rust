#[macro_export]
macro_rules! assert_events {
    ($app:ident contains [$($event:ident $((n = $num:expr))? $(with $pred:expr)?) ,* $(,)?]) => {
        $(
            assert_events!($app contains $event $((n = $num))? $(with $pred)?);
        )*
    };
    ($app:ident contains $event:ident $(with $pred:expr)?) => {
        assert_events!($app contains $event (n = 1) $(with $pred)?);
    };
    ($app:ident contains $event:ident (n = $num:expr) $(with $pred:expr)?) => {
        let events = $app.world.resource::<bevy::prelude::Events<$event>>();
        let events: Vec<&$event> = events.iter_current_update_events().collect();
        assert_eq!($num, events.len(), "Expected {} events of type {}, but found {}", stringify!($num), stringify!($event), events.len());
        $(
            fn get_pred() -> impl Fn(&$event) -> bool {
                $pred
            }
            let pred = get_pred();
            let actual = events.into_iter().next().unwrap();
            assert!(pred(actual), "Expected event of type {} to fulfill predicate {}, but found {:#?}", stringify!($event), stringify!($pred), actual);
        )?
    };
}

pub use assert_events;
