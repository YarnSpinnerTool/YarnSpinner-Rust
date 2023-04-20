/// Wrapper around [`PartialEq`] needed because functions called within Yarn cannot accept references.

pub(crate) trait EqByValue {
    fn eq_by_value(self, other: Self) -> bool;
}

impl<T> EqByValue for T
where
    T: PartialEq,
{
    fn eq_by_value(self, other: Self) -> bool {
        self == other
    }
}
