//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Value.cs>

use crate::prelude::*;
use crate::types::Type;

/// A value as it appears to the compiler. It has additional type checker information
/// and may represent values not constructable by the user, like functions.
///
/// As a consumer, you should not be facing this type.
///
/// ## Implementation Notes
///
/// Corresponds to the internal `Value` class in the original C# implementation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct InternalValue {
    /// The proper Yarn type of this value according to the type checker.
    pub r#type: Type,
    /// The actual value. If [`InternalValue::type`] is [`Type::Function`], this is the return type.
    pub raw_value: YarnValue,
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
                type Error = YarnValueCastError;

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

impl From<YarnValue> for InternalValue {
    fn from(value: YarnValue) -> Self {
        Self {
            r#type: (&value).into(),
            raw_value: value,
        }
    }
}

impl From<InternalValue> for YarnValue {
    fn from(value: InternalValue) -> Self {
        value.raw_value
    }
}

impl AsRef<YarnValue> for InternalValue {
    fn as_ref(&self) -> &YarnValue {
        &self.raw_value
    }
}

impl AsMut<YarnValue> for InternalValue {
    fn as_mut(&mut self) -> &mut YarnValue {
        &mut self.raw_value
    }
}
