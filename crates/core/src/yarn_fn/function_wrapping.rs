use crate::prelude::*;
use std::any::TypeId;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use yarn_slinger_macros::all_tuples;

/// A function that can be registered into and called from Yarn.
/// It must have the following properties:
/// - It is allowed to have zero or more parameters
/// - Each parameter must be one of the following types or a reference to them:
///   - [`bool`]
///   - A numeric type or its reference, i.e. one of [`f32`], [`f64`], [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`], [`isize`],
///   - [`String`] (for a reference, [`&str`] may be used instead of `&String`)
///   - [`YarnValue`], which means that a parameter may be any of the above types
/// - It must return a value.
/// - Its return type must be one of the types listed above, but neither a reference nor a [`YarnValue`].
/// ## Examples
/// ```rust
/// fn give_summary(name: &str, age: usize, is_cool: bool) -> String {
///    format!("{name} is {age} years old and is {} cool", if is_cool { "very" } else { "not" })
/// }
/// ```
/// Which may be called from Yarn as follows:
/// ```yarn
/// <<set $name to "Bob">>
/// <<set $age to 42>>
/// <<set $is_cool to true>>
/// Narrator: {give_summary($name, $age, $is_cool)}
/// ```
pub trait YarnFn<Marker>: Clone + Send + Sync {
    type Out: IntoYarnValueFromNonYarnValue + 'static;
    /// The `Option`s are guaranteed to be `Some` and are just typed like this to be able to `std::mem::take` them.
    fn call(&self, input: Vec<YarnValue>) -> Self::Out;
    fn parameter_types(&self) -> Vec<TypeId>;
    fn return_type(&self) -> TypeId {
        TypeId::of::<Self::Out>()
    }
}

/// A [`YarnFn`] with the `Marker` type parameter erased.
/// See its documentation for more information about what kind of functions are allowed.
pub trait UntypedYarnFn: Debug + Display + Send + Sync {
    fn call(&self, input: Vec<YarnValue>) -> YarnValue;
    fn clone_box(&self) -> Box<dyn UntypedYarnFn>;
    fn parameter_types(&self) -> Vec<TypeId>;
    fn return_type(&self) -> TypeId;
}

impl Clone for Box<dyn UntypedYarnFn> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<Marker, F> UntypedYarnFn for YarnFnWrapper<Marker, F>
where
    Marker: 'static,
    F: YarnFn<Marker> + 'static + Clone,
    F::Out: IntoYarnValueFromNonYarnValue + 'static + Clone,
{
    fn call(&self, input: Vec<YarnValue>) -> YarnValue {
        let output = self.function.call(input);
        output.into_untyped_value()
    }

    fn clone_box(&self) -> Box<dyn UntypedYarnFn> {
        Box::new(self.clone())
    }

    fn parameter_types(&self) -> Vec<TypeId> {
        self.function.parameter_types()
    }

    fn return_type(&self) -> TypeId {
        self.function.return_type()
    }
}

pub(crate) struct YarnFnWrapper<Marker, F>
where
    F: YarnFn<Marker>,
{
    function: F,

    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    _marker: PhantomData<fn() -> Marker>,
}

impl<Marker, F> Clone for YarnFnWrapper<Marker, F>
where
    F: YarnFn<Marker>,
{
    fn clone(&self) -> Self {
        Self {
            function: self.function.clone(),
            _marker: self._marker,
        }
    }
}

impl<Marker, F> From<F> for YarnFnWrapper<Marker, F>
where
    F: YarnFn<Marker>,
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
    F: YarnFn<Marker>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = std::any::type_name::<Marker>();
        let function_path = std::any::type_name::<F>();
        let debug_message = format!("{signature} {{{function_path}}}");
        f.debug_struct(&debug_message).finish()
    }
}

impl<Marker, F> Display for YarnFnWrapper<Marker, F>
where
    F: YarnFn<Marker>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let signature = std::any::type_name::<Marker>();
        f.write_str(signature)
    }
}

impl PartialEq for Box<dyn UntypedYarnFn> {
    fn eq(&self, other: &Self) -> bool {
        // Not guaranteed to be unique, but it's good enough for our purposes.
        let debug = format!("{:?}", self);
        let other_debug = format!("{:?}", other);
        debug == other_debug
    }
}

