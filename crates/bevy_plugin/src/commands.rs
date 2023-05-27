use crate::prelude::YarnValue;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::prelude::*;
use std::marker::PhantomData;
use yarn_slinger::core::{YarnFnParam, YarnValueWrapper};

pub(crate) fn commands_plugin(_app: &mut App) {}

pub trait YarnCommand<Marker>:
    Send + Sync + 'static + SystemParamFunction<Marker, Out = ()>
{
    type ConstrainedIn: YarnFnParam;
    fn run(
        &mut self,
        input: <Self::ConstrainedIn as YarnFnParam>::Item<'_>,
        param_value: <Self::Param as SystemParam>::Item<'_, '_>,
    );
}

impl<T, U, Marker> YarnCommand<Marker> for T
where
    T: SystemParamFunction<Marker, In = U, Out = ()>,
    U: for<'a> YarnFnParam<Item<'a> = U>,
{
    type ConstrainedIn = U;

    fn run(
        &mut self,
        input: <Self::ConstrainedIn as YarnFnParam>::Item<'_>,
        param_value: <Self::Param as SystemParam>::Item<'_, '_>,
    ) {
        self.run(input, param_value);
    }
}

pub trait UntypedYarnCommand: Send + Sync + 'static {
    fn call(&mut self, input: Vec<YarnValue>, world: &mut World);
}

impl<T, Marker> UntypedYarnCommand for YarnCommandWrapper<Marker, T>
where
    Marker: 'static,
    T: YarnCommand<Marker>,
{
    fn call(&mut self, input: Vec<YarnValue>, world: &mut World) {
        let mut system_state: SystemState<T::Param> = SystemState::new(world);
        let param = system_state.get_mut(world);
        let mut input: Vec<_> = input.into_iter().map(YarnValueWrapper::from).collect();
        let mut iter = input.iter_mut();
        let input = T::ConstrainedIn::retrieve(&mut iter);
        assert!(
            iter.next().is_none(),
            "Passed too many arguments to Command"
        );
        YarnCommand::run(&mut self.function, input, param);
        system_state.apply(world);
    }
}

pub(crate) struct YarnCommandWrapper<Marker, F>
where
    F: YarnCommand<Marker>,
{
    function: F,

    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    _marker: PhantomData<fn() -> Marker>,
}

impl<Marker, F> From<F> for YarnCommandWrapper<Marker, F>
where
    F: YarnCommand<Marker>,
{
    fn from(function: F) -> Self {
        Self {
            function,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn accepts_empty_function() {
        fn f() {}
        accepts_yarn_command(f);
    }

    #[test]
    fn accepts_function_with_simple_in_param() {
        fn f(_: In<usize>) {}
        accepts_yarn_command(f);
    }

    #[test]
    fn accepts_function_with_tuple_in_param() {
        fn f(_: In<(usize, isize, (String, &str))>) {}
        accepts_yarn_command(f);
    }

    fn accepts_yarn_command<Marker>(_: impl YarnCommand<Marker>) {}
}
