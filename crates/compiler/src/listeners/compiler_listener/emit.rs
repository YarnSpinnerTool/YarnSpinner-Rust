use crate::listeners::CompilerListener;
use crate::prelude::*;
use antlr_rust::token::Token;
use rusty_yarn_spinner_core::prelude::instruction::OpCode;
use rusty_yarn_spinner_core::prelude::*;

impl<'a, 'input: 'a> CompilerListener<'a, 'input> {
    /// Creates a new instruction, and appends it to a node in the [`Program`].
    pub(crate) fn emit(&mut self, emit: Emit) {
        let instruction = Instruction {
            opcode: emit.op_code.into(),
            operands: emit.operands,
        };

        let current_node = self.current_node.as_mut().unwrap();
        self.current_debug_info
            .line_positions
            .insert(current_node.instructions.len(), emit.source);
        current_node.instructions.push(instruction);
    }
}

#[derive(Debug)]
pub(crate) struct Emit {
    source: Option<Position>,
    op_code: OpCode,
    operands: Vec<Operand>,
}

impl Emit {
    pub(crate) fn from_op_code(op_code: OpCode) -> Self {
        Self {
            op_code,
            source: Default::default(),
            operands: Default::default(),
        }
    }

    pub(crate) fn with_source(mut self, source: Position) -> Self {
        self.source = Some(source);
        self
    }

    pub(crate) fn with_operand(mut self, operand: impl Into<Operand>) -> Self {
        self.operands.push(operand.into());
        self
    }

    #[allow(dead_code)]
    pub(crate) fn with_source_from_token(mut self, token: &impl Token) -> Self {
        self.source = Some(Position {
            line: token.get_line() as usize,
            character: token.get_column() as usize,
        });
        self
    }
}
