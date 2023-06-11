use crate::commands::command_wrapping::YarnCommandWrapper;
use crate::commands::UntypedYarnCommand;
use crate::prelude::*;
use bevy::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Duration;

mod wait;

pub(crate) fn command_registry_plugin(app: &mut App) {
    app.fn_plugin(wait::wait_command_plugin);
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct YarnCommandRegistrations(pub(crate) InnerRegistry);

type InnerRegistry = HashMap<Cow<'static, str>, Box<dyn UntypedYarnCommand>>;

impl Extend<<InnerRegistry as IntoIterator>::Item> for YarnCommandRegistrations {
    fn extend<T: IntoIterator<Item = <InnerRegistry as IntoIterator>::Item>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for YarnCommandRegistrations {
    type Item = <InnerRegistry as IntoIterator>::Item;
    type IntoIter = <InnerRegistry as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl YarnCommandRegistrations {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new method to the registry. See [`YarnFn`]'s documentation for what kinds of methods are allowed.
    pub fn register_command<Marker, F>(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        command: F,
    ) -> &mut Self
    where
        Marker: 'static,
        F: YarnCommand<Marker> + 'static + Clone,
    {
        let name = name.into();
        let wrapped = YarnCommandWrapper::from(command);
        self.0.insert(name, Box::new(wrapped));
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &(dyn UntypedYarnCommand))> {
        self.0
            .iter()
            .map(|(key, value)| (key.as_ref(), value.as_ref()))
    }

    pub fn add_boxed(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        command: Box<dyn UntypedYarnCommand>,
    ) -> &mut Self {
        let name = name.into();
        self.0.insert(name, command);
        self
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&(dyn UntypedYarnCommand)> {
        self.0.get(name).map(|f| f.as_ref())
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut (dyn UntypedYarnCommand)> {
        self.0.get_mut(name).map(|f| f.as_mut())
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|key| key.as_ref())
    }

    pub fn commands(&self) -> impl Iterator<Item = &(dyn UntypedYarnCommand)> {
        self.0.values().map(|value| value.as_ref())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn builtin_commands() -> Self {
        let mut commands = Self::default();
        commands
            .register_command("wait", |In(duration): In<f32>, mut wait: ResMut<Wait>| {
                wait.add(Duration::from_secs_f32(duration))
            })
            .register_command("stop", |_: In<()>| {
                unreachable!("The stop command is a compiler builtin and is thus not callable")
            });
        commands
    }
}

#[macro_export]
macro_rules! yarn_commands {
    ($($name:expr => $function:expr,)*) => {
        {
            let mut map = YarnCommands::default();
            $(
                map.register_command($name, $function);
            )*
            map
        }
    };
}

use crate::commands::command_registry::wait::Wait;
pub use yarn_commands;

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::tasks::{AsyncComputeTaskPool, Task, TaskPool};
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn can_add_fn_with_empty_tuple_in_args() {
        let mut methods = YarnCommandRegistrations::default();
        methods.register_command("test", |_: In<()>| {});
    }

    #[test]
    fn can_add_fn_with_one_in_arg() {
        let mut methods = YarnCommandRegistrations::default();
        methods.register_command("test", |_: In<f32>| {});
    }

    #[test]
    #[should_panic = "It works!"]
    fn can_call_fn_with_no_args() {
        let mut methods = YarnCommandRegistrations::default();

        methods.register_command("test", |_: In<()>| panic!("It works!"));
        let method = methods.get_mut("test").unwrap();
        let mut app = App::new();
        method.call(vec![], &mut app.world);
    }

    #[test]
    fn can_call_fn_with_one_arg() {
        let mut methods = YarnCommandRegistrations::default();

        methods.register_command("test", |In(a): In<f32>| assert_eq!(1.0, a));
        let method = methods.get_mut("test").unwrap();
        let mut app = App::new();
        method.call(to_method_params([1.0]), &mut app.world);
    }

    #[test]
    fn can_add_multiple_fns() {
        let mut methods = YarnCommandRegistrations::default();

        methods.register_command("test1", |_: In<()>| {});
        methods.register_command("test2", |_: In<f32>| {});
    }

    #[test]
    fn can_call_multiple_fns() {
        let mut methods = YarnCommandRegistrations::default();
        methods.register_command("test1", |_: In<()>| {});
        methods.register_command("test2", |In(a): In<f32>| assert_eq!(1.0, a));

        let mut app = App::new();
        {
            let method1 = methods.get_mut("test1").unwrap();
            method1.call(vec![], &mut app.world);
        }
        let method2 = methods.get_mut("test2").unwrap();
        method2.call(to_method_params([1.0]), &mut app.world);
    }

    #[test]
    fn can_mutate_world() {
        let mut methods = YarnCommandRegistrations::default();
        methods.register_command("test", |In(a): In<f32>, mut commands: Commands| {
            commands.insert_resource(Data(a))
        });

        #[derive(Resource)]
        struct Data(f32);

        let method = methods.get_mut("test").unwrap();

        let mut app = App::new();
        method.call(to_method_params([1.0]), &mut app.world);
        let data = app.world.resource::<Data>();
        assert_eq!(data.0, 1.0);
    }

    fn to_method_params(params: impl IntoIterator<Item = impl Into<YarnValue>>) -> Vec<YarnValue> {
        params.into_iter().map(Into::into).collect()
    }

    #[test]
    fn executes_task() {
        AsyncComputeTaskPool::init(TaskPool::new);

        let mut methods = YarnCommandRegistrations::default();
        methods.register_command("test", |_: In<()>| -> Task<()> {
            let thread_pool = AsyncComputeTaskPool::get();
            thread_pool.spawn(async move { sleep(Duration::from_millis(500)) })
        });
        let method = methods.get_mut("test").unwrap();

        let mut app = App::new();
        let task = method.call(vec![], &mut app.world);
        assert!(!task.is_finished());
        sleep(Duration::from_millis(600));
        assert!(task.is_finished());
    }

    #[test]
    fn debug_prints_signature() {
        let mut methods = YarnCommandRegistrations::default();

        methods.register_command("test", |_: In<(f32, f32)>| {});
        let debug_string = format!("{:?}", methods);

        let element_start = debug_string.find('{').unwrap();
        // This looks like an off-by-one error on closer inspection,
        // but on even closer inspection it's correct because there's a space before the second '{' that we don't want to include.
        let element_end = element_start + debug_string[element_start + 1..].find('{').unwrap();
        let element = &debug_string[element_start..element_end];

        // Not testing the part after because its stability is not guaranteed.
        assert_eq!(
            element,
            "{\"test\": fn(bevy_ecs::system::function_system::In<(f32, f32)>)"
        );
    }
}
