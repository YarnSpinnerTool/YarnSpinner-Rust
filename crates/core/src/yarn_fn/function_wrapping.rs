use super::optionality::AllowedOptionalityChain;
use crate::prelude::*;
#[cfg(feature = "bevy")]
use bevy::prelude::World;
use std::any::TypeId;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use variadics_please::all_tuples;

/// A function that can be registered into and called from Yarn.
/// It must have the following properties:
/// - It is allowed to have zero or more parameters
/// - Each parameter must be a [`YarnFnParam`], which means of the following types or a reference to them:
///   - [`bool`]
///   - A numeric type, i.e. one of [`f32`], [`f64`], [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`], [`isize`]
///   - [`String`] (for a reference, [`&str`] may be used instead of `&String`)
///   - [`YarnValue`], which means that a parameter may be any of the above types
///   - Tuples of the above types.
/// - It must return a value.
/// - Its return type must be one of the following types:
///   - [`bool`]
///   - A numeric type, i.e. one of [`f32`], [`f64`], [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], [`usize`], [`isize`]
///   - [`String`]
///
/// If the `bevy` feature is active then it is also possible to register a Bevy `System` and call it from Yarn. The `System` will receive the parameters passed to the yarn
/// as it's input. The `System`'s input must adhere to the same rules as given above for regular function parameters with the exception that System functions cannot accept
/// tuples as parameters. The `System`'s output must adhere to the rules for a regular function's return type listed above.
/// The `System` may take any `SystemParam`.
///
/// Note that in particular, no references can be returned.
/// ## Examples
/// ```rust
/// fn give_summary(name: &str, age: usize, is_cool: bool) -> String {
///    format!("{name} is {age} years old and is {} cool", if is_cool { "very" } else { "not" })
/// }
/// ```
/// Which may be called from Yarn as follows:
/// ```text
/// <<set $name to "Bob">>
/// <<set $age to 42>>
/// <<set $is_cool to true>>
/// Narrator: {give_summary($name, $age, $is_cool)}
/// ```
///
/// A bevy system:
/// ```rust
/// # use bevy::prelude::*;
/// #[derive(Component)]
/// struct Age(u32);
/// fn give_summary_from_bevy(In(name): In<&str>, ages: Query<(&Name, &Age)>) -> String {
///     for (found_name, age) in &ages {
///         if name == found_name.as_str() {
///             return format!("{name} is {} years old", age.0)
///         }
///     }
///    format!("{name} is ageless")
/// }
/// # // assert the example is actually a valid system
/// # World::default().register_system(give_summary_from_bevy);
/// ```
/// Which may be called from Yarn as follows:
/// ```text
/// <<set $name to "Bob">>
/// Narrator: {give_summary_from_bevy($name)}
/// ```
pub trait YarnFn<Marker>: Clone + Send + Sync {
    /// The type of the value returned by this function. See [`YarnFn`] for more information about what is allowed.
    type Out: IntoYarnValueFromNonYarnValue + 'static;
    #[cfg(feature = "bevy")]
    #[doc(hidden)]
    fn call(&self, input: Vec<YarnValue>, world: &mut World) -> Self::Out;
    #[cfg(not(feature = "bevy"))]
    #[doc(hidden)]
    fn call(&self, input: Vec<YarnValue>) -> Self::Out;
    /// The [`TypeId`]s of the parameters of this function.
    fn parameter_types(&self) -> Vec<TypeId>;
    /// The [`TypeId`] of the return type of this function.
    fn return_type(&self) -> TypeId {
        TypeId::of::<Self::Out>()
    }
}

