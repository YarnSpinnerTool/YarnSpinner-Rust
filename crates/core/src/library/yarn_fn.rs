//! Inspired by <https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.SystemParamFunction.html> and <https://docs.rs/bevy_app/latest/bevy_app/struct.App.html>

use crate::prelude::Value;
use rusty_yarn_spinner_macros::all_tuples;
use std::any::TypeId;
use std::borrow::Cow;
use std::marker::PhantomData;

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/type.BoxedSystem.html>
pub type BoxedYarnFn<Out = Value> = Box<dyn YarnFnSystem<Out = Out>>;

pub struct ProvidedValues(Vec<Value>);

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/trait.System.html>
pub trait YarnFnSystem {
    type Out;
    fn name(&self) -> Cow<'static, str>;
    fn type_id(&self) -> TypeId;
    fn is_send(&self) -> bool;

    fn run(&mut self, provided_values: &ProvidedValues) -> Self::Out;
}

impl<Marker, F> YarnFnSystem for YarnFnContainer<Marker, F>
where
    Marker: 'static,
    F: YarnFn<Marker>,
{
    type Out = F::Out;

    #[inline]
    fn name(&self) -> Cow<'static, str> {
        self.system_meta.name.clone()
    }

    #[inline]
    fn type_id(&self) -> TypeId {
        TypeId::of::<F>()
    }

    #[inline]
    fn is_send(&self) -> bool {
        self.system_meta.is_send
    }

    #[inline]
    fn run(&mut self, provided_values: &ProvidedValues) -> Self::Out {
        let params = F::Param::get_param(&self.system_meta, provided_values);
        self.func.run(params)
    }
}

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/trait.SystemParamFunction.html>
pub trait YarnFn<Marker>: Send + Sync + 'static {
    type Param: YarnFnParam;
    type Out;

    fn run(&mut self, param_value: YarnFnParamItem<Self::Param>) -> Self::Out;
}

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/type.SystemParamItem.html>
pub type YarnFnParamItem<P> = <P as YarnFnParam>::Item;

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/trait.SystemParam.html>
pub trait YarnFnParam {
    type Item: YarnFnParam;
    fn get_param(system_meta: &YarnFnMeta, provided_values: &ProvidedValues) -> Self::Item;
}

impl<T> YarnFnParam for T {
    type Item = T;

    fn get_param(system_meta: &YarnFnMeta, provided_values: &ProvidedValues) -> Self::Item {
        todo!()
    }
}

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/struct.FunctionSystem.html>
pub struct YarnFnContainer<Marker, F>
where
    F: YarnFn<Marker>,
{
    func: F,
    system_meta: YarnFnMeta,
    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    marker: PhantomData<fn() -> Marker>,
}

/// Analogue to <https://docs.rs/bevy_ecs/latest/bevy_ecs/system/struct.SystemMeta.html>
#[derive(Clone)]
pub struct YarnFnMeta {
    pub(crate) name: Cow<'static, str>,
    // NOTE: this must be kept private. making a YarnFnMeta non-send is irreversible to prevent
    // SystemParams from overriding each other
    is_send: bool,
}

impl YarnFnMeta {
    pub(crate) fn new<T>() -> Self {
        Self {
            name: std::any::type_name::<T>().into(),
            is_send: true,
        }
    }
    /// Returns the system's name
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if the system is [`Send`].
    #[inline]
    pub fn is_send(&self) -> bool {
        self.is_send
    }

    /// Sets the system to be not [`Send`].
    ///
    /// This is irreversible.
    #[inline]
    pub fn set_non_send(&mut self) {
        self.is_send = false;
    }
}

/// Taken from <https://docs.rs/bevy_ecs/0.10.1/src/bevy_ecs/system/function_system.rs.html#607>
macro_rules! impl_system_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, Func: Send + Sync + 'static, $($param: YarnFnParam),*> YarnFn<fn($($param,)*) -> Out> for Func
        where
        for <'a> &'a mut Func:
                FnMut($($param),*) -> Out +
                FnMut($(YarnFnParamItem<$param>),*) -> Out, Out: 'static
        {
            type Out = Out;
            type Param = ($($param,)*);
            #[inline]
            fn run(&mut self, param_value: YarnFnParamItem< ($($param,)*)>) -> Out {
                // Yes, this is strange, but `rustc` fails to compile this impl
                // without using this function. It fails to recognise that `func`
                // is a function, potentially because of the multiple impls of `FnMut`
                #[allow(clippy::too_many_arguments)]
                fn call_inner<Out, $($param,)*>(
                    mut f: impl FnMut($($param,)*)->Out,
                    $($param: $param,)*
                )->Out{
                    f($($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(self, $($param),*)
            }
        }
    };
}

// Note that we rely on the highest impl to be <= the highest order of the tuple impls
// of `SystemParam` created.
all_tuples!(impl_system_function, 0, 16, F);
