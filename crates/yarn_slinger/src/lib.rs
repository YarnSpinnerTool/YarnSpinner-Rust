pub use log;

pub mod prelude {
    pub use yarn_slinger_compiler::prelude::*;
    pub use yarn_slinger_core::generated::*;
    pub use yarn_slinger_core::prelude::{
        InvalidCastError, Library, Program, Type, YarnFn, YarnFnRegistry,
    };
    pub use yarn_slinger_runtime::prelude::*;
}
