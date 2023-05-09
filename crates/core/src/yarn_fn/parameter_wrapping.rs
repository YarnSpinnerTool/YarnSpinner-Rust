//! This helper code allows us to pass params to YarnFns by value (e.g. `usize`), by reference (e.g. (`&usize`) or by [`std::borrow::Borrow`] (e.g. `String` -> `&str`)
//!
//! Inspired by <https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/chapter2/passing_references.html>

use crate::prelude::*;
use std::any::Any;
use std::borrow::Borrow;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct YarnValueWrapper {
    raw: Option<YarnValue>,
    converted: Option<Box<dyn Any>>,
}

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

pub trait YarnFnParam {
    type Item<'new>;

    fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_>;
}

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

    fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
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

    fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
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

    fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
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

            fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
                ResRef::<$referenced>::retrieve(value).value
            }
        }

        impl YarnFnParam for $referenced {
            type Item<'new> = $referenced;

            fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
                ResOwned::<$referenced>::retrieve(value).value
            }
        }
    };
    ($referenced:ty => $owned:ty: YarnFnParam) => {
        impl YarnFnParam for &$referenced {
            type Item<'new> = &'new $referenced;

            fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
                ResRefBorrow::<$owned, $referenced>::retrieve(value).value
            }
        }

        impl YarnFnParam for &$owned {
            type Item<'new> = &'new $owned;

            fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
                ResRef::<$owned>::retrieve(value).value
            }
        }

        impl YarnFnParam for $owned {
            type Item<'new> = $owned;

            fn retrieve(value: &mut YarnValueWrapper) -> Self::Item<'_> {
                ResOwned::<$owned>::retrieve(value).value
            }
        }
    };
}

impl_yarn_fn_param! {
    [str => String, YarnValue, bool, f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize]: YarnFnParam
}
