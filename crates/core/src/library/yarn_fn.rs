//! Inspired by how Bevy stores [`FnSystem`](https://docs.rs/bevy_ecs/0.10.1/bevy_ecs/system/struct.FnSystem.html)s.

use crate::prelude::Value;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub trait YarnFnWithMarker<Marker> {
    type Out: Into<Value>;
    fn call(&self, input: Vec<Value>) -> Self::Out;
}

pub trait YarnFn {
    fn call(&self, input: Vec<Value>) -> Box<dyn IntoValue>;
    fn clone_box(&self) -> Box<dyn YarnFn>;
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

impl<Marker, F> YarnFnWrapper<Marker, F>
where
    F: YarnFnWithMarker<Marker>,
{
    pub fn new(function: F) -> Self {
        Self {
            function,
            _marker: PhantomData,
        }
    }
}

impl<F, I> YarnFnWithMarker<()> for F
where
    F: Fn() -> I,
    I: Into<Value> + 'static,
{
    type Out = I;
    #[allow(non_snake_case)]
    fn call(&self, input: Vec<Value>) -> Self::Out {
        if let [] = input[..] {
            let input = ();
            let () = input;
            self()
        } else {
            panic!("Wrong number of arguments")
        }
    }
}

impl<F, I, T0> YarnFnWithMarker<(T0,)> for F
where
    F: Fn(T0) -> I,
    I: Into<Value> + 'static,
    T0: TryFrom<Value> + 'static,
{
    type Out = I;
    #[allow(non_snake_case)]
    fn call(&self, input: Vec<Value>) -> Self::Out {
        if let [T0] = &input[..] {
            let input = (T0
                .clone()
                .try_into()
                .unwrap_or_else(|_| panic!("Failed to convert")),);
            let (T0,) = input;
            self(T0)
        } else {
            panic!("Wrong number of arguments")
        }
    }
}

impl Debug for dyn YarnFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YarnFn").finish()
    }
}

impl Clone for Box<dyn YarnFn> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
