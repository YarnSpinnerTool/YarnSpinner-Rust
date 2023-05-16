//! Contains extensions to generated types that in the original implementation are sprinkled around the repo via partial classes

use crate::prelude::instruction::OpCode;
use crate::prelude::*;
use std::fmt::{Debug, Display};
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

impl TryFrom<Operand> for String {
    type Error = ();

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value.value {
            Some(operand::Value::StringValue(s)) => Ok(s),
            _ => Err(()),
        }
    }
}

impl TryFrom<Operand> for f32 {
    type Error = ();

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value.value {
            Some(operand::Value::FloatValue(f)) => Ok(f),
            _ => Err(()),
        }
    }
}

impl TryFrom<Operand> for usize {
    type Error = ();

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value.value {
            // [sic] TODO: we only have float operands, which is
            // unpleasant. we should make 'int' operands a
            // valid type, but doing that implies that the
            // language differentiates between floats and
            // ints, which it doesn't.
            Some(operand::Value::FloatValue(f)) => Ok(f as usize),
            _ => Err(()),
        }
    }
}

impl TryFrom<Operand> for bool {
    type Error = ();

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value.value {
            Some(operand::Value::BoolValue(b)) => Ok(b),
            _ => Err(()),
        }
    }
}

impl From<Operand> for YarnValue {
    fn from(value: Operand) -> Self {
        let value = value.value.unwrap();
        match value {
            operand::Value::StringValue(s) => s.into(),
            operand::Value::FloatValue(f) => f.into(),
            operand::Value::BoolValue(b) => b.into(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect,))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
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

impl Instruction {
    pub fn read_operand<T>(&self, index: usize) -> T
    where
        T: TryFrom<Operand>,
        <T as TryFrom<Operand>>::Error: Debug,
    {
        self.operands[index]
            .clone()
            .try_into()
            .unwrap_or_else(|e| panic!("Failed to convert operand {index}: {e:?}",))
    }
}
