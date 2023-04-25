//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/TypeUtil.cs>

use crate::types::Type;

pub trait SubTypeOf<SubType: ?Sized = Self, Parent: ?Sized = Self> {
    /// Checks to see if `self` is equal to
    /// `parent`, or if `parent` exists in `self`'s type
    /// hierarchy.
    ///
    /// ## Implementation Notes
    ///
    /// The original implementation features the bones of an actual hierarchical type system,
    /// but de factor it was unused. So, this implementation is way simpler, simply checking
    /// for special cases, namely `BuiltinType::Any` and `BuiltinType::Undefined`.
    fn is_sub_type_of(&self, parent: &Parent) -> bool;
}

// The blanket impl catches both [`Type`] and [`BuiltInType`].
impl<SubType, Parent> SubTypeOf<SubType, Parent> for SubType
where
    SubType: Clone,
    Type: From<SubType> + From<Parent>,
    Parent: Clone,
{
    fn is_sub_type_of(&self, parent: &Parent) -> bool {
        let self_type = Type::from(self.clone());
        let parent_type = Type::from(parent.clone());
        match (self_type, parent_type) {
            //  ALL types are a subtype of the Any type, including undefined
            (_, Type::Any(_)) => true,
            // The subtype is undefined. Assume that it is not a subtype of parent.
            (Type::Undefined, _) => false,
            (_, Type::Undefined) => {
                unreachable!("A parent type ended up being undefined. This is a bug. Please report it at https://github.com/Mafii/rusty-yarn-spinner/issues/new")
            }
            (a, b) => a == b,
        }
    }
}

/// Wrapper around [`PartialEq`] needed because functions called within Yarn cannot accept references.
pub(crate) trait PartialEqByValue {
    fn eq_by_value(self, other: Self) -> bool;
    fn ne_by_value(self, other: Self) -> bool;
}

impl<T> PartialEqByValue for T
where
    T: PartialEq,
{
    fn eq_by_value(self, other: Self) -> bool {
        self == other
    }

    fn ne_by_value(self, other: Self) -> bool {
        self != other
    }
}

/// Wrapper around [`PartialOrd`] needed because functions called within Yarn cannot accept references.
pub(crate) trait PartialOrdByValue {
    fn lt_by_value(self, other: Self) -> bool;
    fn le_by_value(self, other: Self) -> bool;
    fn gt_by_value(self, other: Self) -> bool;
    fn ge_by_value(self, other: Self) -> bool;
}

impl<T> PartialOrdByValue for T
where
    T: PartialOrd,
{
    fn lt_by_value(self, other: Self) -> bool {
        self < other
    }

    fn le_by_value(self, other: Self) -> bool {
        self <= other
    }

    fn gt_by_value(self, other: Self) -> bool {
        self > other
    }

    fn ge_by_value(self, other: Self) -> bool {
        self >= other
    }
}
