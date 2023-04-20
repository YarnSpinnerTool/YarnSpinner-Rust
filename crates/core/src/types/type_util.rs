use crate::prelude::Value;
use std::fmt::Debug;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub};
use std::process::Output;

impl Value {
    pub(crate) fn eq_by_value<T: PartialEq + TryFrom<Value>>(self, other: Self) -> bool
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        // Using closure instead of `PartialEq::eq` because we only pass Yarn function parameters by value
        self.compare(other, |a: T, b: T| a == b)
    }

    pub(crate) fn ne_by_value<T: PartialEq + TryFrom<Value>>(self, other: Self) -> bool
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.compare(other, |a: T, b: T| a != b)
    }

    pub(crate) fn not<T: Not<Output = T> + TryFrom<Value>>(self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.map::<T, _>(Not::not)
    }

    /// Logical functions depend on `BitXX` traits because that's the closest thing the standard library offers
    pub(crate) fn and<T: BitAnd<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, BitAnd::bitand)
    }

    pub(crate) fn or<T: BitOr<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, BitOr::bitor)
    }

    pub(crate) fn xor<T: BitXor<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, BitXor::bitxor)
    }

    pub(crate) fn add<T: Add<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, Add::add)
    }

    pub(crate) fn sub<T: Sub<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, Sub::sub)
    }

    pub(crate) fn mul<T: Mul<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, Mul::mul)
    }

    pub(crate) fn div<T: Div<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, Div::div)
    }

    pub(crate) fn rem<T: Rem<Output = T> + TryFrom<Value>>(self, other: Self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.combine(other, Rem::rem)
    }

    pub(crate) fn neg<T: Neg<Output = T> + TryFrom<Value>>(self) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.map::<T, _>(Neg::neg)
    }

    pub(crate) fn lt_by_value<T: PartialOrd + TryFrom<Value>>(self, other: Self) -> bool
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.compare(other, |a: T, b: T| a < b)
    }

    pub(crate) fn gt_by_value<T: PartialOrd + TryFrom<Value>>(self, other: Self) -> bool
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.compare(other, |a: T, b: T| a > b)
    }

    pub(crate) fn le_by_value<T: PartialOrd + TryFrom<Value>>(self, other: Self) -> bool
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.compare(other, |a: T, b: T| a <= b)
    }

    pub(crate) fn ge_by_value<T: PartialOrd + TryFrom<Value>>(self, other: Self) -> bool
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        self.compare(other, |a: T, b: T| a >= b)
    }

    fn convert_self_and_other<T: TryFrom<Value>>(self, other: Self) -> (T, T)
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        let inner_self: T = self.try_into().unwrap();
        let inner_other: T = other.try_into().unwrap();
        (inner_self, inner_other)
    }

    fn combine<T: TryFrom<Value>, F: FnOnce(T, T) -> T>(self, other: Self, f: F) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        let (inner_self, inner_other) = self.convert_self_and_other(other);
        f(inner_self, inner_other)
    }

    fn compare<T: TryFrom<Value>, F: FnOnce(T, T) -> bool>(self, other: Self, f: F) -> bool
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        let (inner_self, inner_other) = self.convert_self_and_other(other);
        f(inner_self, inner_other)
    }

    fn map<T: TryFrom<Value>, F: FnOnce(T) -> T>(self, f: F) -> T
    where
        <T as TryFrom<Value>>::Error: Debug,
    {
        let inner_self: T = self.try_into().unwrap();
        f(inner_self)
    }
}
