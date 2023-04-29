use crate::prelude::*;
use antlr_rust::token::Token;
use rusty_yarn_spinner_core::prelude::instruction::OpCode;
use rusty_yarn_spinner_core::prelude::*;

#[derive(Debug)]
pub(crate) struct CompilerListener {
    /// The current node to which instructions are being added.
    pub(crate) current_node: Option<Node>,
    /// The current debug information that describes [`current_node`].
    pub(crate) current_debug_info: DebugInfo,
}

impl CompilerListener {
    /// Creates a new instruction, and appends it to a node in the [`Program`].
    pub(crate) fn emit(&mut self, emit: EmitBuilder) {
        let emit = emit.build(self);
        let instruction = Instruction {
            opcode: emit.op_code.into(),
            operands: emit.operands,
        };
        emit.debug_info
            .line_positions
            .insert(emit.node.instructions.len(), emit.source);
        emit.node.instructions.push(instruction);
    }
}

#[derive(Debug)]
struct Emit<'a> {
    node: &'a mut Node,
    debug_info: &'a mut DebugInfo,
    source: Option<Position>,
    op_code: OpCode,
    operands: Vec<Operand>,
}

#[derive(Debug)]
pub(crate) struct EmitBuilder<'a> {
    node: Option<&'a mut Node>,
    debug_info: Option<&'a mut DebugInfo>,
    source: Option<Position>,
    op_code: OpCode,
    operands: Vec<Operand>,
}

impl<'a> EmitBuilder<'a> {
    pub(crate) fn from_op_code(op_code: OpCode) -> Self {
        Self {
            op_code,
            debug_info: Default::default(),
            node: Default::default(),
            source: Default::default(),
            operands: Default::default(),
        }
    }

    pub(crate) fn with_node(mut self, node: &'a mut Node) -> Self {
        self.node = Some(node);
        self
    }

    pub(crate) fn with_debug_info(mut self, debug_info: &'a mut DebugInfo) -> Self {
        self.debug_info = Some(debug_info);
        self
    }

    pub(crate) fn with_source(mut self, source: Position) -> Self {
        self.source = Some(source);
        self
    }

    pub(crate) fn with_operands(mut self, operands: Vec<Operand>) -> Self {
        self.operands = operands;
        self
    }

    pub(crate) fn with_source_from_token(mut self, token: &impl Token) -> Self {
        self.source = Some(Position {
            line: token.get_line() as usize,
            character: token.get_column() as usize,
        });
        self
    }

    fn build(self, compiler: &'a mut CompilerListener) -> Emit<'a> {
        Emit {
            node: self
                .node
                .unwrap_or_else(|| compiler.current_node.as_mut().unwrap()),
            debug_info: self
                .debug_info
                .unwrap_or_else(|| &mut compiler.current_debug_info),
            source: self.source,
            op_code: self.op_code,
            operands: self.operands,
        }
    }
}
