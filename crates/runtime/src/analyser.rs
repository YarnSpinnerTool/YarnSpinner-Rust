//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs

pub(crate) use self::default_analysers::*;
pub use self::{context::*, diagnosis::*};
use std::fmt::Debug;
use yarn_slinger_core::prelude::*;

mod context;
pub(crate) mod default_analysers;
mod diagnosis;

pub trait CompiledProgramAnalyser: Debug {
    fn diagnose(&mut self, program: &Program);
    fn gather_diagnoses(&self) -> Vec<Diagnosis>;
}
