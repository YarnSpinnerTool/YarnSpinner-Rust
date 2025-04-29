//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner.Markup/MarkupParseResult.cs>
//! which was split into multiple files.

use crate::prelude::*;
use core::fmt::Display;

/// A value associated with a markup name.
///
/// You do not create instances of this struct yourself. It is created
/// by objects that can parse markup, such as [`Dialogue`]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub enum MarkupValue {
    /// An integer value. Note that while Yarn variables make no distinction between integers and floats, markup values do.
    Integer(u32),
    /// A floating-point value. Note that while Yarn variables make no distinction between integers and floats, markup values do.
    Float(f32),
    /// A string value.
    String(String),
    /// A boolean value.
    Bool(bool),
}

impl MarkupValue {
    /// Returns the name of the enum variant.
    pub fn type_name(&self) -> &'static str {
        match self {
            MarkupValue::Integer(_) => "Integer",
            MarkupValue::Float(_) => "Float",
            MarkupValue::String(_) => "String",
            MarkupValue::Bool(_) => "Bool",
        }
    }
}

impl Display for MarkupValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MarkupValue::Integer(i) => write!(f, "{i}"),
            MarkupValue::Float(fl) => write!(f, "{fl}"),
            MarkupValue::String(s) => write!(f, "{s}"),
            MarkupValue::Bool(b) => write!(f, "{b}"),
        }
    }
}

impl From<String> for MarkupValue {
    fn from(s: String) -> Self {
        MarkupValue::String(s)
    }
}

impl From<&str> for MarkupValue {
    fn from(s: &str) -> Self {
        MarkupValue::String(s.to_string())
    }
}

impl From<u32> for MarkupValue {
    fn from(i: u32) -> Self {
        MarkupValue::Integer(i)
    }
}

impl From<f32> for MarkupValue {
    fn from(f: f32) -> Self {
        MarkupValue::Float(f)
    }
}

impl From<bool> for MarkupValue {
    fn from(b: bool) -> Self {
        MarkupValue::Bool(b)
    }
}
