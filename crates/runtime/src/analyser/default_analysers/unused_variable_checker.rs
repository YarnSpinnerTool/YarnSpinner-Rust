//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Analyser.cs,
//! which was split into multiple files.

use crate::prelude::*;
use std::collections::HashSet;
use yarn_slinger_core::prelude::instruction::*;
use yarn_slinger_core::prelude::*;

#[derive(Debug, Default)]
pub(crate) struct UnusedVariableChecker {
    read_variables: HashSet<String>,
    written_variables: HashSet<String>,
}

impl UnusedVariableChecker {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

impl CompiledProgramAnalyser for UnusedVariableChecker {
    fn diagnose(&mut self, program: &Program) {
        // In each node, find all reads and writes to variables
        let new_variables = program.nodes.values().flat_map(|node| {
            node.instructions
                .iter()
                .filter_map(|instruction| match instruction.opcode() {
                    OpCode::PushVariable | OpCode::StoreVariable => {
                        Some((instruction.opcode(), instruction.operands[0].clone()))
                    }
                    _ => None,
                })
                .map(|(opcode, operand)| (opcode, operand.try_into().unwrap()))
        });
        for (opcode, variable) in new_variables {
            match opcode {
                OpCode::PushVariable => {
                    self.read_variables.insert(variable);
                }
                OpCode::StoreVariable => {
                    self.written_variables.insert(variable);
                }
                _ => unreachable!(),
            }
        }
    }

    fn collect_diagnoses(&self) -> Vec<Diagnosis> {
        // Report the write-only variables
        self.written_variables
            .difference(&self.read_variables)
            .map(|variable| {
                Diagnosis::new(
                    DiagnosisSeverity::Warning,
                    format!("Variable {variable} is assigned, but never read from"),
                )
            })
            .collect()
    }
}
