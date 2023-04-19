use rusty_yarn_spinner_core::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::marker::PhantomData;

fn main() {
    let mut container = FunctionContainer::default();
    container.add("foo", || true);
    container.add("bar", |a: f32| a);

    let first_fn = container.get("foo").unwrap();
    let first_result = first_fn.call(vec![]);
    assert_eq!(first_result.as_value(), true.into());

    let second_fn = container.get("bar").unwrap();
    let second_result = second_fn.call(vec![1.0.into()]);
    assert_eq!(second_result.as_value(), 1.0.into());
    println!("Great Success!");
}

trait ContainedFunctionWithMarker<Marker> {
    type Out: Into<Value>;
    fn call(&self, input: Vec<Value>) -> Self::Out;
}

trait BoxedContainedFunction {
    fn call(&self, input: Vec<Value>) -> Box<dyn IntoValue>;
}

impl<Marker, F> BoxedContainedFunction for FunctionWrapper<Marker, F>
where
    Marker: 'static,
    F: ContainedFunctionWithMarker<Marker> + 'static,
    F::Out: Into<Value> + 'static + Clone,
{
    fn call(&self, input: Vec<Value>) -> Box<dyn IntoValue> {
        let output = self.function.call(input);
        Box::new(output)
    }
}

trait IntoValue {
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

struct FunctionWrapper<Marker, F>
where
    F: ContainedFunctionWithMarker<Marker>,
{
    function: F,

    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    _marker: PhantomData<fn() -> Marker>,
}

impl<F, I> ContainedFunctionWithMarker<()> for F
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

impl<F, I, T0> ContainedFunctionWithMarker<(T0,)> for F
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

#[derive(Default)]
struct FunctionContainer {
    functions: HashMap<String, Box<dyn BoxedContainedFunction>>,
}

impl FunctionContainer {
    fn add<Marker, F>(&mut self, name: &str, function: F)
    where
        Marker: 'static,
        F: ContainedFunctionWithMarker<Marker> + 'static,
        F::Out: Into<Value> + 'static + Clone,
    {
        let wrapped = FunctionWrapper {
            function,
            _marker: PhantomData,
        };
        self.functions.insert(name.to_string(), Box::new(wrapped));
    }

    fn get(&self, name: &str) -> Option<&dyn BoxedContainedFunction> {
        self.functions.get(name).map(|f| f.as_ref())
    }
}
