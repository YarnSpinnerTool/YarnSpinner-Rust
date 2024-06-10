use std::fmt;
use std::ops::{Deref, DerefMut};

/// A wrapper for skipping individual fields when deriving [`fmt::Debug`].
#[derive(Clone, Default)]
pub(crate) struct SkipDebug<T>(pub(crate) T);

impl<T> fmt::Debug for SkipDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("...")
    }
}

impl<T> Deref for SkipDebug<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SkipDebug<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
