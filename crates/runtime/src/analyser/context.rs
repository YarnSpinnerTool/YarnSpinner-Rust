//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs>,
//! which was split into multiple files.

use crate::prelude::*;
use yarn_slinger_core::prelude::*;

/// A structure that holds several [`CompiledProgramAnalyser`]s which are used to analyse one or more compiled Yarn programs with [`Dialogue::analyse`].
/// To get the analysis results, call [`Context::finish_analysis`] afterwards.
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
    /// Creates a new empty [`Context`] with no analysers.
    #[must_use]
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Sets up a [`Context`] with the default analysers. These are:
    /// - Variable Lister: Adds a [`DiagnosisSeverity::Note`] diagnosis for each variable in the program.
    /// - Unused Variable Checker: Adds a [`DiagnosisSeverity::Warning`] diagnosis for each unused variable in the program.
    #[must_use]
    pub fn default_analysers() -> Self {
        let mut context = Self::empty();
        for analyser in default_analysers() {
            context = context.add_analyser(analyser);
        }
        context
    }

    /// Adds an analyser to the [`Context`].
    #[must_use]
    pub fn add_analyser(mut self, analyser: Box<dyn CompiledProgramAnalyser>) -> Self {
        self.0.push(analyser);
        self
    }

    /// Collects the diagnoses from all analysers in the [`Context`] that were previously used with [`Dialogue::analyse`].
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
