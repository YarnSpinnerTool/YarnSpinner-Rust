use crate::prelude::YarnValue;
use bevy::ecs::system::{SystemParam, SystemParamItem, SystemState};
use bevy::prelude::*;
use bevy::utils::all_tuples;
use std::marker::PhantomData;
use yarn_slinger::core::{YarnFnParam, YarnFnParamItem, YarnValueWrapper};

pub(crate) fn commands_plugin(_app: &mut App) {}

pub trait YarnCommand<Marker>: Send + Sync + 'static {
    type In: YarnFnParam;
    type Param: SystemParam;

    fn run(&mut self, input: YarnFnParamItem<Self::In>, param_value: SystemParamItem<Self::Param>);
}

macro_rules! impl_command_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Input, Func: Send + Sync + 'static, $($param: SystemParam),*> YarnCommand<fn(In<Input>, $($param,)*)> for Func
        where
            Input: YarnFnParam,
        for <'a> &'a mut Func:
                FnMut(In<Input>, $($param), *) +
                FnMut(In<Input>, $(SystemParamItem<$param>),*) +
                FnMut(In<YarnFnParamItem<Input>>, $($param), *) +
                FnMut(In<YarnFnParamItem<Input>>, $(SystemParamItem<$param>),*)
        {
            type In = Input;
            type Param = ($($param,)*);
            #[inline]
            fn run(&mut self, input: YarnFnParamItem<Input>, param_value: SystemParamItem< ($($param,)*)>) {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<Input: YarnFnParam, $($param,)*>(
                    mut f: impl FnMut(In<YarnFnParamItem<Input>>, $($param,)*),
                    input: In<YarnFnParamItem<Input>>,
                    $($param: $param,)*
                ){
                    f(input, $($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, In(input), $($param),*)
            }
        }
    };
}

// Note that we rely on the highest impl to be <= the highest order of the tuple impls
// of `SystemParam` created.
all_tuples!(impl_command_function, 0, 1, F);

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
        let input = T::In::retrieve(&mut iter);
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
    fn accepts_empty_tuple_in_param() {
        fn f(_: In<()>) {}
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

    #[test]
    fn accepts_only_query() {
        fn f(_: Query<Entity>) {}
        accepts_yarn_command(f);
    }

    #[test]
    fn accepts_empty_tuple_in_param_and_query() {
        fn f(_: In<()>, _: Query<Entity>) {}
        accepts_yarn_command(f);
    }

    #[test]
    fn accepts_function_with_simple_in_param_and_query() {
        fn f(_: In<usize>, _: Query<Entity>) {}
        accepts_yarn_command(f);
    }

    #[test]
    fn accepts_function_with_tuple_in_param_and_query() {
        fn f(_: In<(usize, isize, (String, &str))>, _: Query<Entity>) {}
        accepts_yarn_command(f);
    }

    fn accepts_yarn_command<Marker>(_: impl YarnCommand<Marker>) {}
}
