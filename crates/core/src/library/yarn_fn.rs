//! Inspired by <https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.SystemParamFunction.html> and <https://docs.rs/bevy_app/latest/bevy_app/struct.App.html>

use crate::prelude::Value;
use rusty_yarn_spinner_macros::all_tuples;
use std::any::TypeId;
use std::borrow::Cow;
use std::marker::PhantomData;

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/type.BoxedSystem.html>
pub type BoxedYarnFn = Box<dyn YarnFnSystem>;

#[derive(Debug, Clone)]
pub struct ProvidedValues(Vec<Value>);

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/trait.System.html>
pub trait YarnFnSystem {
    fn name(&self) -> Cow<'static, str>;
    fn type_id(&self) -> TypeId;
    fn is_send(&self) -> bool;

    fn run(&self, provided_values: &ProvidedValues) -> Value;
}

impl<Marker, F> YarnFnSystem for YarnFnContainer<Marker, F>
where
    Marker: 'static,
    F: YarnFn<Marker>,
{
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
    fn run(&self, provided_values: &ProvidedValues) -> Value {
        let params = F::Param::get_param(&self.system_meta, provided_values);
        self.func.run(params)
    }
}

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/trait.SystemParamFunction.html>
pub trait YarnFn<Item>: Send + Sync + 'static {
    type Param: YarnFnParam<Item>;

    fn run(&self, param_value: Item) -> Value;
}

/// Analogue to <https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/trait.SystemParam.html>
pub trait YarnFnParam<Item> {
    fn get_param(system_meta: &YarnFnMeta, provided_values: &ProvidedValues) -> Item;
}

macro_rules! impl_yarn_fn_param_tuple {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<P, $($param),*> YarnFnParam<($($param,)*)> for P
        where
            $($param: TryInto<Value>),*
        {
            #[inline]
            fn get_param(_system_meta: &YarnFnMeta, provided_values: &ProvidedValues) -> ($($param,)*) {
                if let [$($param),*] = &provided_values[..] {
                    ($($param.try_into().expect("Failed to cast provided value to expected type"),)*)
                } else {
                    panic!("Expected {} parameters, but got {}", stringify!($($param),*), provided_values.0.len()) // TODO: Add more info like name of function
                }
            }
        }
    };
}
all_tuples!(impl_yarn_fn_param_tuple, 0, 1, T);

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
        impl<Func: Send + Sync + 'static, $($param),*> YarnFn<fn($($param,)*) -> Value> for Func
        where
        for <'a> &'a Func:
                Fn($($param),*) -> Value
        {
            type Param = ($($param,)*);
            #[inline]
            fn run(&self, param_value: ($($param,)*)) -> Value {
                // Yes, this is strange, but `rustc` fails to compile this impl
                // without using this function. It fails to recognise that `func`
                // is a function, potentially because of the multiple impls of `Fn`
                #[allow(clippy::too_many_arguments)]
                fn call_inner<$($param,)*>(
                    mut f: impl Fn($($param,)*)->Value,
                    $($param: $param,)*
                )->Value{
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
all_tuples!(impl_system_function, 0, 1, F);