/// A [`YarnFn`] with the `Marker` type parameter erased.
/// See its documentation for more information about what kind of functions are allowed.
pub trait UntypedYarnFn: Debug + Display + Send + Sync {
    #[cfg(feature = "bevy")]
    #[doc(hidden)]
    fn call(&self, input: Vec<YarnValue>, world: &mut World) -> YarnValue;
    #[cfg(not(feature = "bevy"))]
    #[doc(hidden)]
    fn call(&self, input: Vec<YarnValue>) -> YarnValue;
    #[doc(hidden)]
    fn clone_box(&self) -> Box<dyn UntypedYarnFn>;
    /// The [`TypeId`]s of the parameters of this function.
    fn parameter_types(&self) -> Vec<TypeId>;
    /// The [`TypeId`] of the return type of this function.
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
    fn call(&self, input: Vec<YarnValue>, #[cfg(feature = "bevy")] world: &mut World) -> YarnValue {
        #[cfg(feature = "bevy")]
        let output = self.function.call(input, world);
        #[cfg(not(feature = "bevy"))]
        let output = self.function.call(input);
        output.into_yarn_value()
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
            _marker: PhantomData,
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
/// This is useful when registering functions in a [`Library`] with [`Library::add_function`].
#[macro_export]
macro_rules! yarn_fn_type {
    (impl Fn($($param:ty),+) -> $ret:ty) => {
        impl $crate::prelude::YarnFn<fn($($param),+) -> $ret, Out = $ret>
    };
}
pub use yarn_fn_type;

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! count_tts {
    ($($tts:tt)*) => {<[()]>::len(&[$(replace_expr!($tts ())),*])};
}

/// Adapted from <https://github.com/bevyengine/bevy/blob/fe852fd0adbce6856f5886d66d20d62cfc936287/crates/bevy_ecs/src/system/system_param.rs#L1370>
#[cfg(feature = "bevy")]
mod bevy_functions {
    use super::*;
    use bevy::ecs::system::SystemId;
    use bevy::prelude::*;
    use std::collections::VecDeque;

    macro_rules! impl_yarn_fn_tuple_bevy {
        ($($yarn_param: ident),*) => {
            #[allow(non_snake_case, unused_parens)]
            impl<'a, Output, $($yarn_param),*> YarnFn<(($($yarn_param,)*), Output)> for SystemId<In<($($yarn_param),*)>, Output>
            where
                $($yarn_param: TryFrom<YarnValue> + 'static,)*
                Output: IntoYarnValueFromNonYarnValue + 'static,
                {
                    type Out = Output;
                    #[allow(non_snake_case)]
                    fn call(&self, input: Vec<YarnValue>, world: &mut World) -> Self::Out {
                        #[allow(unused)]
                        let mut input = VecDeque::from(input);
                        #[allow(unused)]
                        let input_len  = input.len();
                        let expected_len = count_tts!($($yarn_param) *);

                        assert!(input_len == expected_len, "YarnFn expected {expected_len} arguments but received {input_len}");
                        $(
                            let $yarn_param:$yarn_param = $yarn_param::try_from(input.pop_front().unwrap()).ok().expect("Invalid argument type");
                        )*
                        world.run_system_with(*self, ($($yarn_param),*)).unwrap()
                    }

                    fn parameter_types(&self) -> Vec<TypeId> {
                        vec![$(TypeId::of::<$yarn_param>()),*]
                    }
                }
            };
    }
    all_tuples!(impl_yarn_fn_tuple_bevy, 0, 16, P);

    impl<Output> YarnFn<Output> for SystemId<(), Output>
    where
        Output: IntoYarnValueFromNonYarnValue + 'static,
    {
        type Out = Output;
        #[allow(non_snake_case)]
        fn call(&self, _input: Vec<YarnValue>, world: &mut World) -> Self::Out {
            world.run_system(*self).unwrap()
        }

        fn parameter_types(&self) -> Vec<TypeId> {
            vec![]
        }
    }
}

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
            ($(<$param as YarnFnParam>::Optionality,)*): AllowedOptionalityChain,
            {
                type Out = O;
                #[allow(non_snake_case)]
                fn call(
                    &self, input: Vec<YarnValue>,
                    #[cfg(feature = "bevy")]
                    _world: &mut World
                ) -> Self::Out {
                    let input_len = input.len();
                    let mut params: Vec<_> = input.into_iter().map(YarnValueWrapper::from).collect();

                    #[allow(unused_variables, unused_mut)] // for n = 0 tuples
                    let mut iter = params.iter_mut().peekable();

                    // $param is the type implementing YarnFnParam
                    let input = (
                        $($param::retrieve(&mut iter),)*
                    );
                    assert!(iter.next().is_none(), "YarnFn expected {} arguments but received {}", count_tts!($($param),*), input_len);

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
    #[cfg(feature = "bevy")]
    use bevy::prelude::*;

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
    fn accepts_optional_value() {
        fn f(_: Option<String>) -> bool {
            true
        }
        accept_yarn_fn(f);
    }

    #[test]
    fn accepts_optional_value_ref() {
        fn f(_: Option<&YarnValue>) -> bool {
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

    #[cfg(feature = "bevy")]
    #[test]
    fn accepts_system() {
        let mut world = World::default();
        fn f(_: In<u32>, _: Query<Entity>) -> u32 {
            0
        }
        accept_yarn_fn(world.register_system(f));
    }
    #[cfg(feature = "bevy")]
    #[test]
    fn accepts_systemparam_only_system() {
        let mut world = World::default();
        fn f(_: Query<Entity>) -> u32 {
            0
        }
        accept_yarn_fn(world.register_system(f));
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn accepts_degenerate_system() {
        let mut world = World::default();
        fn f() -> u32 {
            0
        }
        accept_yarn_fn(world.register_system(f));
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn can_call_degenerate_system() {
        let mut world = World::default();
        fn f() -> u32 {
            42
        }
        let id = world.register_system(f);
        let out = id.call(vec![], &mut world);
        assert_eq!(out, 42);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn can_call_system_with_input() {
        let mut world = World::default();
        fn f(In(num): In<u32>) -> u32 {
            num
        }
        let id = world.register_system(f);
        let out = id.call(vec![YarnValue::from(42)], &mut world);
        assert_eq!(out, 42);
    }

    #[cfg(feature = "bevy")]
    #[test]
    fn can_call_system_with_multiple_inputs() {
        let mut world = World::default();
        fn f(In((a, b)): In<(u32, u32)>, _: Query<Entity>) -> u32 {
            a + b
        }
        let id = world.register_system(f);
        let out = id.call(vec![YarnValue::from(40), YarnValue::from(2)], &mut world);
        assert_eq!(out, 42);
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
        #[cfg(feature = "bevy")]
        let out = f.call(input, &mut World::default());
        #[cfg(not(feature = "bevy"))]
        let out = f.call(input);
        out
    }

    mod optionality {
        use super::*;

        macro_rules! assert_is_yarn_fn {
            (($($param:ty),*) -> $ret:ty) => {
                static_assertions::assert_impl_all!(fn($($param),*) -> $ret: YarnFn<fn($($param),*) -> $ret>);
            };
        }

        macro_rules! assert_is_not_yarn_fn {
            (($($param:ty),*) -> $ret:ty) => {
                static_assertions::assert_not_impl_any!(fn($($param),*) -> $ret: YarnFn<fn($($param),*) -> $ret>);
            };
        }

        assert_is_yarn_fn! { (()) -> bool }
        assert_is_yarn_fn! { (Option<()>) -> bool }

        assert_is_yarn_fn! { ((), ()) -> bool }
        assert_is_yarn_fn! { ((), Option<()>) -> bool }
        assert_is_yarn_fn! { (Option<()>, Option<()>) -> bool }
        assert_is_not_yarn_fn! { (Option<()>, ()) -> bool }

        assert_is_yarn_fn! { (Option<()>, Option<()>, Option<()>, Option<()>) -> bool }
        assert_is_not_yarn_fn! { (Option<()>, Option<()>, Option<()>, ()) -> bool }

        assert_is_yarn_fn! { (((), (), ()), ((), Option<()>), (Option<()>, Option<()>)) -> bool }
        assert_is_yarn_fn! { ((), ((), ((), ((), Option<()>)))) -> bool }
        assert_is_not_yarn_fn! { ((), ((), ((), ((), Option<()>))), ()) -> bool }
    }
}
