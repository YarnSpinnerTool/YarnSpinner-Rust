//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Library.cs>

mod yarn_fn;

pub use yarn_fn::*;

use crate::prelude::Value;
use std::collections::HashMap;

/// A collection of functions that can be called from Yarn scripts.
///
/// You do not create instances of this class yourself. The [`Dialogue`]
/// class creates one for you, and you can access it through the
/// [`Library`] property.
#[derive(Debug, Clone, Default)]
pub struct Library {
    /// The functions that are available to Yarn scripts.
    functions: HashMap<String, Box<dyn YarnFn>>,
}

impl Library {
    fn add<Marker, F>(&mut self, name: &str, function: F)
    where
        Marker: 'static + Clone,
        F: YarnFnWithMarker<Marker> + 'static + Clone,
        F::Out: Into<Value> + 'static + Clone,
    {
        let wrapped = YarnFnWrapper::new(function);
        self.functions.insert(name.to_string(), Box::new(wrapped));
    }

    fn get(&self, name: &str) -> Option<&dyn YarnFn> {
        self.functions.get(name).map(|f| f.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_fn_with_no_args() {
        let mut library = Library::default();
        library.add("test", || true);
    }

    #[test]
    fn can_add_fn_with_one_arg() {
        let mut library = Library::default();
        library.add("test", |a: f32| a);
    }

    #[test]
    fn can_call_fn_with_no_args() {
        let mut library = Library::default();
        library.add("test", || true);
        let function = library.get("test").unwrap();
        let result = function.call(vec![]);
        assert_eq!(result.as_value(), Value::Bool(true));
    }

    #[test]
    fn can_call_fn_with_one_arg() {
        let mut library = Library::default();
        library.add("test", |a: f32| a);
        let function = library.get("test").unwrap();
        let result = function.call(vec![Value::Number(1.0)]);
        assert_eq!(result.as_value(), Value::Number(1.0));
    }

    #[test]
    fn can_add_multiple_fns() {
        let mut library = Library::default();
        library.add("test1", || true);
        library.add("test2", |a: f32| a);
    }

    #[test]
    fn can_call_multiple_fns() {
        let mut library = Library::default();
        library.add("test1", || true);
        library.add("test2", |a: f32| a);
        let function1 = library.get("test1").unwrap();
        let function2 = library.get("test2").unwrap();
        let result1 = function1.call(vec![]);
        let result2 = function2.call(vec![Value::Number(1.0)]);
        assert_eq!(result1.as_value(), Value::Bool(true));
        assert_eq!(result2.as_value(), Value::Number(1.0));
    }
}
