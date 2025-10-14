use bevy::prelude::*;
use bevy_yarnspinner::events::*;

pub struct AssertionPlugin;
impl Plugin for AssertionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TriggeredEvents::<PresentLine>(Vec::new()))
        .insert_resource(TriggeredEvents::<PresentOptions>(Vec::new()))
        .insert_resource(TriggeredEvents::<ExecuteCommand>(Vec::new()))
        .insert_resource(TriggeredEvents::<NodeCompleted>(Vec::new()))
        .insert_resource(TriggeredEvents::<NodeStarted>(Vec::new()))
        .insert_resource(TriggeredEvents::<LineHints>(Vec::new()))
        .insert_resource(TriggeredEvents::<DialogueStarted>(Vec::new()))
        .insert_resource(TriggeredEvents::<DialogueCompleted>(Vec::new()))
        .add_observer(|event: On<PresentLine>, mut triggered_events: ResMut<TriggeredEvents<PresentLine>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_observer(|event: On<PresentOptions>, mut triggered_events: ResMut<TriggeredEvents<PresentOptions>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_observer(|event: On<ExecuteCommand>, mut triggered_events: ResMut<TriggeredEvents<ExecuteCommand>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_observer(|event: On<NodeCompleted>, mut triggered_events: ResMut<TriggeredEvents<NodeCompleted>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_observer(|event: On<NodeStarted>, mut triggered_events: ResMut<TriggeredEvents<NodeStarted>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_observer(|event: On<LineHints>, mut triggered_events: ResMut<TriggeredEvents<LineHints>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_observer(|event: On<DialogueStarted>, mut triggered_events: ResMut<TriggeredEvents<DialogueStarted>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_observer(|event: On<DialogueCompleted>, mut triggered_events: ResMut<TriggeredEvents<DialogueCompleted>>|  {
            triggered_events.0.push(event.event().clone());
        })
        .add_systems(Update, (
            clear_triggered_events::<PresentLine>,
            clear_triggered_events::<PresentOptions>,
            clear_triggered_events::<ExecuteCommand>,
            clear_triggered_events::<NodeCompleted>,
            clear_triggered_events::<NodeStarted>,
            clear_triggered_events::<LineHints>,
            clear_triggered_events::<DialogueStarted>,
            clear_triggered_events::<DialogueCompleted>,
        ));
    }
}
fn clear_triggered_events<T: EntityEvent>(mut bar: ResMut<TriggeredEvents<T>>) {
    bar.0.clear();
}


#[derive(Resource, Debug, Default)]
pub struct TriggeredEvents<T>(pub Vec<T>);

#[macro_export]
macro_rules! assert_events {
    ($app:ident contains [$($event:ident $((n = $num:expr))? $(with $pred:expr)?) ,* $(,)?]) => {
        $(
            { assert_events!($app contains $event $((n = $num))? $(with $pred)?); }
        )*
    };
    ($app:ident contains $event:ident $(with $pred:expr)?) => {
        assert_events!($app contains $event (n = 1) $(with $pred)?);
    };
    ($app:ident contains $event:ident (n = $num:expr) $(with $pred:expr)?) => {
        let events = $app.world().resource::<TriggeredEvents<$event>>();


        assert_eq!($num, events.0.len(), "Expected {} events of type {}, but found {}: {events:#?}", stringify!($num), stringify!($event), events.0.len());
        $(
            {
                fn get_pred() -> impl Fn(&$event) -> bool {
                    $pred
                }
                let pred = get_pred();
                let actual = events.0.iter().next().unwrap();
                assert!(pred(actual), "Expected event of type {} to fulfill predicate {}, but found {:#?}", stringify!($event), stringify!($pred), actual);
            }
        )?
    };
}
