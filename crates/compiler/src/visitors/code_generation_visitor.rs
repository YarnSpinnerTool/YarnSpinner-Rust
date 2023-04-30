//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CodeGenerationVisitor.cs>

use crate::compiler;
use crate::listeners::{CompilerListener, Emit};
use crate::parser::generated::yarnspinnerparser::{
    Line_statementContext, Set_statementContext, YarnSpinnerParserContext,
};
use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::{
    Line_statementContextAttrs, Set_statementContextAttrs, YarnSpinnerParserContextType,
};
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::ActualParserContext;
use crate::visitors::{HashableInterval, KnownTypes};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, Tree};
use rusty_yarn_spinner_core::prelude::instruction::OpCode;
use rusty_yarn_spinner_core::prelude::Operator;
use rusty_yarn_spinner_core::types::Type;
use std::collections::HashMap;
use std::rc::Rc;

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

impl<'a, 'input: 'a> YarnSpinnerParserVisitorCompat<'input> for CodeGenerationVisitor<'a, 'input> {
    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) -> Self::Return {
        // Evaluate the inline expressions and push the results onto the
        // stack.
        let formatted_text = ctx.line_formatted_text().unwrap();
        let expression_count =
            self.generate_code_for_expressions_in_formatted_text(formatted_text.get_children());
        let line_id_tag = compiler::get_line_id_tag(&ctx.hashtag_all())
            .expect("Internal error: line should have an implicit or explicit line ID tag, but none was found. This is a bug. Please report it at https://github.com/Mafii/rusty-yarn-spinner/issues/new");
        let line_id = line_id_tag.text.as_ref().unwrap().get_text().to_owned();
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::RunLine)
                .with_source_from_token(&*ctx.start())
                .with_operand(line_id)
                .with_operand(expression_count),
        );
    }

    fn visit_set_statement(&mut self, ctx: &Set_statementContext<'input>) -> Self::Return {
        // Ensure that the correct result is on the stack by evaluating the
        // expression. If this assignment includes an operation (e.g. +=),
        // do that work here too.
        let operator_token = ctx.op.as_ref().unwrap();
        let expression = ctx.expression().unwrap();
        let variable = ctx.variable().unwrap();
        let mut generate_code_for_operation = |op: Operator| {
            let r#type = self
                .compiler_listener
                .types
                .get(expression.as_ref())
                .unwrap()
                .clone();
            self.generate_code_for_operation(
                op,
                operator_token.as_ref(),
                &r#type,
                &[variable.clone(), expression.clone()],
            )
        };
        match operator_token.get_token_type() {
            yarnspinnerlexer::OPERATOR_ASSIGNMENT => {
                self.visit(expression.as_ref());
            }
            yarnspinnerlexer::OPERATOR_MATHS_ADDITION_EQUALS => {
                generate_code_for_operation(Operator::Add);
            }
            yarnspinnerlexer::OPERATOR_MATHS_SUBTRACTION_EQUALS => {
                generate_code_for_operation(Operator::Subtract);
            }
            yarnspinnerlexer::OPERATOR_MATHS_MULTIPLICATION_EQUALS => {
                generate_code_for_operation(Operator::Multiply);
            }
            yarnspinnerlexer::OPERATOR_MATHS_DIVISION_EQUALS => {
                generate_code_for_operation(Operator::Divide);
            }
            yarnspinnerlexer::OPERATOR_MATHS_MODULUS_EQUALS => {
                generate_code_for_operation(Operator::Modulo);
            }
            _ => {
                // ## Implementation note
                // Apparently, we don't do anything here. Maybe a panic would be better?
            }
        }

        // now store the variable and clean up the stack
        let variable_name = variable.get_text().to_owned();
        let token = variable.start();
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::StoreVariable)
                .with_source_from_token(&*token)
                .with_operand(variable_name),
        );
        self.compiler_listener
            .emit(Emit::from_op_code(OpCode::Pop).with_source_from_token(&*token));
    }
}

impl<'a, 'input: 'a> CodeGenerationVisitor<'a, 'input> {
    fn generate_code_for_expressions_in_formatted_text(
        &mut self,
        nodes: impl Iterator<Item = Rc<ActualParserContext<'input>>>,
    ) -> usize {
        // First, visit all of the nodes, which are either terminal text
        // nodes or expressions. if they're expressions, we evaluate them,
        // and inject a positional reference into the final string.

        // If there are zero subnodes: terminal node.
        // nothing to do; string assembly will have been done by the
        // StringTableGeneratorVisitor
        // Otherwise: assume that this is an expression (the parser only
        // permits them to be expressions, but we can't specify that here)
        // -> visit it, and we will emit code that pushes the
        // final value of this expression onto the stack. running
        // the line will pop these expressions off the stack.
        nodes
            .filter_map(|child| (child.get_child_count() > 0).then(|| self.visit(child.as_ref())))
            .count()
    }

    /// Emits code that calls a method appropriate for the operator
    fn generate_code_for_operation(
        &mut self,
        op: Operator,
        operator_token: &impl Token,
        r#type: &Type,
        operands: &[Rc<ActualParserContext<'input>>],
    ) {
        // Generate code for each of the operands, so that their value is
        // now on the stack.
        for operand in operands {
            self.visit(operand.as_ref());
        }

        // Indicate that we are pushing this many items for comparison
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushFloat)
                .with_source_from_token(operator_token)
                .with_operand(operands.len()),
        );
        // Figure out the canonical name for the method that the VM should
        // invoke in order to perform this work
        let method_name = op.to_string();
        let has_method = r#type.has_method(&method_name);
        assert!(
            has_method,
            "Codegen failed to get implementation type for {} given input type {}.",
            op,
            r#type.properties().name,
        );
        let function_name = r#type.get_canonical_name_for_method(&method_name);
        // Call that function.
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::CallFunc)
                .with_source_from_token(operator_token)
                .with_operand(function_name),
        );
    }
}