impl Eq for Box<dyn UntypedYarnFn> {}

/// A macro for using [`YarnFn`] as a return type or parameter type without needing
/// to know the implementation details of the [`YarnFn`] trait.
///
/// This is useful when registering functions in a [`Library`] with [`Library::register_function`].
#[macro_export]
macro_rules! yarn_fn_type {
    (impl Fn($($param:ty),+) -> $ret:ty) => {
        impl $crate::prelude::YarnFn<fn($($param),+) -> $ret, Out = $ret>
    };
}
pub use yarn_fn_type;

/// Adapted from <https://github.com/bevyengine/bevy/blob/fe852fd0adbce6856f5886d66d20d62cfc936287/crates/bevy_ecs/src/system/system_param.rs#L1370>
macro_rules! impl_yarn_fn_tuple {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<F, O, $($param,)*> YarnFn<fn($($param,)*) -> O> for F
            where
            for<'a> F:
                Send + Sync + Clone +
                Fn($($param,)*) -> O +
                Fn($(<$param as YarnFnParam>::Item<'a>,)*) -> O,
            O: IntoYarnValueFromNonYarnValue + 'static,
            $($param: YarnFnParam + 'static,)*
            {
                type Out = O;
                #[allow(non_snake_case)]
                fn call(&self, input: Vec<YarnValue>) -> Self::Out {
                    let mut params: Vec<_> = input.into_iter().map(YarnValueWrapper::from).collect();

                    #[allow(unused_variables, unused_mut)] // for n = 0 tuples
                    let mut iter = params.iter_mut();

                    // $param is the type implementing YarnFnParam
                    let input = (
                        $($param::retrieve(&mut iter),)*
                    );
                    assert!(iter.next().is_none(), "Passed too many arguments to YarnFn");

                    let ($($param,)*) = input;
                    self($($param,)*)
                }

                fn parameter_types(&self) -> Vec<TypeId> {
                    vec![$(TypeId::of::<$param>()),*]
                }
            }
    };
}

all_tuples!(impl_yarn_fn_tuple, 0, 16, P);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_no_params() {
        fn f() -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_string() {
        fn f(_: String) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_string_ref() {
        fn f(_: &String) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_string_slice() {
        fn f(_: &str) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_usize() {
        fn f(_: usize) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_usize_ref() {
        fn f(_: &usize) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_yarn_value() {
        fn f(_: YarnValue) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_yarn_value_ref() {
        fn f(_: &YarnValue) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_multiple_strings() {
        fn f(s: String, _: String, _: &str, _: String, _: &str) -> String {
            s
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_lots_of_different_types() {
        #[allow(clippy::too_many_arguments)]
        fn f(
            _: String,
            _: usize,
            _: &str,
            _: &YarnValue,
            _: &bool,
            _: isize,
            _: String,
            _: &u32,
        ) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_tuples() {
        #[allow(clippy::too_many_arguments)]
        fn f(
            _: (String, usize),
            _: usize,
            _: (&str, (&str, &String)),
            _: &YarnValue,
            _: (&bool, bool, bool, (&str, String)),
            _: isize,
            _: String,
            _: &u32,
        ) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn unpacks_tuples_in_right_order() {
        #[allow(clippy::too_many_arguments)]
        fn f(a: usize, (b, c): (usize, usize), d: usize, (e, f, g): (usize, usize, usize)) -> bool {
            a == 1 && b == 2 && c == 3 && d == 4 && e == 5 && f == 6 && g == 7
        }
        let input: Vec<_> = (1..=7).map(YarnValue::from).collect();
        let result = apply_yarn_fn(f, input);
        assert!(result);
    }

    #[test]
    fn accepts_function_with_single_tuple_param() {
        fn f(_: (usize, isize, (String, &str))) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    fn accept_yarn_fn<Marker>(_: impl YarnFn<Marker>) {}

    fn apply_yarn_fn<T, Marker>(f: T, input: Vec<YarnValue>) -> T::Out
    where
        T: YarnFn<Marker>,
    {
        f.call(input)
    }
}
