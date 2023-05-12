//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs,
//! which was split into multiple files.

use crate::prelude::*;
use yarn_slinger_core::prelude::*;

#[derive(Debug)]
pub struct Context(Vec<Box<dyn CompiledProgramAnalyser>>);

impl IntoIterator for Context {
    type Item = Box<dyn CompiledProgramAnalyser>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<Box<dyn CompiledProgramAnalyser>> for Context {
    fn extend<T: IntoIterator<Item = Box<dyn CompiledProgramAnalyser>>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl Context {
    #[must_use]
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn default_analysers() -> Self {
        let mut context = Self::empty();
        for analyser in default_analysers() {
            context = context.add_analyser(analyser);
        }
        context
    }

    #[must_use]
    pub fn add_analyser(mut self, analyser: Box<dyn CompiledProgramAnalyser>) -> Self {
        self.0.push(analyser);
        self
    }

    #[must_use]
    pub fn finish_analysis(&self) -> Vec<Diagnosis> {
        self.0
            .iter()
            .flat_map(|analyser| analyser.collect_diagnoses())
            .collect()
    }

    /// ## Implementation notes
    /// Corresponds to the original `AddProgramToAnalysis`
    pub(crate) fn diagnose_program(&mut self, program: &Program) {
        for analyser in &mut self.0 {
            analyser.diagnose(program);
        }
    }
}
