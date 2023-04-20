use crate::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// A more type safe version of what in the original implementation was an `IDictionary<string, Delegate>`.
/// Necessary because of Rust's type system, as every function signature comes with a distinct type,
/// so we cannot simply hold a collection of different functions without all this effort.
pub struct YarnFnHashMap(pub HashMap<String, Box<dyn YarnFn>>);

impl YarnFnHashMap {
    pub fn add<Marker, F>(&mut self, name: impl Into<Cow<'static, str>>, function: F)
    where
        Marker: 'static + Clone,
        F: YarnFnWithMarker<Marker> + 'static + Clone,
        F::Out: Into<Value> + 'static + Clone,
    {
        let name = name.into().to_string();
        let wrapped = YarnFnWrapper::from(function);
        self.insert(name, Box::new(wrapped));
    }

    pub fn get(&self, name: &str) -> Option<&dyn YarnFn> {
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

/// Create a [`YarnFnHashMap`] from a list of named functions.
#[macro_export]
macro_rules! yarn_fn_hash_map {
    ($($name:expr => $function:expr,)*) => {
        {
            let mut map = YarnFnHashMap::default();
            $(
                map.add($name, $function);
            )*
            map
        }
    };
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
        let result: bool = function.call(vec![]).as_value().try_into().unwrap();

        assert_eq!(result, true);
    }

    #[test]
    fn can_call_fn_with_one_arg() {
        let mut library = YarnFnHashMap::default();

        library.add("test", |a: f32| a);
        let function = library.get("test").unwrap();
        let result: f32 = function
            .call(vec![1.0.into()])
            .as_value()
            .try_into()
            .unwrap();

        assert_eq!(result, 1.0);
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

        let result1: bool = function1.call(vec![]).as_value().try_into().unwrap();
        let result2: f32 = function2
            .call(vec![1.0.into()])
            .as_value()
            .try_into()
            .unwrap();

        assert_eq!(result1, true);
        assert_eq!(result2, 1.0);
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

        let result1: bool = function1.call(vec![]).as_value().try_into().unwrap();
        let result2: f32 = function2
            .call(vec![1.0.into(), 2.0.into()])
            .as_value()
            .try_into()
            .unwrap();
        let result3: f32 = function3
            .call(vec![1.0.into(), 2.0.into(), 3.0.into()])
            .as_value()
            .try_into()
            .unwrap();
        let result4: String = function4
            .call(vec![
                "a".into(),
                "b".into(),
                "c".into(),
                true.into(),
                1.0.into(),
            ])
            .as_value()
            .try_into()
            .unwrap();

        assert_eq!(result1, true);
        assert_eq!(result2, 3.0);
        assert_eq!(result3, 7.0);
        assert_eq!(result4, "abctrue1".to_string());
    }
}
