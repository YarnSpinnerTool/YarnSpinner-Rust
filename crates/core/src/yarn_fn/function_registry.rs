use crate::prelude::*;
use alloc::borrow::Cow;
#[cfg(feature = "bevy")]
use bevy::prelude::*;
use std::collections::HashMap;

/// A registry of functions that can be called from Yarn after they have been added via [`YarnFnRegistry::register_function`].
///
/// # Implementation Notes
///
/// A more type safe version of what in the original implementation was an `IDictionary<string, Delegate>`.
/// Necessary because of Rust's type system, as every function signature comes with a distinct type,
/// so we cannot simply hold a collection of different functions without all this effort.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct YarnFnRegistry(pub(crate) InnerRegistry);

type InnerRegistry = HashMap<Cow<'static, str>, Box<dyn UntypedYarnFn>>;

impl Extend<<InnerRegistry as IntoIterator>::Item> for YarnFnRegistry {
    fn extend<T: IntoIterator<Item = <InnerRegistry as IntoIterator>::Item>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl IntoIterator for YarnFnRegistry {
    type Item = <InnerRegistry as IntoIterator>::Item;
    type IntoIter = <InnerRegistry as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl YarnFnRegistry {
    /// Adds a new function to the registry. See [`YarnFn`]'s documentation for what kinds of functions are allowed.
    pub(crate) fn register_function<Marker, F>(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        function: F,
    ) -> &mut Self
    where
        Marker: 'static,
        F: YarnFn<Marker> + 'static + Clone,
        F::Out: IntoYarnValueFromNonYarnValue + 'static + Clone,
    {
        let name = name.into();
        let wrapped = YarnFnWrapper::from(function);
        self.0.insert(name, Box::new(wrapped));
        self
    }

    /// Iterates over all functions in the registry.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, &(dyn UntypedYarnFn))> {
        self.0
            .iter()
            .map(|(key, value)| (key.as_ref(), value.as_ref()))
    }

    pub(crate) fn add_boxed(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        function: Box<dyn UntypedYarnFn>,
    ) -> &mut Self {
        let name = name.into();
        self.0.insert(name, function);
        self
    }

    /// Returns `true` if the registry contains a function with the given name.
    pub(crate) fn contains_function(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    pub(crate) fn get(&self, name: &str) -> Option<&(dyn UntypedYarnFn)> {
        self.0.get(name).map(|f| f.as_ref())
    }

    pub(crate) fn names(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(|key| key.as_ref())
    }

    pub(crate) fn functions(&self) -> impl Iterator<Item = &(dyn UntypedYarnFn)> {
        self.0.values().map(|value| value.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_fn_with_no_args() {
        let mut functions = YarnFnRegistry::default();
        functions.register_function("test", || true);
    }

    #[test]
    fn can_add_fn_with_one_arg() {
        let mut functions = YarnFnRegistry::default();
        functions.register_function("test", |a: f32| a);
    }

    #[test]
    fn can_call_fn_with_no_args() {
        let mut functions = YarnFnRegistry::default();

        functions.register_function("test", || true);
        let function = functions.get("test").unwrap();
        let result: bool = function
            .call(
                vec![],
                #[cfg(feature = "bevy")]
                &mut World::default(),
            )
            .try_into()
            .unwrap();

        assert!(result);
    }

    #[test]
    fn can_call_fn_with_one_arg() {
        let mut functions = YarnFnRegistry::default();

        functions.register_function("test", |a: f32| a);
        let function = functions.get("test").unwrap();
        let result: f32 = function
            .call(
                to_function_params([1.0]),
                #[cfg(feature = "bevy")]
                &mut World::default(),
            )
            .try_into()
            .unwrap();

        assert_eq!(result, 1.0);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn can_access_bevy_world() {
        let mut functions = YarnFnRegistry::default();
        let mut world = World::default();

        let entity = world.spawn(Name::new("test_entity")).id();

        functions.register_function(
            "test1",
            world.register_system(move |query: Query<(Entity, &Name)>| {
                let mut did_find = false;
                for (found_entity, found_name) in &query {
                    assert_eq!(found_entity, entity);
                    assert_eq!(found_name.as_str(), "test_entity");
                    did_find = true;
                }
                did_find
            }),
        );

        let function1 = functions.get("test1").unwrap();
        let result1: bool = function1.call(vec![], &mut world).try_into().unwrap();
        assert!(result1);
    }

    #[test]
    fn can_add_multiple_fns() {
        let mut functions = YarnFnRegistry::default();

        functions.register_function("test1", || true);
        functions.register_function("test2", |a: f32| a);
    }

    #[test]
    fn can_call_multiple_fns() {
        let mut functions = YarnFnRegistry::default();
        functions.register_function("test1", || true);
        functions.register_function("test2", |a: f32| a);

        let function1 = functions.get("test1").unwrap();
        let function2 = functions.get("test2").unwrap();

        #[cfg(feature = "bevy")]
        let mut world = World::default();

        let result1: bool = function1
            .call(
                vec![],
                #[cfg(feature = "bevy")]
                &mut world,
            )
            .try_into()
            .unwrap();
        let result2: f32 = function2
            .call(
                to_function_params([1.0]),
                #[cfg(feature = "bevy")]
                &mut world,
            )
            .try_into()
            .unwrap();

        assert!(result1);
        assert_eq!(result2, 1.0);
    }

    #[test]
    fn can_call_multiple_fns_with_many_params() {
        let mut functions = YarnFnRegistry::default();

        functions.register_function("test1", || true);
        functions.register_function("test2", |a: f32, b: f32| a + b);
        functions.register_function("test3", |a: f32, b: f32, c: f32| a + b * c);
        functions.register_function(
            "test4",
            |a: String, b: String, c: String, d: bool, e: f32| format!("{}{}{}{}{}", a, b, c, d, e),
        );

        let function1 = functions.get("test1").unwrap();
        let function2 = functions.get("test2").unwrap();
        let function3 = functions.get("test3").unwrap();
        let function4 = functions.get("test4").unwrap();

        #[cfg(feature = "bevy")]
        let mut world = World::default();

        let result1: bool = function1
            .call(
                vec![],
                #[cfg(feature = "bevy")]
                &mut world,
            )
            .try_into()
            .unwrap();
        let result2: f32 = function2
            .call(
                to_function_params([1.0, 2.0]),
                #[cfg(feature = "bevy")]
                &mut world,
            )
            .try_into()
            .unwrap();
        let result3: f32 = function3
            .call(
                to_function_params([1.0, 2.0, 3.0]),
                #[cfg(feature = "bevy")]
                &mut world,
            )
            .try_into()
            .unwrap();
        let result4: String = function4
            .call(
                to_function_params([
                    YarnValue::from("a"),
                    "b".into(),
                    "c".into(),
                    true.into(),
                    1.0.into(),
                ]),
                #[cfg(feature = "bevy")]
                &mut world,
            )
            .into();

        assert!(result1);
        assert_eq!(result2, 3.0);
        assert_eq!(result3, 7.0);
        assert_eq!(result4, "abctrue1".to_string());
    }

    fn to_function_params(
        params: impl IntoIterator<Item = impl Into<YarnValue>>,
    ) -> Vec<YarnValue> {
        params.into_iter().map(Into::into).collect()
    }

    #[test]
    fn debug_prints_signature() {
        let mut functions = YarnFnRegistry::default();

        functions.register_function("test", |a: f32, b: f32| a + b);
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
