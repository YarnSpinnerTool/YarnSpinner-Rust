//! # Yarn Slinger
//! The friendly dialogue creation tool for Rust.
//!
//! This crate provides a compiler and runtime that can be used standalone, but will most likely be used by a crate providing the functionality
//! to a game engine. For example, [Bevy](https://bevyengine.org/) engine support is given by the [`bevy_yarn_slinger`](https://crates.io/crates/bevy_yarn_slinger) crate.
#![warn(missing_docs, missing_debug_implementations)]

pub use log;

pub mod prelude {
    //! Everything you need to get started using Yarn Slinger.
    pub use crate::compiler::{
        Compilation, CompilationType, Compiler as YarnCompiler, CompilerError, File as YarnFile,
        LineInfo, Result as YarnCompilerResult, StringInfo,
    };
    pub use crate::core::{
        yarn_fn_registry, IntoYarnValueFromNonYarnValue, Library as YarnFnLibrary, LineId,
        Program as YarnProgram, YarnFn, YarnValue,
    };
    pub use crate::runtime::{
        Command as YarnCommand, CompiledProgramAnalyser as YarnAnalyser,
        Context as YarnAnalysisContext, Dialogue, DialogueError, DialogueEvent, DialogueOption,
        Language, Line as YarnLine, MarkupAttribute, MarkupValue, OptionId,
        Result as YarnRuntimeResult, StringTable, TextProvider, UnsupportedLanguageError,
        VariableStorage,
    };
}

pub mod core {
    //! Core types and traits that are used by both the compiler and runtime.
    pub use yarn_slinger_core::prelude::{
        yarn_fn_registry, yarn_fn_type, Header, Instruction, IntoYarnValueFromNonYarnValue,
        InvalidOpCodeError, Library, LineId, Node, Position, Program, Type, UntypedYarnFn, YarnFn,
        YarnFnParam, YarnFnParamItem, YarnValue, YarnValueCastError, YarnValueWrapper,
        YarnValueWrapperIter,
    };
}
pub mod compiler {
    //! Types and traits used by the compiler, in particular the [`Compiler`] struct.
    pub use yarn_slinger_compiler::prelude::*;
    pub use yarn_slinger_compiler::Result;
}

pub mod runtime {
    //! Types and traits used by the runtime, in particular the [`Dialogue`] struct.
    pub use yarn_slinger_runtime::markup::{
        MarkupAttribute, MarkupParseError, MarkupValue, CHARACTER_ATTRIBUTE,
        CHARACTER_ATTRIBUTE_NAME_PROPERTY, TRIM_WHITESPACE_PROPERTY,
    };
    pub use yarn_slinger_runtime::prelude::*;
    pub use yarn_slinger_runtime::Result;
}
