//! Implements a subset of dotnet's [`Convert`](https://learn.microsoft.com/en-us/dotnet/api/system.convert?view=net-8.0) type.
use crate::types::InvalidDowncastError;
use std::any::Any;
use thiserror::Error;

/// Implements meaningful conversions, i.e. impls for [`TryFrom`] and [`From`] from the variants to Rust's base types.
/// A failure to convert one variant to another will result in an [`InvalidCastError`].
#[derive(Debug, Clone, PartialEq)]
pub enum Convertible {
    /// Any kind of Rust number, e.g. `i32`, `f32`, `u64`, `isize`, etc.
    Number(f32),
    /// An owned Rust string.
    String(String),
    /// A Rust boolean.
    Boolean(bool),
}

/// Needed to ensure that the return type of a registered function is
/// able to be turned into a [`Value`], but not a [`Value`] itself.
pub trait IntoConvertibleFromNonConvertible {
    fn into_convertible(self) -> Convertible;
}

impl Convertible {
    pub fn eq(&self, other: &Self, epsilon: f32) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => (a - b).abs() < epsilon,
            (a, b) => a == b,
        }
    }
}

impl TryFrom<Convertible> for f32 {
    type Error = InvalidCastError;

    fn try_from(value: Convertible) -> Result<Self, Self::Error> {
        match value {
            Convertible::Number(value) => Ok(value),
            Convertible::String(value) => value.parse().map_err(Into::into),
            Convertible::Boolean(value) => Ok(if value { 1.0 } else { 0.0 }),
        }
    }
}

impl<T> From<&T> for Convertible
where
    T: Copy,
    Convertible: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(*value)
    }
}

impl From<f32> for Convertible {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}

impl IntoConvertibleFromNonConvertible for f32 {
    fn into_convertible(self) -> Convertible {
        self.into()
    }
}

macro_rules! impl_from_numeral {
    ($($from_type:ty,)*) => {
        $(
            impl From<$from_type> for Convertible {
                fn from(value: $from_type) -> Self {
                    Self::Number(value as f32)
                }
            }

            impl TryFrom<Convertible> for $from_type {
                type Error = InvalidCastError;

                fn try_from(value: Convertible) -> Result<Self, Self::Error> {
                    f32::try_from(value).map(|value| value as $from_type)
                }
            }


            impl IntoConvertibleFromNonConvertible for $from_type {
                fn into_convertible(self) -> Convertible {
                    self.into()
                }
            }
        )*
    };
}

impl_from_numeral![f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,];

impl From<Convertible> for String {
    fn from(value: Convertible) -> Self {
        match value {
            Convertible::Number(value) => value.to_string(),
            Convertible::String(value) => value,
            Convertible::Boolean(value) => value.to_string(),
        }
    }
}

impl From<String> for Convertible {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Convertible {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl IntoConvertibleFromNonConvertible for String {
    fn into_convertible(self) -> Convertible {
        self.into()
    }
}

impl TryFrom<Convertible> for bool {
    type Error = InvalidCastError;

    fn try_from(value: Convertible) -> Result<Self, Self::Error> {
        match value {
            Convertible::Number(value) => Ok(value != 0.0),
            Convertible::String(value) => value.parse().map_err(Into::into),
            Convertible::Boolean(value) => Ok(value),
        }
    }
}

impl From<bool> for Convertible {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl IntoConvertibleFromNonConvertible for bool {
    fn into_convertible(self) -> Convertible {
        self.into()
    }
}

#[derive(Error, Debug)]
/// Represents a failure to convert one variant of [`Convertible`] to a base type.
pub enum InvalidCastError {
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseBoolError(#[from] std::str::ParseBoolError),
}
