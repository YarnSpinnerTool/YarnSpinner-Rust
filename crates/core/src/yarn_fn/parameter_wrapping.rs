//! This helper code allows us to pass params to YarnFns by value (e.g. `usize`), by reference (e.g. (`&usize`) or by [`std::borrow::Borrow`] (e.g. `String` -> `&str`)
//!
//! Inspired by <https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/chapter2/passing_references.html>

use crate::prelude::*;
use std::any::Any;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::slice::IterMut;
use yarn_slinger_macros::all_tuples;

/// Helper class for implementing something like [`YarnFn`] yourself.
/// You probably don't want to use this directly as a consumer unless you're doing some wizardry.
#[derive(Debug)]
pub struct YarnValueWrapper {
    raw: Option<YarnValue>,
    converted: Option<Box<dyn Any>>,
}

pub type YarnValueWrapperIter<'a> = IterMut<'a, YarnValueWrapper>;

impl From<YarnValue> for YarnValueWrapper {
    fn from(value: YarnValue) -> Self {
        Self {
            raw: Some(value),
            converted: None,
        }
    }
}

impl YarnValueWrapper {
    fn convert<T>(&mut self)
    where
        T: TryFrom<YarnValue> + 'static,
        <T as TryFrom<YarnValue>>::Error: Debug,
    {
        let raw = std::mem::take(&mut self.raw).unwrap();
        let converted: T = raw.try_into().unwrap();
        self.converted.replace(Box::new(converted));
    }
}

/// Helper trait for implementing something like [`YarnFn`] yourself.
/// You probably don't want to use this directly as a consumer unless you're doing some wizardry.
pub trait YarnFnParam {
    type Item<'new>: YarnFnParam;

    fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a>;
}

pub type YarnFnParamItem<'a, P> = <P as YarnFnParam>::Item<'a>;

macro_rules! impl_yarn_fn_param_tuple {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<$($param,)*> YarnFnParam for ($($param,)*)
        where $($param: YarnFnParam,)* {
            type Item<'new> = ($($param::Item<'new>,)*);

            #[allow(unused_variables)] // for n = 0 tuples
            fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
               ($($param::retrieve(iter),)*)
            }
        }
    };
}

all_tuples!(impl_yarn_fn_param_tuple, 0, 16, P);

struct ResRef<'a, T>
where
    T: TryFrom<YarnValue> + 'static,
    <T as TryFrom<YarnValue>>::Error: Debug,
{
    value: &'a T,
    phantom_data: PhantomData<T>,
}

impl<'res, T> YarnFnParam for ResRef<'res, T>
where
    T: TryFrom<YarnValue> + 'static,
    <T as TryFrom<YarnValue>>::Error: Debug,
{
    type Item<'new> = ResRef<'new, T>;

    fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
        let value = iter.next().expect("Passed too few arguments to YarnFn");
        value.convert::<T>();
        let converted = value.converted.as_ref().unwrap();
        let value = converted.downcast_ref::<T>().unwrap();
        ResRef {
            value,
            phantom_data: PhantomData,
        }
    }
}

/// For types like `String`, of which a reference to `&str` and not `&String`.
/// These kinds of types implement [`Borrow`], hence this struct's name
struct ResRefBorrow<'a, T, U>
where
    T: TryFrom<YarnValue> + 'static,
    <T as TryFrom<YarnValue>>::Error: Debug,
    T: Borrow<U>,
    U: ?Sized + 'static,
{
    value: &'a U,
    phantom_data: PhantomData<T>,
}

impl<'res, T, U> YarnFnParam for ResRefBorrow<'res, T, U>
where
    T: TryFrom<YarnValue> + 'static,
    <T as TryFrom<YarnValue>>::Error: Debug,
    T: Borrow<U>,
    U: ?Sized + 'static,
{
    type Item<'new> = ResRefBorrow<'new, T, U>;

    fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
        let value = iter.next().expect("Passed too few arguments to YarnFn");
        value.convert::<T>();
        let converted = value.converted.as_ref().unwrap();
        let value = converted.downcast_ref::<T>().unwrap();
        ResRefBorrow {
            value: value.borrow(),
            phantom_data: PhantomData,
        }
    }
}

struct ResOwned<T>
where
    T: TryFrom<YarnValue> + 'static,
    <T as TryFrom<YarnValue>>::Error: Debug,
{
    value: T,
}

impl<T> YarnFnParam for ResOwned<T>
where
    T: TryFrom<YarnValue> + 'static,
    <T as TryFrom<YarnValue>>::Error: Debug,
{
    type Item<'new> = ResOwned<T>;

    fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
        let value = iter.next().expect("Passed too few arguments to YarnFn");
        value.convert::<T>();
        let converted = value.converted.take().unwrap();
        let value = *converted.downcast::<T>().unwrap();
        ResOwned { value }
    }
}

macro_rules! impl_yarn_fn_param {
    ([$($referenced:ty $(=> $owned:ty)?),*]: YarnFnParam) => {
        $(
            impl_yarn_fn_param_inner!{
                $referenced $(=> $owned)?: YarnFnParam
            }
        )*
    }
}

macro_rules! impl_yarn_fn_param_inner {
    ($referenced:ty: YarnFnParam) => {
        impl YarnFnParam for &$referenced {
            type Item<'new> = &'new $referenced;

            fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
                ResRef::<$referenced>::retrieve(iter).value
            }
        }

        impl YarnFnParam for $referenced {
            type Item<'new> = $referenced;

            fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
                ResOwned::<$referenced>::retrieve(iter).value
            }
        }
    };
    ($referenced:ty => $owned:ty: YarnFnParam) => {
        impl YarnFnParam for &$referenced {
            type Item<'new> = &'new $referenced;

            fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
                ResRefBorrow::<$owned, $referenced>::retrieve(iter).value
            }
        }

        impl YarnFnParam for &$owned {
            type Item<'new> = &'new $owned;

            fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
                ResRef::<$owned>::retrieve(iter).value
            }
        }

        impl YarnFnParam for $owned {
            type Item<'new> = $owned;

            fn retrieve<'a>(iter: &mut YarnValueWrapperIter<'a>) -> Self::Item<'a> {
                ResOwned::<$owned>::retrieve(iter).value
            }
        }
    };
}

impl_yarn_fn_param! {
    [str => String, YarnValue, bool, f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize]: YarnFnParam
}
