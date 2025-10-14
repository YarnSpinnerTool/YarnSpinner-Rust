use core::fmt::{Arguments, Debug};

#[macro_export]
macro_rules! bug {
    ($message:literal) => {
        $crate::bug!($message,)
    };

    ($message:literal, $($arg:tt)*) => {
        panic!("{}\n{}", format_args!($message, $($arg)*), $crate::bug::BUG_MESSAGE)
    };
}

#[macro_export]
macro_rules! assert_or_bug {
    ($cond:expr $(,)?) => {
        $crate::assert_or_bug!($cond, "Assertion failed: {}", stringify!($cond))
    };

    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            $crate::bug!($($arg)*)
        }
    };
}

/// Contains unwrapping methods that include a link to open a github issue in the panic message
pub trait UnwrapExt: Sized {
    type Output;

    #[doc(hidden)]
    #[track_caller]
    fn unwrap_impl(self, arguments: Arguments) -> Self::Output;

    #[track_caller]
    #[inline]
    fn unwrap_or_bug(self) -> Self::Output {
        self.expect_or_bug("Tried to unwrap a missing or invalid value.")
    }

    #[track_caller]
    #[inline]
    fn expect_or_bug(self, message: &str) -> Self::Output {
        self.unwrap_impl(format_args!("{message}\n{BUG_MESSAGE}"))
    }

    #[track_caller]
    #[inline]
    fn expect_or_bug_with(self, arguments: Arguments) -> Self::Output {
        self.unwrap_impl(format_args!("{arguments}\n{BUG_MESSAGE}"))
    }
}

impl<T> UnwrapExt for Option<T> {
    type Output = T;

    #[inline]
    fn unwrap_impl(self, arguments: Arguments) -> Self::Output {
        self.unwrap_or_else(|| bug!("{arguments}"))
    }
}

impl<T, E: Debug> UnwrapExt for Result<T, E> {
    type Output = T;

    #[inline]
    fn unwrap_impl(self, arguments: Arguments) -> Self::Output {
        self.unwrap_or_else(|err| bug!("{arguments}\n  Caused by: {err:?}"))
    }
}

pub const BUG_MESSAGE: &str = "This is a bug. Please report it at https://github.com/YarnSpinnerTool/YarnSpinner-Rust/issues/new";

#[cfg(test)]
mod test {
    use std::panic;

    use super::*;

    #[test]
    #[should_panic]
    fn bug_macro_panics() {
        bug!("")
    }

    #[test]
    #[should_panic]
    fn bug_macro_with_str_message() {
        let message = *panic::catch_unwind(|| bug!("This is a str message."))
            .unwrap_err()
            .downcast::<String>()
            .unwrap();

        assert_eq!(message, format!("This is a str message.\n{BUG_MESSAGE}"));
    }

    #[test]
    #[should_panic]
    fn bug_macro_with_formatted_message() {
        let message = *panic::catch_unwind(|| bug!("Hello {}", "world."))
            .unwrap_err()
            .downcast::<String>()
            .unwrap();

        assert_eq!(message, format!("Hello world.\n{BUG_MESSAGE}"));
    }
}
