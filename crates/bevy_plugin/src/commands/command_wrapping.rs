use crate::prelude::YarnValue;
use bevy::ecs::system::{SystemParam, SystemParamItem, SystemState};
use bevy::prelude::*;
use bevy::utils::all_tuples;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use yarn_slinger::core::{YarnFnParam, YarnFnParamItem, YarnValueWrapper};

pub(crate) fn command_wrapping_plugin(_app: &mut App) {}

pub trait YarnCommandFn<Marker>: Send + Sync + 'static {
    type In: YarnFnParam;
    type Param: SystemParam;

    fn run(&mut self, input: YarnFnParamItem<Self::In>, param_value: SystemParamItem<Self::Param>);
}

macro_rules! impl_command_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Input, Func: Send + Sync + 'static, $($param: SystemParam),*> YarnCommandFn<fn(In<Input>, $($param,)*)> for Func
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
all_tuples!(impl_command_function, 0, 16, F);

pub trait UntypedYarnCommandFn: Debug + Send + Sync + 'static {
    fn call(&mut self, input: Vec<YarnValue>, world: &mut World);
}

impl<T, Marker> UntypedYarnCommandFn for YarnCommandFnWrapper<Marker, T>
where
    Marker: 'static,
    T: YarnCommandFn<Marker>,
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
        YarnCommandFn::run(&mut self.function, input, param);
        system_state.apply(world);
    }
}

pub(crate) struct YarnCommandFnWrapper<Marker, F>
where
    F: YarnCommandFn<Marker>,
{
    function: F,

    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    _marker: PhantomData<fn() -> Marker>,
}

impl<Marker, F> From<F> for YarnCommandFnWrapper<Marker, F>
where
    F: YarnCommandFn<Marker>,
{
    fn from(function: F) -> Self {
        Self {
            function,
            _marker: PhantomData,
        }
    }
}

impl<Marker, F> Debug for YarnCommandFnWrapper<Marker, F>
where
    F: YarnCommandFn<Marker>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = std::any::type_name::<Marker>();
        let function_path = std::any::type_name::<F>();
        let debug_message = format!("{signature} {{{function_path}}}");
        f.debug_struct(&debug_message).finish()
    }
}

impl<Marker, F> Display for YarnCommandFnWrapper<Marker, F>
where
    F: YarnCommandFn<Marker>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = std::any::type_name::<Marker>();
        f.write_str(signature)
    }
}

impl PartialEq for Box<dyn UntypedYarnCommandFn> {
    fn eq(&self, other: &Self) -> bool {
        // Not guaranteed to be unique, but it's good enough for our purposes.
        let debug = format!("{:?}", self);
        let other_debug = format!("{:?}", other);
        debug == other_debug
    }
}

impl Eq for Box<dyn UntypedYarnCommandFn> {}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn accepts_empty_function() {
        fn _f() {}
        // Currently not supported
        // accepts_yarn_command(f);
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
    #[ignore]
    fn accepts_only_query() {
        fn _f(_: Query<Entity>) {}
        // Currently not supported
        // accepts_yarn_command(f);
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

    fn accepts_yarn_command<Marker>(_: impl YarnCommandFn<Marker>) {}
}
