#![allow(missing_debug_implementations)]

use yarnspinner_macros::all_tuples;

/// Marker trait for valid optionality hints.
pub trait Optionality {}

/// An optional parameter or a tuple where
/// the last element is optional.
pub struct Optional;

impl Optionality for Optional {}

/// A parameter that is required.
pub struct Required;

impl Optionality for Required {}

/// A valid chain of optionality hints
/// i.e. a chain where no optional element follows
/// a required element.
pub trait AllowedOptionalityChain {
    /// The optionality hint of the last element in the chain.
    type Last: Optionality;
}

impl AllowedOptionalityChain for () {
    type Last = Required;
}

impl<O: Optionality> AllowedOptionalityChain for (O,) {
    type Last = O;
}

impl<O: Optionality> AllowedOptionalityChain for (Required, O) {
    type Last = O;
}

impl AllowedOptionalityChain for (Optional, Optional) {
    type Last = Optional;
}

// `impl AllowedOptionalityChain for (Optional, Required) {}`
// is intentionally missing (that's the whole point of this trait).

macro_rules! impl_chain {
    // Implementations for zero, one and two-element tuples covered manually.
    () => {};
    ($p1:ident) => {};
    ($p1:ident, $p2:ident) => {};
    ($($param:ident),*) => {
        // A tuple of three or more elements is valid
        // if all two-pairs from left to right are valid.
        // example: (A, B, C) is valid if (A, B) and (B, C) are.
        impl_chain!(@pairwise [$($param),*] [] $($param,)*);
    };
    (@pairwise [$($param:ident),*] [$($tt:tt)*] $a:ident, $b:ident,) => {
        impl_chain!(@emit [$($param),*] [$($tt)* ($a, $b): AllowedOptionalityChain,] $b,);
    };
    (@pairwise [$($param:ident),*] [$($tt:tt)*] $a:ident, $b:ident, $($tail:ident,)*) => {
        impl_chain!(@pairwise [$($param),*] [$($tt)* ($a, $b): AllowedOptionalityChain,] $b, $($tail,)*);
    };
    (@emit [$($param: ident),*] [$($tt:tt)*] $last:ident,) => {
        impl<$($param: Optionality),*> AllowedOptionalityChain for ($($param),*) where $($tt)* {
            type Last = $last;
        }
    };
}

all_tuples!(impl_chain, 0, 16, O);
