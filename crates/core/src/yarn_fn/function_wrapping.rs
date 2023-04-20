use crate::prelude::Value;
use rusty_yarn_spinner_macros::all_tuples;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub trait YarnFnWithMarker<Marker> {
    type Out: Into<Value>;
    fn call(&self, input: Vec<Value>) -> Self::Out;
}

pub trait YarnFn: Debug {
    fn call(&self, input: Vec<Value>) -> Box<dyn IntoValue>;
    fn clone_box(&self) -> Box<dyn YarnFn>;
}

impl Clone for Box<dyn YarnFn> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<Marker, F> YarnFn for YarnFnWrapper<Marker, F>
where
    Marker: 'static + Clone,
    F: YarnFnWithMarker<Marker> + 'static + Clone,
    F::Out: Into<Value> + 'static + Clone,
{
    fn call(&self, input: Vec<Value>) -> Box<dyn IntoValue> {
        let output = self.function.call(input);
        Box::new(output)
    }

    fn clone_box(&self) -> Box<dyn YarnFn> {
        Box::new(self.clone())
    }
}

/// Necessary because [`Into`] requires `self` to be consumed and thus be [`Sized`], which in turn means no trait objects.
pub trait IntoValue {
    fn as_value(&self) -> Value;
}

impl<T> IntoValue for T
where
    T: Into<Value> + Clone,
{
    fn as_value(&self) -> Value {
        self.clone().into()
    }
}

#[derive(Clone)]
pub struct YarnFnWrapper<Marker, F>
where
    F: YarnFnWithMarker<Marker>,
{
    function: F,

    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    _marker: PhantomData<fn() -> Marker>,
}

impl<Marker, F> From<F> for YarnFnWrapper<Marker, F>
where
    F: YarnFnWithMarker<Marker>,
{
    fn from(function: F) -> Self {
        Self {
            function,
            _marker: PhantomData,
        }
    }
}

impl<Marker, F> Debug for YarnFnWrapper<Marker, F>
where
    F: YarnFnWithMarker<Marker>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let params = std::any::type_name::<Marker>();
        let function_path = std::any::type_name::<F>();
        let debug_message = format!("YarnFn{params} -> Box<dyn IntoValue> {{{function_path}}}");
        f.debug_struct(&debug_message).finish()
    }
}

impl PartialEq for Box<dyn YarnFn> {
    fn eq(&self, other: &Self) -> bool {
        // Not guaranteed to be unique, but that's good enough for our purposes.
        let debug = format!("{:?}", self);
        let other_debug = format!("{:?}", other);
        debug == other_debug
    }
}

impl Eq for Box<dyn YarnFn> {}

/// Adapted from <https://github.com/bevyengine/bevy/blob/fe852fd0adbce6856f5886d66d20d62cfc936287/crates/bevy_ecs/src/system/system_param.rs#L1370>
macro_rules! impl_yarn_fn_tuple {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<F, I, $($param,)*> YarnFnWithMarker<($($param,)*)> for F
            where
                F: Fn($($param,)*) -> I,
                I: Into<Value> + 'static,
                $($param: TryFrom<Value> + 'static,)*
            {
                type Out = I;
                #[allow(non_snake_case)]
                fn call(&self, input: Vec<Value>) -> Self::Out {
                    if let [$($param,)*] = &input[..] {
                        let input = (
                            $($param
                            .clone()
                            .try_into()
                            .unwrap_or_else(|_| panic!("Failed to convert")),)*
                        );
                        let ($($param,)*) = input;
                        self($($param,)*)
                    } else {
                        panic!("Wrong number of arguments")
                    }
                }
            }
    };
}

all_tuples!(impl_yarn_fn_tuple, 0, 16, P);