//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs>

pub(crate) use self::default_analysers::*;
pub use self::{context::*, diagnosis::*};
use std::fmt::Debug;
use yarn_slinger_core::prelude::*;

mod context;
pub(crate) mod default_analysers;
mod diagnosis;

pub trait CompiledProgramAnalyser: Debug {
    /// Reads data from the provided program that is later used in [`CompiledProgramAnalyser::collect_diagnoses`].
    fn diagnose(&mut self, program: &Program);

    /// Takes the data collected by [`CompiledProgramAnalyser::diagnose`], analyzes it and returns the resulting [`Diagnosis`] instances.
    ///
    /// ## Implementation note
    /// Corresponds to the original `GatherDiagnoses`, but was renamed to `collect_diagnoses` because that terminology is more idiomatic in Rust.
    fn collect_diagnoses(&self) -> Vec<Diagnosis>;
}
