//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/TypeUtil.cs>

use crate::types::Type;

pub trait SubTypeOf<T: ?Sized = Self> {
    /// Checks to see if `self` is equal to `parent`,
    /// or if `parent` exists in `self`'s type hierarchy.
    ///
    /// ## Implementation Notes
    ///
    /// The original implementation features the bones of an actual hierarchical type system,
    /// but de facto it was unused. So, this implementation is way simpler, simply checking
    /// for special cases, namely `Type::Any` and `Type::Undefined`.
    ///
    /// Careful, the original implementation has the param order flipped!
    fn is_sub_type_of(&self, parent: &T) -> bool;
}

impl<T> SubTypeOf<T> for Type
where
    Type: From<T>,
    T: Clone,
{
    fn is_sub_type_of(&self, parent: &T) -> bool {
        let parent = Type::from(parent.clone());
        match (self, parent) {
            //  ALL types are a subtype of the Any type, including undefined
            (_, Type::Any) => true,
            (a, b) => *a == b,
        }
    }
}

impl<T> SubTypeOf<T> for Option<Type>
where
    Type: From<T>,
    T: Clone,
{
    fn is_sub_type_of(&self, parent: &T) -> bool {
        let parent = Type::from(parent.clone());
        match (self, parent) {
            //  ALL types are a subtype of the Any type, including undefined
            (_, Type::Any) => true,
            // The subtype is undefined. Assume that it is not a subtype of parent.
            (None, _) => false,
            (Some(a), b) => *a == b,
        }
    }
}

impl<T> SubTypeOf<Option<T>> for Type
where
    Type: From<T>,
    T: Clone,
{
    fn is_sub_type_of(&self, parent: &Option<T>) -> bool {
        let parent = parent.clone().map(|parent| Type::from(parent));
        match (self, parent) {
            //  ALL types are a subtype of the Any type, including undefined
            (_, Some(Type::Any)) => true,
            (_, None) => false,
            (a, Some(b)) => *a == b,
        }
    }
}

impl<T> SubTypeOf<Option<T>> for Option<Type>
where
    Type: From<T>,
    T: Clone,
{
    fn is_sub_type_of(&self, parent: &Option<T>) -> bool {
        let parent = parent.clone().map(|parent| Type::from(parent));
        match (self, parent) {
            //  ALL types are a subtype of the Any type, including undefined
            (_, Some(Type::Any)) => true,
            // The subtype is undefined. Assume that it is not a subtype of parent.
            (None, _) => false,
            (_, None) => false,
            (a, b) => *a == b,
        }
    }
}
