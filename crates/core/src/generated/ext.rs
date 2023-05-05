//! Contains extensions to generated types that in the original implementation are sprinkled around the repo via partial classes

use crate::prelude::instruction::OpCode;
use crate::prelude::*;
use std::fmt::Display;
use thiserror::Error;

impl From<String> for Operand {
    fn from(s: String) -> Self {
        Self {
            value: Some(operand::Value::StringValue(s)),
        }
    }
}

impl From<f32> for Operand {
    fn from(f: f32) -> Self {
        Self {
            value: Some(operand::Value::FloatValue(f)),
        }
    }
}

impl From<usize> for Operand {
    fn from(f: usize) -> Self {
        Self::from(f as f32)
    }
}

impl From<bool> for Operand {
    fn from(b: bool) -> Self {
        Self {
            value: Some(operand::Value::BoolValue(b)),
        }
    }
}

impl TryInto<String> for Operand {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        match self.value {
            Some(operand::Value::StringValue(s)) => Ok(s),
            _ => Err(()),
        }
    }
}

impl TryInto<f32> for Operand {
    type Error = ();

    fn try_into(self) -> Result<f32, Self::Error> {
        match self.value {
            Some(operand::Value::FloatValue(f)) => Ok(f),
            _ => Err(()),
        }
    }
}

impl TryInto<usize> for Operand {
    type Error = ();

    fn try_into(self) -> Result<usize, Self::Error> {
        match self.value {
            // [sic] TODO: we only have float operands, which is
            // unpleasant. we should make 'int' operands a
            // valid type, but doing that implies that the
            // language differentiates between floats and
            // ints itself. something to think about.
            Some(operand::Value::FloatValue(f)) => Ok(f as usize),
            _ => Err(()),
        }
    }
}

impl TryInto<bool> for Operand {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        match self.value {
            Some(operand::Value::BoolValue(b)) => Ok(b),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for OpCode {
    type Error = InvalidOpCodeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::JumpTo),
            1 => Ok(OpCode::Jump),
            2 => Ok(OpCode::RunLine),
            3 => Ok(OpCode::RunCommand),
            4 => Ok(OpCode::AddOption),
            5 => Ok(OpCode::ShowOptions),
            6 => Ok(OpCode::PushString),
            7 => Ok(OpCode::PushFloat),
            8 => Ok(OpCode::PushBool),
            9 => Ok(OpCode::PushNull),
            10 => Ok(OpCode::JumpIfFalse),
            11 => Ok(OpCode::Pop),
            12 => Ok(OpCode::CallFunc),
            13 => Ok(OpCode::PushVariable),
            14 => Ok(OpCode::StoreVariable),
            15 => Ok(OpCode::Stop),
            16 => Ok(OpCode::RunNode),
            _ => Err(InvalidOpCodeError(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, Error)]
pub struct InvalidOpCodeError(pub i32);

impl Display for InvalidOpCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} is not a valid OpCode", self.0)
    }
}

impl Program {
    /// Creates a new Program by merging multiple Programs together.
    ///
    /// The new program will contain every node from every input program.
    /// Returns [`None`] if the input is empty.
    pub fn combine(programs: Vec<Program>) -> Option<Self> {
        if programs.is_empty() {
            return None;
        }
        let mut output = Program::default();
        for program in programs {
            for (node_name, node) in program.nodes {
                assert!(
                    !output.nodes.contains_key(&node_name),
                    "This program already contains a node named {node_name}",
                );
                output.nodes.insert(node_name, node);
            }
            output.initial_values.extend(program.initial_values);
        }
        Some(output)
    }
}
