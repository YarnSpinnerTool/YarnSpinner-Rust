pub mod bug;

pub mod prelude {
    pub const LINE_ID_PREFIX: &str = "line:";
    pub use crate::bug::UnwrapExt;
    pub use crate::{assert_or_bug, bug};
}
