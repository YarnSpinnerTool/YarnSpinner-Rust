use crate::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// A more type safe version of what in the original implementation was an `IDictionary<string, Delegate>`.
/// Necessary because of Rust's type system, as every function signature comes with a distinct type,
/// so we cannot simply hold a collection of different functions without all this effort.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct YarnFnRegistry(pub HashMap<Cow<'static, str>, Box<dyn YarnFn>>);

impl YarnFnRegistry {
    pub fn add<Marker, F>(&mut self, name: impl Into<Cow<'static, str>>, function: F)
    where
        Marker: 'static + Clone,
        F: YarnFnWithMarker<Marker> + 'static + Clone,
        F::Out: Into<Value> + 'static + Clone,
    {
        let name = name.into();
        let wrapped = YarnFnWrapper::from(function);
        self.insert(name, Box::new(wrapped));
    }

    pub fn add_boxed(&mut self, name: impl Into<Cow<'static, str>>, function: Box<dyn YarnFn>) {
        let name = name.into();
        self.insert(name, function);
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub fn get(&self, name: &str) -> Option<&dyn YarnFn> {
        self.0.get(name).map(|f| f.as_ref())
    }
}

impl Deref for YarnFnRegistry {
    type Target = HashMap<Cow<'static, str>, Box<dyn YarnFn>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for YarnFnRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Create a [`YarnFnRegistry`] from a list of named functions.
macro_rules! yarn_fn_registry {
    ($($name:expr => $function:expr,)*) => {
        {
            let mut map = YarnFnRegistry::default();
            $(
                map.add($name, $function);
            )*
            map
        }
    };
}
pub(crate) use yarn_fn_registry;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_fn_with_no_args() {
        let mut functions = YarnFnRegistry::default();
        functions.add("test", || true);
    }

    #[test]
    fn can_add_fn_with_one_arg() {
        let mut functions = YarnFnRegistry::default();
        functions.add("test", |a: f32| a);
    }

    #[test]
    fn can_call_fn_with_no_args() {
        let mut functions = YarnFnRegistry::default();

        functions.add("test", || true);
        let function = functions.get("test").unwrap();
        let result: bool = function.call(vec![]).try_into().unwrap();

        assert!(result);
    }

    #[test]
    fn can_call_fn_with_one_arg() {
        let mut functions = YarnFnRegistry::default();

        functions.add("test", |a: f32| a);
        let function = functions.get("test").unwrap();
        let result: f32 = function.call(vec![1.0.into()]).try_into().unwrap();

        assert_eq!(result, 1.0);
    }

    #[test]
    fn can_add_multiple_fns() {
        let mut functions = YarnFnRegistry::default();

        functions.add("test1", || true);
        functions.add("test2", |a: f32| a);
    }

    #[test]
    fn can_call_multiple_fns() {
        let mut functions = YarnFnRegistry::default();
        functions.add("test1", || true);
        functions.add("test2", |a: f32| a);

        let function1 = functions.get("test1").unwrap();
        let function2 = functions.get("test2").unwrap();

        let result1: bool = function1.call(vec![]).try_into().unwrap();
        let result2: f32 = function2.call(vec![1.0.into()]).try_into().unwrap();

        assert!(result1);
        assert_eq!(result2, 1.0);
    }

    #[test]
    fn can_call_multiple_fns_with_many_params() {
        let mut functions = YarnFnRegistry::default();

        functions.add("test1", || true);
        functions.add("test2", |a: f32, b: f32| a + b);
        functions.add("test3", |a: f32, b: f32, c: f32| a + b * c);
        functions.add(
            "test4",
            |a: String, b: String, c: String, d: bool, e: f32| format!("{}{}{}{}{}", a, b, c, d, e),
        );

        let function1 = functions.get("test1").unwrap();
        let function2 = functions.get("test2").unwrap();
        let function3 = functions.get("test3").unwrap();
        let function4 = functions.get("test4").unwrap();

        let result1: bool = function1.call(vec![]).try_into().unwrap();
        let result2: f32 = function2
            .call(vec![1.0.into(), 2.0.into()])
            .try_into()
            .unwrap();
        let result3: f32 = function3
            .call(vec![1.0.into(), 2.0.into(), 3.0.into()])
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
            .try_into()
            .unwrap();

        assert!(result1);
        assert_eq!(result2, 3.0);
        assert_eq!(result3, 7.0);
        assert_eq!(result4, "abctrue1".to_string());
    }

    #[test]
    fn debug_prints_signature() {
        let mut functions = YarnFnRegistry::default();

        functions.add("test", |a: f32, b: f32| a + b);
        let debug_string = format!("{:?}", functions);

        let element_start = debug_string.find('{').unwrap();
        // This looks like an off-by-one error on closer inspection,
        // but on even closer inspection it's correct because there's a space before the second '{' that we don't want to include.
        let element_end = element_start + debug_string[element_start + 1..].find('{').unwrap();
        let element = &debug_string[element_start..element_end];

        // Not testing the part after because its stability is not guaranteed.
        assert_eq!(element, "{\"test\": fn(f32, f32) -> f32");
    }
}
