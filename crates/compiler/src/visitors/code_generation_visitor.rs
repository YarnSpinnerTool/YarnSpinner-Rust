//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CodeGenerationVisitor.cs>

use crate::compiler;
use crate::listeners::{CompilerListener, Emit};
use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::ActualParserContext;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
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
        let variable_name = variable.get_text();
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

    /// for the shortcut options (-> line of text <<if expression>> indent statements dedent)+
    fn visit_shortcut_option_statement(
        &mut self,
        ctx: &Shortcut_option_statementContext<'input>,
    ) -> Self::Return {
        let end_of_group_label = self.compiler_listener.register_label("group_end");
        let mut labels = Vec::new();

        // For each option, create an internal destination label that, if
        // the user selects the option, control flow jumps to. Then,
        // evaluate its associated line_statement, and use that as the
        // option text. Finally, add this option to the list of upcoming
        // options.
        for (option_count, shortcut) in ctx.shortcut_option_all().into_iter().enumerate() {
            // Generate the name of internal label that we'll jump to if
            // this option is selected. We'll emit the label itself later.
            // ## Implementation note
            // The original uses no null propagation and checks for a null where none can be,
            // so this implementation places the `map` where we think it was intended to be.
            let name = self
                .compiler_listener
                .current_node
                .as_ref()
                .map(|node| node.name.clone())
                .unwrap_or_else(|| "node".to_string());
            let option_destination_label = self
                .compiler_listener
                .register_label(format!("shortcutoption_{name}_{}", option_count + 1).as_str());
            labels.push(option_destination_label.clone());

            // This line statement may have a condition on it. If it does,
            // emit code that evaluates the condition, and add a flag on the
            // 'Add Option' instruction that indicates that a condition exists.
            let has_line_condition = if let Some(expression) = shortcut
                .line_statement()
                .and_then(|ctx| ctx.line_condition())
                .and_then(|ctx| ctx.expression())
            {
                // Evaluate the condition, and leave it on the stack
                self.visit(expression.as_ref());
                true
            } else {
                false
            };

            // We can now prepare and add the option.

            // Start by figuring out the text that we want to add. This will
            // involve evaluating any inline expressions.
            let line_statement = shortcut.line_statement().unwrap();
            let expression_count = self.generate_code_for_expressions_in_formatted_text(
                line_statement.line_formatted_text().unwrap().get_children(),
            );

            // Get the line ID from the hashtags if it has one
            let line_id_tag = compiler::get_line_id_tag(&line_statement.hashtag_all())
                .expect("Internal error: no line ID provided. This is a bug. Please report it at https://github.com/Mafii/rusty-yarn-spinner/issues/new");
            let line_id = line_id_tag.text.as_ref().unwrap().get_text().to_owned();

            // And add this option to the list.
            self.compiler_listener.emit(
                Emit::from_op_code(OpCode::AddOption)
                    .with_source_from_token(line_statement.start().deref())
                    .with_operand(line_id)
                    .with_operand(option_destination_label)
                    .with_operand(expression_count)
                    .with_operand(has_line_condition),
            );
        }
        // All of the options that we intend to show are now ready to go.
        let token = ctx.stop();
        self.compiler_listener
            .emit(Emit::from_op_code(OpCode::ShowOptions).with_source_from_token(token.deref()));

        // The top of the stack now contains the name of the label we want
        // to jump to. Jump to it now.
        self.compiler_listener
            .emit(Emit::from_op_code(OpCode::Jump).with_source_from_token(token.deref()));

        // We'll now emit the labels and code associated with each option.
        for (option_count, shortcut) in ctx.shortcut_option_all().into_iter().enumerate() {
            // Emit the label for this option's code
            let current_node = self.compiler_listener.current_node.as_mut().unwrap();
            current_node.labels.insert(
                labels[option_count].clone(),
                current_node.instructions.len() as i32,
            );

            // Run through all the children statements of the shortcut option
            for child in shortcut.statement_all() {
                self.visit(child.as_ref());
            }

            // Jump to the end of this shortcut option group.
            self.compiler_listener.emit(
                Emit::from_op_code(OpCode::JumpTo)
                    .with_source_from_token(shortcut.stop().deref())
                    .with_operand(end_of_group_label.clone()),
            );
        }

        // We made it to the end! Mark the end of the group, so we can jump to it
        let current_node = self.compiler_listener.current_node.as_mut().unwrap();
        current_node
            .labels
            .insert(end_of_group_label, current_node.instructions.len() as i32);
        self.compiler_listener
            .emit(Emit::from_op_code(OpCode::Pop).with_source_from_token(token.deref()));
    }

    /// (expression)
    fn visit_expParens(&mut self, ctx: &ExpParensContext<'input>) -> Self::Return {
        self.visit(ctx.expression().unwrap().as_ref())
    }

    /// -expression
    fn visit_expNegative(&mut self, ctx: &ExpNegativeContext<'input>) -> Self::Return {
        let operator_token = ctx.op.as_ref().unwrap();
        let r#type = self.compiler_listener.types.get(ctx).unwrap().clone();
        let expressions = vec![ctx.expression().unwrap() as Rc<ActualParserContext<'input>>];
        self.generate_code_for_operation(
            Operator::UnarySubtract,
            operator_token.deref(),
            &r#type,
            &expressions,
        )
    }

    /// [sic] (not NOT !)expression
    fn visit_expNot(&mut self, ctx: &ExpNotContext<'input>) -> Self::Return {
        let operator_token = ctx.op.as_ref().unwrap();
        let r#type = self.compiler_listener.types.get(ctx).unwrap().clone();
        let expressions = vec![ctx.expression().unwrap() as Rc<ActualParserContext<'input>>];
        self.generate_code_for_operation(
            Operator::Not,
            operator_token.deref(),
            &r#type,
            &expressions,
        )
    }

    /// Variable
    fn visit_expValue(&mut self, ctx: &ExpValueContext<'input>) -> Self::Return {
        self.visit(ctx.value().unwrap().as_ref())
    }

    /// * / %
    fn visit_expMultDivMod(&mut self, ctx: &ExpMultDivModContext<'input>) -> Self::Return {
        let operator_token = ctx.op.as_ref().unwrap();
        let operator = Self::token_to_operator(operator_token.get_token_type()).unwrap();
        let r#type = self.compiler_listener.types.get(ctx).unwrap().clone();
        let expressions = vec![
            ctx.expression(0).unwrap() as Rc<ActualParserContext<'input>>,
            ctx.expression(1).unwrap(),
        ];
        self.generate_code_for_operation(operator, operator_token.deref(), &r#type, &expressions)
    }

    /// + -
    fn visit_expAddSub(&mut self, ctx: &ExpAddSubContext<'input>) -> Self::Return {
        let operator_token = ctx.op.as_ref().unwrap();
        let operator = Self::token_to_operator(operator_token.get_token_type()).unwrap();
        let r#type = self.compiler_listener.types.get(ctx).unwrap().clone();
        let expressions = vec![
            ctx.expression(0).unwrap() as Rc<ActualParserContext<'input>>,
            ctx.expression(1).unwrap(),
        ];
        self.generate_code_for_operation(operator, operator_token.deref(), &r#type, &expressions)
    }

    /// < <= > >=
    fn visit_expComparison(&mut self, ctx: &ExpComparisonContext<'input>) -> Self::Return {
        let operator_token = ctx.op.as_ref().unwrap();
        let operator = Self::token_to_operator(operator_token.get_token_type()).unwrap();
        let r#type = self.compiler_listener.types.get(ctx).unwrap().clone();
        let expressions = vec![
            ctx.expression(0).unwrap() as Rc<ActualParserContext<'input>>,
            ctx.expression(1).unwrap(),
        ];
        self.generate_code_for_operation(operator, operator_token.deref(), &r#type, &expressions)
    }

    /// == !=
    fn visit_expEquality(&mut self, ctx: &ExpEqualityContext<'input>) -> Self::Return {
        let operator_token = ctx.op.as_ref().unwrap();
        let operator = Self::token_to_operator(operator_token.get_token_type()).unwrap();
        let r#type = self.compiler_listener.types.get(ctx).unwrap().clone();
        let expressions = vec![
            ctx.expression(0).unwrap() as Rc<ActualParserContext<'input>>,
            ctx.expression(1).unwrap(),
        ];
        self.generate_code_for_operation(operator, operator_token.deref(), &r#type, &expressions)
    }

    /// and && or || xor ^
    fn visit_expAndOrXor(&mut self, ctx: &ExpAndOrXorContext<'input>) -> Self::Return {
        let operator_token = ctx.op.as_ref().unwrap();
        let operator = Self::token_to_operator(operator_token.get_token_type()).unwrap();
        let r#type = self.compiler_listener.types.get(ctx).unwrap().clone();
        let expressions = vec![
            ctx.expression(0).unwrap() as Rc<ActualParserContext<'input>>,
            ctx.expression(1).unwrap(),
        ];
        self.generate_code_for_operation(operator, operator_token.deref(), &r#type, &expressions)
    }

    fn visit_valueVar(&mut self, ctx: &ValueVarContext<'input>) -> Self::Return {
        self.visit(ctx.variable().unwrap().as_ref())
    }

    fn visit_valueNumber(&mut self, ctx: &ValueNumberContext<'input>) -> Self::Return {
        let number: f32 = ctx.NUMBER().unwrap().get_text().parse().unwrap();
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushFloat)
                .with_source_from_token(ctx.start().deref())
                .with_operand(number),
        )
    }

    fn visit_valueTrue(&mut self, ctx: &ValueTrueContext<'input>) -> Self::Return {
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushBool)
                .with_source_from_token(ctx.start().deref())
                .with_operand(true),
        )
    }

    fn visit_valueFalse(&mut self, ctx: &ValueFalseContext<'input>) -> Self::Return {
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushBool)
                .with_source_from_token(ctx.start().deref())
                .with_operand(false),
        )
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        let variable_name = ctx.VAR_ID().unwrap().get_text();
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushVariable)
                .with_source_from_token(ctx.start().deref())
                .with_operand(variable_name),
        )
    }

    fn visit_valueString(&mut self, ctx: &ValueStringContext<'input>) -> Self::Return {
        // [sic] stripping the " off the front and back actually is this what we want?
        let string_value = ctx
            .STRING()
            .unwrap()
            .get_text()
            .trim_matches('"')
            .to_owned();
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushString)
                .with_source_from_token(ctx.start().deref())
                .with_operand(string_value),
        )
    }

    /// all we need do is visit the function itself, it will handle everything
    fn visit_valueFunc(&mut self, ctx: &ValueFuncContext<'input>) -> Self::Return {
        self.visit(ctx.function_call().unwrap().as_ref())
    }

    /// null value
    fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>) -> Self::Return {
        self.compiler_listener
            .emit(Emit::from_op_code(OpCode::PushNull).with_source_from_token(ctx.start().deref()))
    }

    fn visit_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) -> Self::Return {
        // Declare statements do not participate in code generation
    }

    /// A <<jump>> command, which immediately jumps to another node, given its name.
    fn visit_jumpToNodeName(&mut self, ctx: &JumpToNodeNameContext<'input>) -> Self::Return {
        if let Some(tracking_enabled) = self.tracking_enabled.clone() {
            Self::generate_tracking_code(self.compiler_listener, tracking_enabled);
        }
        let destination = ctx.destination.as_ref().unwrap();
        self.compiler_listener.emit(
            Emit::from_op_code(OpCode::PushString)
                .with_source_from_token(destination.deref())
                .with_operand(destination.get_text().to_owned()),
        );
        self.compiler_listener
            .emit(Emit::from_op_code(OpCode::RunNode).with_source_from_token(ctx.start().deref()))
    }

    /// A <<jump>> command, which immediately jumps to another node, given an
    /// expression that resolves to a node's name.
    fn visit_jumpToExpression(&mut self, ctx: &JumpToExpressionContext<'input>) -> Self::Return {
        if let Some(tracking_enabled) = self.tracking_enabled.clone() {
            Self::generate_tracking_code(self.compiler_listener, tracking_enabled);
        }
        // Evaluate the expression, and jump to the result on the stack.
        self.visit(ctx.expression().unwrap().as_ref());
        self.compiler_listener
            .emit(Emit::from_op_code(OpCode::RunNode).with_source_from_token(ctx.start().deref()))
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
