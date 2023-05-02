//! Implements a subset of dotnet's [`Convert`](https://learn.microsoft.com/en-us/dotnet/api/system.convert?view=net-8.0) type.
use thiserror::Error;

/// Represents a Yarn value without a specific type, so something like a generic.
///
/// The type implements meaningful conversions between types through [`TryFrom`] and [`From`].
/// A failure to convert one variant to another will result in an [`InvalidCastError`].
///
/// ## Implementation Notes
///
/// Corresponds to C#'s [`Convert`](https://docs.microsoft.com/en-us/dotnet/api/system.convert?view=net-5.0) class.
#[derive(Debug, Clone, PartialEq)]
pub enum UntypedValue {
    /// Any kind of Rust number, i.e. one of `f32`, `f64`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `isize`.
    /// They are internally stored as `f32` through simple type casts. When pulling out a whole number, the floating point number is rounded.
    Number(f32),
    /// An owned Rust string.
    String(String),
    /// A Rust boolean.
    Boolean(bool),
}

/// Needed to ensure that the return type of a registered function is
/// able to be turned into a [`Value`], but not a [`Value`] itself.
pub trait IntoUntypedValueFromNonUntypedValue {
    fn into_untyped_value(self) -> UntypedValue;
}

impl UntypedValue {
    pub fn eq(&self, other: &Self, epsilon: f32) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => (a - b).abs() < epsilon,
            (a, b) => a == b,
        }
    }
}

impl<T> From<&T> for UntypedValue
where
    T: Copy,
    UntypedValue: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(*value)
    }
}

macro_rules! impl_floating_point {
        ($($from_type:ty,)*) => {
        $(
            impl From<$from_type> for UntypedValue {
                fn from(value: $from_type) -> Self {
                    Self::Number(value as f32)
                }
            }

            impl TryFrom<UntypedValue> for $from_type {
                type Error = InvalidCastError;

                fn try_from(value: UntypedValue) -> Result<Self, Self::Error> {
                    match value {
                        UntypedValue::Number(value) => Ok(value as $from_type),
                        UntypedValue::String(value) => value.parse().map_err(Into::into),
                        UntypedValue::Boolean(value) => Ok(if value { 1.0 as $from_type } else { 0.0 }),
                    }
                }
            }


            impl IntoUntypedValueFromNonUntypedValue for $from_type {
                fn into_untyped_value(self) -> UntypedValue {
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
            impl From<$from_type> for UntypedValue {
                fn from(value: $from_type) -> Self {
                    Self::Number(value as f32)
                }
            }

            impl TryFrom<UntypedValue> for $from_type {
                type Error = InvalidCastError;

                fn try_from(value: UntypedValue) -> Result<Self, Self::Error> {
                    f32::try_from(value).map(|value| value.round() as $from_type)
                }
            }


            impl IntoUntypedValueFromNonUntypedValue for $from_type {
                fn into_untyped_value(self) -> UntypedValue {
                    self.into()
                }
            }
        )*
    };
}

impl_whole_number![i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,];

impl From<UntypedValue> for String {
    fn from(value: UntypedValue) -> Self {
        match value {
            UntypedValue::Number(value) => value.to_string(),
            UntypedValue::String(value) => value,
            UntypedValue::Boolean(value) => value.to_string(),
        }
    }
}

impl From<String> for UntypedValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for UntypedValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl IntoUntypedValueFromNonUntypedValue for String {
    fn into_untyped_value(self) -> UntypedValue {
        self.into()
    }
}

impl TryFrom<UntypedValue> for bool {
    type Error = InvalidCastError;

    fn try_from(value: UntypedValue) -> Result<Self, Self::Error> {
        match value {
            UntypedValue::Number(value) => Ok(value != 0.0),
            UntypedValue::String(value) => value.parse().map_err(Into::into),
            UntypedValue::Boolean(value) => Ok(value),
        }
    }
}

impl From<bool> for UntypedValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl IntoUntypedValueFromNonUntypedValue for bool {
    fn into_untyped_value(self) -> UntypedValue {
        self.into()
    }
}

#[derive(Error, Debug)]
/// Represents a failure to convert one variant of [`UntypedValue`] to a base type.
pub enum InvalidCastError {
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseBoolError(#[from] std::str::ParseBoolError),
}
