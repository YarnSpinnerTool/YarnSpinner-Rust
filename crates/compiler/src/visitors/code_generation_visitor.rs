//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CodeGenerationVisitor.cs>

use crate::compiler;
use crate::listeners::{CompilerListener, Emit};
use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::{ActualParserContext, YarnSpinnerParserContextExt};
use crate::visitors::KnownTypes;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::{CommonToken, Token};
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, Tree};
use rusty_yarn_spinner_core::prelude::instruction::OpCode;
use rusty_yarn_spinner_core::prelude::Operator;
use rusty_yarn_spinner_core::types::Type;
use std::ops::Deref;
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
    /// a regular ol' line of text
    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) -> Self::Return {
        // [sic] TODO: add support for line conditions:
        //
        // Mae: here's a line <<if true>>
        //
        // is identical to
        //
        // <<if true>> Mae: here's a line <<endif>>

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

    /// A set command: explicitly setting a value to an expression <<set $foo to 1>>
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

    fn visit_call_statement(&mut self, ctx: &Call_statementContext<'input>) -> Self::Return {
        // Visit our function call, which will invoke the function
        self.visit(ctx.function_call().unwrap().as_ref());
        // [sic] TODO: if this function returns a value, it will be pushed onto
        // the stack, but there's no way for the compiler to know that, so
        // the stack will not be tidied up. is there a way for that to work?
    }

    /// semi-free form text that gets passed along to the game for things
    /// like <<turn fred left>> or <<unlockAchievement FacePlant>>
    fn visit_command_statement(&mut self, ctx: &Command_statementContext<'input>) -> Self::Return {
        let formatted_text = ctx.command_formatted_text().unwrap();
        let (composed_string, expression_count) = formatted_text.get_children().fold(
            (String::new(), 0_usize),
            |(composed_string, expression_count), node| {
                if node.get_child_count() == 0 {
                    // Terminal node
                    (composed_string + &node.get_text(), expression_count)
                } else {
                    // Generate code for evaluating the expression at runtime
                    self.visit(node.as_ref());
                    // Don't include the '{' and '}', because it will have been
                    // added as a terminal node already
                    (
                        composed_string + &expression_count.to_string(),
                        expression_count + 1,
                    )
                }
            },
        );

        // [sic] TODO: look into replacing this as it seems a bit odd
        match composed_string.as_str() {
            "stop" => {
                // "stop" is a special command that immediately stops
                // execution
                self.compiler_listener.emit(
                    Emit::from_op_code(OpCode::Stop)
                        .with_source_from_token(formatted_text.start().deref()),
                );
            }
            _ => {
                self.compiler_listener.emit(
                    Emit::from_op_code(OpCode::RunCommand)
                        .with_source_from_token(formatted_text.start().deref())
                        .with_operand(composed_string)
                        .with_operand(expression_count),
                );
            }
        }
    }

    /// handles emitting the correct instructions for the function
    fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) -> Self::Return {
        // generate the instructions for all of the parameters
        let expressions = ctx.expression_all();
        for parameter in &expressions {
            self.visit(parameter.as_ref());
        }

        let token = ctx.start();
        // push the number of parameters onto the stack
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushFloat)
                .with_source_from_token(token.deref())
                .with_operand(expressions.len()),
        );

        // then call the function itself
        let function_name = ctx.FUNC_ID().unwrap().get_text();
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::CallFunc)
                .with_source_from_token(token.deref())
                .with_operand(function_name),
        );
    }

    /// if statement ifclause (elseifclause)* (elseclause)? <<endif>>
    fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) -> Self::Return {
        // Implementation note: Idk what this is supposed to do. Looks like a noop.
        // context.AddErrorNode(null);

        // label to give us a jump point for when the if finishes
        let end_of_if_statement_label = self.compiler_listener.register_label("endif");

        // handle the if
        let if_clause = ctx.if_clause().unwrap();
        self.generate_code_for_clause(
            end_of_if_statement_label.clone(),
            if_clause.as_ref(),
            &if_clause.statement_all(),
            if_clause.expression().unwrap(),
        );

        // all elseifs
        for else_if_clause in &ctx.else_if_clause_all() {
            self.generate_code_for_clause(
                end_of_if_statement_label.clone(),
                else_if_clause.as_ref(),
                &else_if_clause.statement_all(),
                else_if_clause.expression().unwrap(),
            );
        }

        // the else, if there is one
        if let Some(else_clause) = ctx.else_clause() {
            self.generate_code_for_clause(
                end_of_if_statement_label.clone(),
                else_clause.as_ref(),
                &else_clause.statement_all(),
                None,
            );
        }

        let current_node = self.compiler_listener.current_node.as_mut().unwrap();
        current_node.labels.insert(
            end_of_if_statement_label,
            current_node.instructions.len() as i32,
        );
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

    fn generate_code_for_clause(
        &mut self,
        jump_label: String,
        ctx: &impl ParserRuleContext<'input>,
        children: &[Rc<StatementContext<'input>>],
        expression: impl Into<Option<Rc<ExpressionContextAll<'input>>>>,
    ) {
        let expression = expression.into();
        let end_of_clause_label = self.compiler_listener.register_label("skipclause");
        // handling the expression (if it has one) will only be called on ifs and elseifs
        if let Some(expression) = expression.clone() {
            // Code-generate the expression
            self.visit(expression.as_ref());

            self.compiler_listener.emit(
                Emit::from_op_code(OpCode::JumpIfFalse)
                    .with_source_from_token(expression.start().deref())
                    .with_operand(end_of_clause_label.clone()),
            );
        }

        // running through all of the children statements
        for child in children {
            self.visit(child.as_ref());
        }

        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::JumpTo)
                .with_source_from_token(ctx.stop().deref())
                .with_operand(jump_label),
        );

        if let Some(expression) = expression {
            let current_node = self.compiler_listener.current_node.as_mut().unwrap();
            current_node
                .labels
                .insert(end_of_clause_label, current_node.instructions.len() as i32);
            self.compiler_listener.emit(
                Emit::from_op_code(OpCode::Pop).with_source_from_token(expression.stop().deref()),
            );
        }
    }
}
