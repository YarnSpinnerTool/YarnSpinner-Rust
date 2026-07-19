//! Contains extensions to generated types that in the original implementation are sprinkled around the repo via partial classes

use crate::generated::instruction::InstructionType;
use crate::prelude::*;
use core::error::Error;
use core::fmt::{Debug, Display};
use std::iter::empty;

impl From<String> for Operand {
    fn from(s: String) -> Self {
        Self {
            value: Some(OperandValue::StringValue(s)),
        }
    }
}

impl From<f32> for Operand {
    fn from(f: f32) -> Self {
        Self {
            value: Some(OperandValue::FloatValue(f)),
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
            value: Some(OperandValue::BoolValue(b)),
        }
    }
}

impl TryFrom<Operand> for String {
    type Error = ();

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value.value {
            Some(OperandValue::StringValue(s)) => Ok(s),
            _ => Err(()),
        }
    }
}

impl TryFrom<Operand> for f32 {
    type Error = ();

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value.value {
            Some(OperandValue::FloatValue(f)) => Ok(f),
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
            Some(OperandValue::FloatValue(f)) => Ok(f as usize),
            _ => Err(()),
        }
    }
}

impl TryFrom<Operand> for bool {
    type Error = ();

    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value.value {
            Some(OperandValue::BoolValue(b)) => Ok(b),
            _ => Err(()),
        }
    }
}

impl From<Operand> for YarnValue {
    fn from(value: Operand) -> Self {
        let value = value.value.unwrap();
        match value {
            OperandValue::StringValue(s) => s.into(),
            OperandValue::FloatValue(f) => f.into(),
            OperandValue::BoolValue(b) => b.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct InvalidOpCodeError(pub i32);

impl Error for InvalidOpCodeError {}

impl Display for InvalidOpCodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

#[derive(Debug)]
pub enum InstructionCastError {
    IncorrectInstructionType,
    NotADestinationInstructionType
}

macro_rules! instruction_from {
    ($( $enum_val:ident($instruction:ty) ),+) => {
        $(
            impl From<$instruction> for Instruction {
                fn from(value: $instruction) -> Self {
                    Self { instruction_type: Some(InstructionType::$enum_val(value)) }
                }
            }

            impl<'a> TryFrom<&'a Instruction> for &'a $instruction {
                type Error = InstructionCastError;

                fn try_from(value: &'a Instruction) -> Result<Self, Self::Error> {
                    match &value.instruction_type {
                        Some(InstructionType::$enum_val(t)) => Ok(t),
                        _ => Err(Self::Error::IncorrectInstructionType)
                    }
                }
            }

            impl<'a> TryFrom<&'a mut Instruction> for &'a mut $instruction {
                type Error = InstructionCastError;

                fn try_from(value: &'a mut Instruction) -> Result<Self, Self::Error> {
                    match &mut value.instruction_type {
                        Some(InstructionType::$enum_val(t)) => Ok(t),
                        _ => Err(Self::Error::IncorrectInstructionType)
                    }
                }
            }
        )*
    };
}

instruction_from! {
    JumpTo(JumpToInstruction),
    PeekAndJump(PeekAndJumpInstruction),
    RunLine(RunLineInstruction),
    RunCommand(RunCommandInstruction),
    AddOption(AddOptionInstruction),
    ShowOptions(ShowOptionsInstruction),
    PushString(PushStringInstruction),
    PushFloat(PushFloatInstruction),
    PushBool(PushBoolInstruction),
    JumpIfFalse(JumpIfFalseInstruction),
    Pop(PopInstruction),
    CallFunc(CallFunctionInstruction),
    PushVariable(PushVariableInstruction),
    StoreVariable(StoreVariableInstruction),
    Stop(StopInstruction),
    RunNode(RunNodeInstruction),
    PeekAndRunNode(PeekAndRunNodeInstruction),
    DetourToNode(DetourToNodeInstruction),
    PeekAndDetourToNode(PeekAndDetourToNode),
    Return(ReturnInstruction),
    AddSaliencyCandidate(AddSaliencyCandidateInstruction),
    AddSaliencyCandidateFromNode(AddSaliencyCandidateFromNodeInstruction),
    SelectSaliencyCandidate(SelectSaliencyCandidateInstruction)
}

impl Node {

    pub fn get_header(&self, key: &str) -> Option<String> {
        return self.headers.iter().find_map(|h| if h.key == key { Some(h.value.clone()) } else { None })
    }

    pub fn tags(&self) -> Vec<String> {
        let Some(tags) = self.get_header("tags") else { return empty().collect(); };

        tags.split(' ').map(|t| t.into()).collect()
    }
}

pub trait Destination {
    fn destination(&self) -> usize;
    fn set_destination(&mut self, destination: usize);
}

macro_rules! impl_destination {
    ($($enum_val:ident($ty:ident)),+) => {
        $(
            impl Destination for $ty {
                fn destination(&self) -> usize {
                    self.destination as usize
                }

                fn set_destination(&mut self, destination: usize) {
                    self.destination = destination as i32;
                }
            }
        )+

        impl<'a> TryFrom<&'a mut Instruction> for &'a mut dyn Destination {
            type Error = InstructionCastError;

            fn try_from(value: &'a mut Instruction) -> Result<Self, Self::Error> {
                match &mut value.instruction_type {
                    $(
                        Some(InstructionType::$enum_val(val)) => {
                            Ok(val as &mut dyn Destination)
                        },
                    )+
                    _ => Err(InstructionCastError::NotADestinationInstructionType)
                }
            }
        }
    }
}

impl_destination!(
    JumpTo(JumpToInstruction),
    AddOption(AddOptionInstruction),
    JumpIfFalse(JumpIfFalseInstruction),
    AddSaliencyCandidate(AddSaliencyCandidateInstruction),
    AddSaliencyCandidateFromNode(AddSaliencyCandidateFromNodeInstruction)
);