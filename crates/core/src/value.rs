//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Value.cs>

use crate::prelude::types::Type;

pub mod convertible;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub r#type: Type,
    internal_value: convertible::Convertible,
}

macro_rules! impl_from {
    ($($from_type:ty,)*) => {
        $(
            impl From<$from_type> for Value {
                fn from(value: $from_type) -> Self {
                    Self {
                        r#type: (&value).into(),
                        internal_value: value.into(),
                    }
                }
            }
        )*
    };
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

impl_from![f32, f64, usize, String, bool,];

// The macro above doesn't work for &str because it's trying to work with &&str

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            r#type: value.into(),
            internal_value: value.into(),
        }
    }
}
