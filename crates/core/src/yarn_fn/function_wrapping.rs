use crate::prelude::*;
use std::any::TypeId;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use yarn_slinger_macros::all_tuples;

/// A function that can be registered into and called from Yarn.
pub trait YarnFn<Marker> {
    type Out: IntoUntypedValueFromNonUntypedValue + 'static;
    fn call(&self, input: Vec<UntypedValue>) -> Self::Out;
    fn parameter_types(&self) -> Vec<TypeId>;
    fn return_type(&self) -> TypeId {
        TypeId::of::<Self::Out>()
    }
}

/// A [`YarnFn`] with the `Marker` type parameter erased.
pub trait UntypedYarnFn: Debug {
    fn call(&self, input: Vec<UntypedValue>) -> UntypedValue;
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
    Marker: 'static + Clone,
    F: YarnFn<Marker> + 'static + Clone,
    F::Out: IntoUntypedValueFromNonUntypedValue + 'static + Clone,
{
    fn call(&self, input: Vec<UntypedValue>) -> UntypedValue {
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

#[derive(Clone)]
pub(crate) struct YarnFnWrapper<Marker, F>
where
    F: YarnFn<Marker>,
{
    function: F,

    // NOTE: PhantomData<fn()-> T> gives this safe Send/Sync impls
    _marker: PhantomData<fn() -> Marker>,
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

impl PartialEq for Box<dyn UntypedYarnFn> {
    fn eq(&self, other: &Self) -> bool {
        // Not guaranteed to be unique, but that's good enough for our purposes.
        let debug = format!("{:?}", self);
        let other_debug = format!("{:?}", other);
        debug == other_debug
    }
}

impl Eq for Box<dyn UntypedYarnFn> {}

/// Adapted from <https://github.com/bevyengine/bevy/blob/fe852fd0adbce6856f5886d66d20d62cfc936287/crates/bevy_ecs/src/system/system_param.rs#L1370>
macro_rules! impl_yarn_fn_tuple {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<F, O, $($param,)*> YarnFn<fn($($param,)*) -> O> for F
            where
                F: Fn($($param,)*) -> O,
                O: IntoUntypedValueFromNonUntypedValue + 'static,
                $($param: TryFrom<UntypedValue> + 'static,)*
            {
                type Out = O;
                #[allow(non_snake_case)]
                fn call(&self, input: Vec<UntypedValue>) -> Self::Out {
                    let [$($param,)*] = &input[..] else {
                        panic!("Wrong number of arguments")
                    };

                    let input = (
                        $($param
                            .clone()
                            .try_into()
                            .unwrap_or_else(|_| panic!("Failed to convert")),
                        )*
                    );
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
