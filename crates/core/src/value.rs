//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Value.cs>

use crate::prelude::types::Type;

mod untyped_value;
pub use untyped_value::*;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]

/// A value as it appears to the compiler. As a consumer, you should not be facing this type.
///
/// ## Implementation Notes
///
/// Corresponds to the internal `Value` class in the original C# implementation.
pub struct InternalValue {
    /// The proper Yarn type of this value according to the type checker.
    pub r#type: Type,
    /// The actual value
    pub raw_value: UntypedValue,
}

macro_rules! impl_from {
    ($($from_type:ty,)*) => {
        $(
            impl From<$from_type> for InternalValue {
                fn from(value: $from_type) -> Self {
                    Self {
                        r#type: (&value).into(),
                        raw_value: value.into(),
                    }
                }
            }

            impl TryFrom<InternalValue> for $from_type {
                type Error = InvalidCastError;

                fn try_from(value: InternalValue) -> Result<Self, Self::Error> {
                    value.raw_value.try_into()
                }
            }

        )*
    };
}

impl<T> From<&T> for InternalValue
where
    T: Copy,
    InternalValue: From<T>,
{
    fn from(value: &T) -> Self {
        Self::from(*value)
    }
}

impl_from![bool, f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize,];

// The macro above doesn't work for &str because it's trying to work with &&str

impl From<&str> for InternalValue {
    fn from(value: &str) -> Self {
        Self {
            r#type: value.into(),
            raw_value: value.into(),
        }
    }
}

impl From<String> for InternalValue {
    fn from(value: String) -> Self {
        Self {
            r#type: (&value).into(),
            raw_value: value.into(),
        }
    }
}

impl From<InternalValue> for String {
    fn from(value: InternalValue) -> Self {
        value.raw_value.into()
    }
}
