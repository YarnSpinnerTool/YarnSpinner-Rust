//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Value.cs>

use crate::prelude::convertible::Convertible;
use crate::prelude::{convertible::InvalidCastError, types::Type};
use std::any::Any;

pub mod convertible;

#[derive(Debug, Clone, PartialEq, Default)]
/// A value appearing in a Yarn program. Convert it into a Rust type using
/// the [`TryFrom`] trait.
pub struct Value {
    /// The proper Yarn type according to the type checker of this value.
    pub r#type: Option<Type>,
    pub(crate) internal_value: Option<Convertible>,
}

macro_rules! impl_from {
    ($($from_type:ty,)*) => {
        $(
            impl From<$from_type> for Value {
                fn from(value: $from_type) -> Self {
                    Self {
                        r#type: Some((&value).into()),
                        internal_value: Some(value.into()),
                    }
                }
            }

            impl TryFrom<Value> for $from_type {
                type Error = InvalidCastError;

                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    let convertible: Convertible = value.internal_value.try_into()?;
                    convertible.try_into()
                }
            }
        )*
    };
}

impl TryInto<Convertible> for Option<Convertible> {
    type Error = InvalidCastError;

    fn try_into(self) -> Result<Convertible, Self::Error> {
        match self {
            Some(convertible) => Ok(convertible),
            None => Err(InvalidCastError::UninitializedValue),
        }
    }
}

impl<T> From<&T> for Value
where
    T: Copy,
    Value: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(*value)
    }
}

impl_from![bool, f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,];

// The macro above doesn't work for &str because it's trying to work with &&str

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            r#type: Some(value.into()),
            internal_value: Some(value.into()),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            r#type: Some((&value).into()),
            internal_value: Some(value.into()),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = InvalidCastError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let convertible: Convertible = value.internal_value.try_into()?;
        Ok(convertible.into())
    }
}

impl TryFrom<Box<dyn Any>> for Value {
    type Error = InvalidCastError;
    fn try_from(value: Box<dyn Any>) -> Result<Self, Self::Error> {
        Ok(Self {
            r#type: Some((&value).into()),
            internal_value: Some(value.try_into()?),
        })
    }
}

impl TryFrom<Value> for Box<dyn Any> {
    type Error = InvalidCastError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let convertible: Convertible = value.internal_value.try_into()?;
        Ok(convertible.into())
    }
}

impl TryFrom<Value> for Convertible {
    type Error = InvalidCastError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        value.internal_value.try_into()
    }
}
