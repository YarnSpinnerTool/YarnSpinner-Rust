macro_rules! string_newtype {
    ($(#[$attr:meta])* pub struct $name:ident(pub String);) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name(pub String);

        impl std::ops::Deref for $name {
            type Target = String;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }
    }
}
pub(crate) use string_newtype;
