use crate::prelude::*;
use alloc::borrow::Cow;
use core::fmt;

/// The available operators that can be used with Yarn values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub enum Operator {
    /// A binary operator that represents equality.
    EqualTo,

    /// A binary operator that represents a value being
    /// greater than another.
    GreaterThan,

    /// A binary operator that represents a value being
    /// greater than or equal to another.
    GreaterThanOrEqualTo,

    /// A binary operator that represents a value being less
    /// than another.
    LessThan,

    /// A binary operator that represents a value being less
    /// than or equal to another.
    LessThanOrEqualTo,

    /// A binary operator that represents
    /// inequality.
    NotEqualTo,

    /// A binary operator that represents a logical
    /// or.
    Or,

    /// A binary operator that represents a logical
    /// and.
    And,

    /// A binary operator that represents a logical exclusive
    /// or.
    Xor,

    /// A binary operator that represents a logical
    /// not.
    Not,

    /// A unary operator that represents negation.
    ///
    /// ## Implementation note
    ///
    /// This is called `UnaryMinus` in the original implementation, but was
    /// renamed for consistency with the other operators.
    UnarySubtract,

    /// A binary operator that represents addition.
    Add,

    /// A binary operator that represents
    /// subtraction.
    ///
    /// ## Implementation note
    ///
    /// This is called `Minus` in the original implementation, but was
    /// renamed for consistency with the other operators.
    Subtract,

    /// A binary operator that represents
    /// multiplication.
    Multiply,

    /// A binary operator that represents division.
    Divide,

    /// A binary operator that represents the remainder
    /// operation.
    Modulo,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::EqualTo => f.write_str("EqualTo"),
            Operator::GreaterThan => f.write_str("GreaterThan"),
            Operator::GreaterThanOrEqualTo => f.write_str("GreaterThanOrEqualTo"),
            Operator::LessThan => f.write_str("LessThan"),
            Operator::LessThanOrEqualTo => f.write_str("LessThanOrEqualTo"),
            Operator::NotEqualTo => f.write_str("NotEqualTo"),
            Operator::Or => f.write_str("Or"),
            Operator::And => f.write_str("And"),
            Operator::Xor => f.write_str("Xor"),
            Operator::Not => f.write_str("Not"),
            Operator::UnarySubtract => f.write_str("UnarySubtract"),
            Operator::Add => f.write_str("Add"),
            Operator::Subtract => f.write_str("Subtract"),
            Operator::Multiply => f.write_str("Multiply"),
            Operator::Divide => f.write_str("Divide"),
            Operator::Modulo => f.write_str("Modulo"),
        }
    }
}

/// Implementing this is probably bad practice, but this greatly reduces boilerplate when used with `yarn_fn_registry!`
impl From<Operator> for Cow<'static, str> {
    fn from(value: Operator) -> Self {
        value.to_string().into()
    }
}
