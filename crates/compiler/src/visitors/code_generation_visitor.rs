use crate::listeners::CompilerListener;
use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::YarnSpinnerParserContextType;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use antlr_rust::tree::ParseTreeVisitorCompat;
use rusty_yarn_spinner_core::prelude::Operator;

pub(crate) struct CodeGenerationVisitor<'a, 'b: 'a, 'input: 'a + 'b> {
    compiler_listener: &'a mut CompilerListener<'b, 'input>,
    tracking_enabled: Option<String>,
    _dummy: (),
}

impl<'a, 'b: 'a, 'input: 'a + 'b> CodeGenerationVisitor<'a, 'b, 'input> {
    pub(crate) fn new(
        compiler_listener: &'a mut CompilerListener<'b, 'input>,
        tracking_enabled: impl Into<Option<String>>,
    ) -> Self {
        Self {
            compiler_listener,
            tracking_enabled: tracking_enabled.into(),
            _dummy: Default::default(),
        }
    }
}

impl<'a, 'b: 'a, 'input: 'a + 'b> ParseTreeVisitorCompat<'input>
    for CodeGenerationVisitor<'a, 'b, 'input>
{
    type Node = YarnSpinnerParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'a, 'b: 'a, 'input: 'a + 'b> YarnSpinnerParserVisitorCompat<'input>
    for CodeGenerationVisitor<'a, 'b, 'input>
{
}

pub(crate) fn token_to_operator(token: isize) -> Option<Operator> {
    // operators for the standard expressions
    match token {
        yarnspinnerlexer::OPERATOR_LOGICAL_LESS_THAN_EQUALS => Some(Operator::LessThanOrEqualTo),
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
