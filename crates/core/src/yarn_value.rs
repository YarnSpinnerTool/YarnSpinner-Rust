//! Implements a subset of dotnet's [`Convert`](https://learn.microsoft.com/en-us/dotnet/api/system.convert?view=net-8.0) type.
use crate::prelude::*;
use core::error::Error;
use core::fmt::{Display, Formatter};

/// Represents a Yarn value. The chosen variant corresponds to the last assignment of the value,
/// with the type being inferred from the type checker.
///
/// The type implements meaningful conversions between types through [`TryFrom`] and [`From`].
/// A failure to convert one variant to another will result in an [`YarnValueCastError`].
///
/// ## Implementation Notes
///
/// Corresponds to C#'s [`Convert`](https://docs.microsoft.com/en-us/dotnet/api/system.convert?view=net-5.0) class.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub enum YarnValue {
    /// Any kind of Rust number, i.e. one of `f32`, `f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `isize`.
    /// They are internally stored as `f32` through simple type casts.
    Number(f32),
    /// An owned Rust string.
    String(String),
    /// A Rust boolean.
    Boolean(bool),
}

/// The return value of a [`YarnFn`]. See [`YarnFn`] for more information on the kinds of signatures that can be registered.
///
/// Needed to ensure that the return type of a registered function is
/// able to be turned into a [`YarnValue`], but not a [`YarnValue`] itself.
pub trait IntoYarnValueFromNonYarnValue {
    #[doc(hidden)]
    fn into_yarn_value(self) -> YarnValue;
}

impl YarnValue {
    /// Checks if two [`YarnValue`]s are equal, with a given epsilon for two [`YarnValue::Number`]s.
    /// Note that all equality operations are type-safe, i.e. comparing a [`YarnValue::Number`] to a [`YarnValue::String`] will always return `false`.
    pub fn eq(&self, other: &Self, epsilon: f32) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => (a - b).abs() < epsilon,
            (a, b) => a == b,
        }
    }
}

impl<T> From<&T> for YarnValue
where
    T: Copy,
    YarnValue: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(*value)
    }
}

macro_rules! impl_floating_point {
        ($($from_type:ty,)*) => {
        $(
            impl From<$from_type> for YarnValue {
                fn from(value: $from_type) -> Self {
                    Self::Number(value as f32)
                }
            }

            impl TryFrom<YarnValue> for $from_type {
                type Error = YarnValueCastError;

                fn try_from(value: YarnValue) -> Result<Self, Self::Error> {
                    Self::try_from(&value)
                }
            }

            impl TryFrom<&YarnValue> for $from_type {
                type Error = YarnValueCastError;

                fn try_from(value: &YarnValue) -> Result<Self, Self::Error> {
                    match value {
                        YarnValue::Number(value) => Ok(*value as $from_type),
                        YarnValue::String(value) => value.parse().map_err(Into::into),
                        YarnValue::Boolean(value) => Ok(if *value { 1.0 as $from_type } else { 0.0 }),
                    }
                }
            }


            impl IntoYarnValueFromNonYarnValue for $from_type {
                fn into_yarn_value(self) -> YarnValue {
                    self.into()
                }
            }
        )*
    };
}

impl_floating_point![f32, f64,];

macro_rules! impl_whole_number {
    ($($from_type:ty,)*) => {
        $(
            impl From<$from_type> for YarnValue {
                fn from(value: $from_type) -> Self {
                    Self::Number(value as f32)
                }
            }

            impl TryFrom<YarnValue> for $from_type {
                type Error = YarnValueCastError;

                fn try_from(value: YarnValue) -> Result<Self, Self::Error> {
                    Self::try_from(&value)
                }
            }

            impl TryFrom<&YarnValue> for $from_type {
                type Error = YarnValueCastError;

                fn try_from(value: &YarnValue) -> Result<Self, Self::Error> {
                    f32::try_from(value).map(|value| value as $from_type)
                }
            }

            impl IntoYarnValueFromNonYarnValue for $from_type {
                fn into_yarn_value(self) -> YarnValue {
                    self.into()
                }
            }
        )*
    };
}

impl_whole_number![
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,
];

impl From<YarnValue> for String {
    fn from(value: YarnValue) -> Self {
        match value {
            YarnValue::Number(value) => value.to_string(),
            YarnValue::String(value) => value,
            YarnValue::Boolean(value) => value.to_string(),
        }
    }
}

impl From<&YarnValue> for String {
    fn from(value: &YarnValue) -> Self {
        Self::from(value.clone())
    }
}

impl From<String> for YarnValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for YarnValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl IntoYarnValueFromNonYarnValue for String {
    fn into_yarn_value(self) -> YarnValue {
        self.into()
    }
}

impl TryFrom<YarnValue> for bool {
    type Error = YarnValueCastError;

    fn try_from(value: YarnValue) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&YarnValue> for bool {
    type Error = YarnValueCastError;

    fn try_from(value: &YarnValue) -> Result<Self, Self::Error> {
        match value {
            YarnValue::Number(value) => Ok(*value != 0.0),
            YarnValue::String(value) => value.parse().map_err(Into::into),
            YarnValue::Boolean(value) => Ok(*value),
        }
    }
}

impl From<bool> for YarnValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl IntoYarnValueFromNonYarnValue for bool {
    fn into_yarn_value(self) -> YarnValue {
        self.into()
    }
}

/// Represents a failure to convert one variant of [`YarnValue`] to a base type.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum YarnValueCastError {
    ParseFloatError(core::num::ParseFloatError),
    ParseIntError(core::num::ParseIntError),
    ParseBoolError(core::str::ParseBoolError),
}

impl Error for YarnValueCastError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            YarnValueCastError::ParseFloatError(e) => Some(e),
            YarnValueCastError::ParseIntError(e) => Some(e),
            YarnValueCastError::ParseBoolError(e) => Some(e),
        }
    }
}

impl Display for YarnValueCastError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            YarnValueCastError::ParseFloatError(e) => Display::fmt(e, f),
            YarnValueCastError::ParseIntError(e) => Display::fmt(e, f),
            YarnValueCastError::ParseBoolError(e) => Display::fmt(e, f),
        }
    }
}

impl From<core::num::ParseFloatError> for YarnValueCastError {
    fn from(value: core::num::ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl From<core::num::ParseIntError> for YarnValueCastError {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<core::str::ParseBoolError> for YarnValueCastError {
    fn from(value: core::str::ParseBoolError) -> Self {
        Self::ParseBoolError(value)
    }
}

impl Display for YarnValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
        }
    }
}
