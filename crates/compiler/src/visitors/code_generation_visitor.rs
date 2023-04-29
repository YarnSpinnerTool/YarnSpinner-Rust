use crate::prelude::generated::yarnspinnerlexer;
use rusty_yarn_spinner_core::prelude::Operator;

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
