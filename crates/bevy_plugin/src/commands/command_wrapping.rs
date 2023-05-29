use crate::prelude::YarnValue;
use bevy::ecs::system::{SystemParam, SystemParamItem, SystemState};
use bevy::prelude::*;
use bevy::tasks::Task;
use bevy::utils::all_tuples;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use yarn_slinger::core::{YarnFnParam, YarnFnParamItem, YarnValueWrapper};

pub(crate) fn command_wrapping_plugin(_app: &mut App) {}

pub trait YarnCommand<Marker>: Send + Sync + 'static {
    type In: YarnFnParam;
    type Param: SystemParam;

    fn run(
        &mut self,
        input: YarnFnParamItem<Self::In>,
        param_value: SystemParamItem<Self::Param>,
    ) -> Option<Task<()>>;
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
            fn run(&mut self, input: YarnFnParamItem<Input>, param_value: SystemParamItem< ($($param,)*)>) -> Option<Task<()>> {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<Input: YarnFnParam, $($param,)*>(
                    mut f: impl FnMut(In<YarnFnParamItem<Input>>, $($param,)*),
                    input: In<YarnFnParamItem<Input>>,
                    $($param: $param,)*
                ){
                    f(input, $($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, In(input), $($param),*);
                None
            }
        }

        #[allow(non_snake_case)]
        impl<Input, Func: Send + Sync + 'static, $($param: SystemParam),*> YarnCommand<fn(In<Input>, $($param,)*) -> Task<()>> for Func
        where
            Input: YarnFnParam,
        for <'a> &'a mut Func:
                FnMut(In<Input>, $($param), *) -> Task<()> +
                FnMut(In<Input>, $(SystemParamItem<$param>),*) -> Task<()> +
                FnMut(In<YarnFnParamItem<Input>>, $($param), *) -> Task<()> +
                FnMut(In<YarnFnParamItem<Input>>, $(SystemParamItem<$param>),*) -> Task<()>
        {
            type In = Input;
            type Param = ($($param,)*);
            #[inline]
            fn run(&mut self, input: YarnFnParamItem<Input>, param_value: SystemParamItem< ($($param,)*)>) -> Option<Task<()>> {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<Input: YarnFnParam, $($param,)*>(
                    mut f: impl FnMut(In<YarnFnParamItem<Input>>, $($param,)*) -> Task<()>,
                    input: In<YarnFnParamItem<Input>>,
                    $($param: $param,)*
                ) -> Task<()>{
                    f(input, $($param,)*)
                }
                let ($($param,)*) = param_value;
                let task = call_inner(self, In(input), $($param),*);
                Some(task)
            }
        }
    };
}

// Note that we rely on the highest impl to be <= the highest order of the tuple impls
// of `SystemParam` created.
all_tuples!(impl_command_function, 0, 16, F);

pub trait UntypedYarnCommand: Debug + Send + Sync + 'static {
    fn call(&mut self, input: Vec<YarnValue>, world: &mut World) -> Option<Task<()>>;
}

impl<T, Marker> UntypedYarnCommand for YarnCommandWrapper<Marker, T>
where
    Marker: 'static,
    T: YarnCommand<Marker>,
{
    fn call(&mut self, input: Vec<YarnValue>, world: &mut World) -> Option<Task<()>> {
        let mut system_state: SystemState<T::Param> = SystemState::new(world);
        let param = system_state.get_mut(world);
        let mut input: Vec<_> = input.into_iter().map(YarnValueWrapper::from).collect();
        let mut iter = input.iter_mut();
        let input = T::In::retrieve(&mut iter);
        assert!(
            iter.next().is_none(),
            "Passed too many arguments to Command"
        );
        let task = YarnCommand::run(&mut self.function, input, param);
        system_state.apply(world);
        task
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

impl<Marker, F> Debug for YarnCommandWrapper<Marker, F>
where
    F: YarnCommand<Marker>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = std::any::type_name::<Marker>();
        let function_path = std::any::type_name::<F>();
        let debug_message = format!("{signature} {{{function_path}}}");
        f.debug_struct(&debug_message).finish()
    }
}

impl<Marker, F> Display for YarnCommandWrapper<Marker, F>
where
    F: YarnCommand<Marker>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = std::any::type_name::<Marker>();
        f.write_str(signature)
    }
}

impl PartialEq for Box<dyn UntypedYarnCommand> {
    fn eq(&self, other: &Self) -> bool {
        // Not guaranteed to be unique, but it's good enough for our purposes.
        let debug = format!("{:?}", self);
        let other_debug = format!("{:?}", other);
        debug == other_debug
    }
}

impl Eq for Box<dyn UntypedYarnCommand> {}

#[cfg(test)]
pub mod tests {
    use super::*;
    use bevy::tasks::AsyncComputeTaskPool;

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

    #[test]
    fn accepts_returning_task() {
        fn f(_: In<()>) -> Task<()> {
            let thread_pool = AsyncComputeTaskPool::get();
            thread_pool.spawn(async move {
                println!("Hello from task");
            })
        }
        accepts_yarn_command(f);
    }

    fn accepts_yarn_command<Marker>(_: impl YarnCommand<Marker>) {}
}
