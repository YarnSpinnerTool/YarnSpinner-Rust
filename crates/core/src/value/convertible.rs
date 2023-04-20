//! Implements a subset of dotnet's [`Convert`](https://learn.microsoft.com/en-us/dotnet/api/system.convert?view=net-8.0) type.
use thiserror::Error;

/// Implements meaningful conversions, i.e. impls for [`TryFrom`] and [`From`] from the variants to Rust's base types.
/// A failure to convert one variant to another will result in an [`InvalidCastError`].
#[derive(Debug, Clone, PartialEq)]
pub enum Convertible {
    Number(f32),
    String(String),
    Bool(bool),
}

impl TryFrom<Convertible> for f32 {
    type Error = InvalidCastError;

    fn try_from(value: Convertible) -> Result<Self, Self::Error> {
        match value {
            Convertible::Number(value) => Ok(value),
            Convertible::String(value) => value.parse().map_err(Into::into),
            Convertible::Bool(value) => Ok(if value { 1.0 } else { 0.0 }),
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

impl TryFrom<Convertible> for f64 {
    type Error = InvalidCastError;

    fn try_from(value: Convertible) -> Result<Self, Self::Error> {
        match value {
            Convertible::Number(value) => Ok(value as f64),
            Convertible::String(value) => value.parse().map_err(Into::into),
            Convertible::Bool(value) => Ok(if value { 1.0 } else { 0.0 }),
        }
    }
}

impl From<f64> for Convertible {
    fn from(value: f64) -> Self {
        Self::Number(value as f32)
    }
}

impl TryFrom<Convertible> for usize {
    type Error = InvalidCastError;

    fn try_from(value: Convertible) -> Result<Self, Self::Error> {
        match value {
            Convertible::Number(value) => Ok(value as usize),
            Convertible::String(value) => value.parse().map_err(Into::into),
            Convertible::Bool(value) => Ok(if value { 1 } else { 0 }),
        }
    }
}

impl From<usize> for Convertible {
    fn from(value: usize) -> Self {
        Self::Number(value as f32)
    }
}

impl TryFrom<Convertible> for String {
    type Error = InvalidCastError;

    fn try_from(value: Convertible) -> Result<Self, Self::Error> {
        match value {
            Convertible::Number(value) => Ok(value.to_string()),
            Convertible::String(value) => Ok(value),
            Convertible::Bool(value) => Ok(value.to_string()),
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

impl TryFrom<Convertible> for bool {
    type Error = InvalidCastError;

    fn try_from(value: Convertible) -> Result<Self, Self::Error> {
        match value {
            Convertible::Number(value) => Ok(value != 0.0),
            Convertible::String(value) => value.parse().map_err(Into::into),
            Convertible::Bool(value) => Ok(value),
        }
    }
}

impl From<bool> for Convertible {
    fn from(value: bool) -> Self {
        Self::Bool(value)
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
