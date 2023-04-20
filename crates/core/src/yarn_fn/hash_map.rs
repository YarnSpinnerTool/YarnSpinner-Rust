use crate::prelude::{yarn_fn::*, Value};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct YarnFnHashMap(pub HashMap<String, Box<dyn YarnFn>>);

impl YarnFnHashMap {
    fn add<Marker, F>(&mut self, name: &str, function: F)
    where
        Marker: 'static + Clone,
        F: YarnFnWithMarker<Marker> + 'static + Clone,
        F::Out: Into<Value> + 'static + Clone,
    {
        let wrapped = YarnFnWrapper::from(function);
        self.insert(name.to_string(), Box::new(wrapped));
    }

    fn get(&self, name: &str) -> Option<&dyn YarnFn> {
        self.0.get(name).map(|f| f.as_ref())
    }
}

impl Deref for YarnFnHashMap {
    type Target = HashMap<String, Box<dyn YarnFn>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YarnFnHashMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_fn_with_no_args() {
        let mut library = YarnFnHashMap::default();
        library.add("test", || true);
    }

    #[test]
    fn can_add_fn_with_one_arg() {
        let mut library = YarnFnHashMap::default();
        library.add("test", |a: f32| a);
    }

    #[test]
    fn can_call_fn_with_no_args() {
        let mut library = YarnFnHashMap::default();

        library.add("test", || true);
        let function = library.get("test").unwrap();
        let result = function.call(vec![]);

        assert_eq!(result.as_value(), Value::Bool(true));
    }

    #[test]
    fn can_call_fn_with_one_arg() {
        let mut library = YarnFnHashMap::default();

        library.add("test", |a: f32| a);
        let function = library.get("test").unwrap();
        let result = function.call(vec![Value::Number(1.0)]);

        assert_eq!(result.as_value(), Value::Number(1.0));
    }

    #[test]
    fn can_add_multiple_fns() {
        let mut library = YarnFnHashMap::default();

        library.add("test1", || true);
        library.add("test2", |a: f32| a);
    }

    #[test]
    fn can_call_multiple_fns() {
        let mut library = YarnFnHashMap::default();
        library.add("test1", || true);
        library.add("test2", |a: f32| a);

        let function1 = library.get("test1").unwrap();
        let function2 = library.get("test2").unwrap();

        let result1 = function1.call(vec![]);
        let result2 = function2.call(vec![Value::Number(1.0)]);

        assert_eq!(result1.as_value(), Value::Bool(true));
        assert_eq!(result2.as_value(), Value::Number(1.0));
    }

    #[test]
    fn can_call_multiple_fns_with_many_params() {
        let mut library = YarnFnHashMap::default();

        library.add("test1", || true);
        library.add("test2", |a: f32, b: f32| a + b);
        library.add("test3", |a: f32, b: f32, c: f32| a + b * c);
        library.add(
            "test4",
            |a: String, b: String, c: String, d: bool, e: f32| format!("{}{}{}{}{}", a, b, c, d, e),
        );

        let function1 = library.get("test1").unwrap();
        let function2 = library.get("test2").unwrap();
        let function3 = library.get("test3").unwrap();
        let function4 = library.get("test4").unwrap();

        let result1 = function1.call(vec![]);
        let result2 = function2.call(vec![Value::Number(1.0), Value::Number(2.0)]);
        let result3 = function3.call(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);
        let result4 = function4.call(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
            Value::String("c".to_string()),
            Value::Bool(true),
            Value::Number(1.0),
        ]);

        assert_eq!(result1.as_value(), Value::Bool(true));
        assert_eq!(result2.as_value(), Value::Number(3.0));
        assert_eq!(result3.as_value(), Value::Number(7.0));
        assert_eq!(result4.as_value(), Value::String("abctrue1".to_string()));
    }
}
