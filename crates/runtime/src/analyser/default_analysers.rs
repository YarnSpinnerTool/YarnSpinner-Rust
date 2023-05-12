use self::{unused_variable_checker::*, variable_lister::*};
use crate::prelude::*;

mod unused_variable_checker;
mod variable_lister;

macro_rules! boxes {
    ($($x:ident),*) => {
        vec![$(Box::new($x::new())),*]
    };
}
pub(crate) fn default_analysers() -> Vec<Box<dyn CompiledProgramAnalyser>> {
    boxes![VariableLister, UnusedVariableChecker]
}
