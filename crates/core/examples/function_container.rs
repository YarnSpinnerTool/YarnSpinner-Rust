use rusty_yarn_spinner_core::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut container = FunctionContainer::default();
    container.add("foo", || true);
}

trait ContainedFunction {
    type Out: Into<Value>;
    fn call(&self, input: &[Value]) -> Self::Out;
}

trait BoxedContainedFunction {
    fn call(&self, input: &[Value]) -> Box<dyn IntoValue>;
}

impl<F> BoxedContainedFunction for F
where
    F: BoxedContainedFunctionWithMarker<()>,
{
    fn call(&self, input: &[Value]) -> Box<dyn IntoValue> {
        self.call(input)
    }
}

trait IntoValue {
    fn into_value(self) -> Value;
}

impl<T> IntoValue for T
where
    T: Into<Value>,
{
    fn into_value(self) -> Value {
        self.into()
    }
}

trait BoxedContainedFunctionWithMarker<Marker> {
    fn call(&self, input: &[Value]) -> Box<dyn IntoValue>;
}

impl<F, I, Marker> BoxedContainedFunctionWithMarker<Marker> for F
where
    F: Fn() -> I,
    I: Into<Value> + 'static,
{
    fn call(&self, input: &[Value]) -> Box<dyn IntoValue + 'static> {
        if let [] = &input[..] {
            let output = self();
            Box::new(output)
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
    fn add<F, I>(&mut self, name: &str, function: F)
    where
        F: Fn() -> I + BoxedContainedFunction + 'static,
        I: Into<Value>,
    {
        self.functions.insert(name.to_string(), Box::new(function));
    }
}
