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
