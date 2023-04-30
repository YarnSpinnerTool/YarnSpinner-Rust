//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CodeGenerationVisitor.cs>

use crate::listeners::{CompilerListener, Emit};
use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::YarnSpinnerParserContextType;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use antlr_rust::tree::ParseTreeVisitorCompat;
use rusty_yarn_spinner_core::prelude::instruction::OpCode;
use rusty_yarn_spinner_core::prelude::Operator;

pub(crate) struct CodeGenerationVisitor<'a, 'input: 'a> {
    compiler_listener: &'a mut CompilerListener<'input>,
    tracking_enabled: Option<String>,
    _dummy: (),
}

impl<'a, 'input: 'a> CodeGenerationVisitor<'a, 'input> {
    pub(crate) fn new(
        compiler_listener: &'a mut CompilerListener<'input>,
        tracking_enabled: impl Into<Option<String>>,
    ) -> Self {
        Self {
            compiler_listener,
            tracking_enabled: tracking_enabled.into(),
            _dummy: Default::default(),
        }
    }
    pub(crate) fn token_to_operator(token: isize) -> Option<Operator> {
        // operators for the standard expressions
        match token {
            yarnspinnerlexer::OPERATOR_LOGICAL_LESS_THAN_EQUALS => {
                Some(Operator::LessThanOrEqualTo)
            }
            yarnspinnerlexer::OPERATOR_LOGICAL_GREATER_THAN_EQUALS => {
                Some(Operator::GreaterThanOrEqualTo)
            }
            yarnspinnerlexer::OPERATOR_LOGICAL_LESS => Some(Operator::LessThan),
            yarnspinnerlexer::OPERATOR_LOGICAL_GREATER => Some(Operator::GreaterThan),
            yarnspinnerlexer::OPERATOR_LOGICAL_EQUALS => Some(Operator::EqualTo),
            yarnspinnerlexer::OPERATOR_LOGICAL_NOT_EQUALS => Some(Operator::NotEqualTo),
            yarnspinnerlexer::OPERATOR_LOGICAL_AND => Some(Operator::And),
            yarnspinnerlexer::OPERATOR_LOGICAL_OR => Some(Operator::Or),
            yarnspinnerlexer::OPERATOR_LOGICAL_XOR => Some(Operator::Xor),
            yarnspinnerlexer::OPERATOR_LOGICAL_NOT => Some(Operator::Not),
            yarnspinnerlexer::OPERATOR_MATHS_ADDITION => Some(Operator::Add),
            yarnspinnerlexer::OPERATOR_MATHS_SUBTRACTION => Some(Operator::Subtract),
            yarnspinnerlexer::OPERATOR_MATHS_MULTIPLICATION => Some(Operator::Multiply),
            yarnspinnerlexer::OPERATOR_MATHS_DIVISION => Some(Operator::Divide),
            yarnspinnerlexer::OPERATOR_MATHS_MODULUS => Some(Operator::Modulo),
            _ => None,
        }
    }

    // [sic] really ought to make this emit like a list of opcodes actually
    pub(crate) fn generate_tracking_code(compiler: &mut CompilerListener, variable_name: String) {
        // pushing the var and the increment onto the stack
        compiler.emit(Emit::from_op_code(OpCode::PushVariable).with_operand(variable_name.clone()));
        compiler.emit(Emit::from_op_code(OpCode::PushFloat).with_operand(1.));

        // Indicate that we are pushing this many items for comparison
        compiler.emit(Emit::from_op_code(OpCode::PushFloat).with_operand(2.));

        // calling the function
        compiler.emit(Emit::from_op_code(OpCode::CallFunc).with_operand("Number.Add".to_owned()));

        // now store the variable and clean up the stack
        compiler.emit(Emit::from_op_code(OpCode::StoreVariable).with_operand(variable_name));
        compiler.emit(Emit::from_op_code(OpCode::Pop));
    }
}

impl<'a, 'input: 'a> ParseTreeVisitorCompat<'input> for CodeGenerationVisitor<'a, 'input> {
    type Node = YarnSpinnerParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'a, 'input: 'a> YarnSpinnerParserVisitorCompat<'input> for CodeGenerationVisitor<'a, 'input> {}
