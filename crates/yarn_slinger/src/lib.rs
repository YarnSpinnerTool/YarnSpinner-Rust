pub use log;

pub mod prelude {
    pub use crate::{compiler::*, core::*, runtime::*};
}

pub mod core {
    pub use yarn_slinger_core::prelude::{
        Header, Instruction, InvalidCastError, InvalidOpCodeError, Library, LineId, Node, Position,
        Program, Type, YarnFn, YarnValue,
    };
}
pub mod compiler {
    pub use yarn_slinger_compiler::prelude::*;
    pub use yarn_slinger_compiler::Result as CompilerResult;
}

pub mod runtime {
    pub use yarn_slinger_runtime::markup::{MarkupAttribute, MarkupParseError, MarkupValue};
    pub use yarn_slinger_runtime::prelude::*;
    pub use yarn_slinger_runtime::Result as RuntimeResult;
}
