//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs,
//! which was split into multiple files.

use crate::prelude::*;
use std::collections::HashSet;
use yarn_slinger_core::prelude::instruction::*;
use yarn_slinger_core::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct VariableLister {
    variables: HashSet<String>,
}

impl VariableLister {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl CompiledProgramAnalyser for VariableLister {
    fn diagnose(&mut self, program: &Program) {
        // In each node, find all reads and writes to variables
        let new_variables = program.nodes.values().flat_map(|node| {
            node.instructions
                .iter()
                .filter_map(|instruction| match instruction.opcode() {
                    OpCode::PushVariable | OpCode::StoreVariable => {
                        Some(instruction.operands[0].clone())
                    }
                    _ => None,
                })
                .map(|operand| operand.try_into().unwrap())
        });
        self.variables.extend(new_variables);
    }

    fn gather_diagnoses(&self) -> Vec<Diagnosis> {
        self.variables
            .iter()
            .map(|variable| {
                Diagnosis::new(
                    DiagnosisSeverity::Note,
                    format!("Script uses variable {}", variable),
                )
            })
            .collect()
    }
}
