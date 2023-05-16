pub use log;

pub mod prelude {
    pub mod compiler {
        pub use yarn_slinger_compiler::prelude::*;
    }
    pub use yarn_slinger_core::generated::*;
    pub use yarn_slinger_core::prelude::{
        InvalidCastError, Library, LineId, Operator, Position, Program, YarnFn, YarnFnRegistry,
        YarnValue,
    };
    pub use yarn_slinger_runtime::prelude::*;
}
