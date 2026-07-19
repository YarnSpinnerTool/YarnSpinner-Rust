//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/CodeGenerationVisitor.cs>

use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use antlr4rust::parser_rule_context::ParserRuleContext;
use antlr4rust::token::Token;
use antlr4rust::tree::{ParseTree, ParseTreeVisitorCompat, Tree};
use std::ops::Deref;
use std::rc::Rc;
use antlr4rust::parser::ParserNodeType;
use yarnspinner_compiler_macros::emit;
use yarnspinner_core::prelude::*;
use yarnspinner_core::types::Type;
use crate::listeners::CompilerListener;

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
    pub(crate) fn token_to_operator(token: i32) -> Option<Operator> {
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
    pub(crate) fn generate_tracking_code(compiler: &mut NodeBuilder, variable_name: String) {
        // pushing the var and the increment onto the stack

        emit! {
            compiler;
            PushVariableInstruction { variable_name: variable_name.clone() },
            PushFloatInstruction { value: 1. },
            // Indicate that we are pushing this many items for comparison
            PushFloatInstruction { value: 2. },
            // calling the function
            CallFunctionInstruction { function_name: "Number.Add".to_owned() },
            // now store the variable and clean up the stack
            StoreVariableInstruction { variable_name },
            PopInstruction
        };
    }

    fn emit_function_call(&mut self, fn_name: String, args: usize) -> usize {
        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            PushFloatInstruction { value: args as f32 },
            CallFunctionInstruction { function_name: fn_name }
        }
    }

    fn emit_jump_to_named_node(
        &mut self,
        name: String,
        detour: bool,
    ) -> usize {
        let compiler = self.compiler_listener.node_builder.as_mut().unwrap();
        if detour {
            emit! {
                compiler;
                DetourToNodeInstruction {
                    node_name: name
                }
            };
        } else {
            emit! {
                compiler;
                RunNodeInstruction {
                    node_name: name
                }
            };
        }
        1
    }

    fn emit_jump_to_expression(
        &mut self,
        expr: &ExpressionContextAll<'input>,
        detour: bool,
    ) {
        self.visit(expr);

        let compiler = self.compiler_listener.node_builder.as_mut().unwrap();
        if detour {
            emit! {
                compiler;
                PeekAndDetourToNode
            };
        } else {
            emit! {
                compiler;
                PeekAndRunNodeInstruction
            };
        }
    }
}

impl<'a, 'input: 'a> ParseTreeVisitorCompat<'input> for CodeGenerationVisitor<'a, 'input> {
    type Node = YarnSpinnerParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

#[allow(non_snake_case)]
impl<'a, 'input: 'a> YarnSpinnerParserVisitorCompat<'input> for CodeGenerationVisitor<'a, 'input> {
    /// a regular ol' line of text
    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) -> Self::Return {
        // Evaluate the inline expressions and push the results onto the
        // stack.
        let line_id = get_line_id(ctx);

        let mut jump_over = Vec::new();

        let (cond, _) = self.evaluate_line_condition(ctx, line_id.clone());

        if cond {
            emit! {
                self.compiler_listener.node_builder.as_mut().unwrap();
                push jump_over = @JumpIfFalseInstruction
            };
        }

        let formatted_text = ctx.line_formatted_text().unwrap();
        let expression_count =
            self.generate_code_for_expressions_in_formatted_text(formatted_text.get_children());

        let compiler = self.compiler_listener.node_builder.as_mut().unwrap();

        emit! {
            compiler;
            RunLineInstruction {
                line_id: line_id.to_string(),
                substitution_count: expression_count as i32
            }
        };

        if ctx.line_condition().is_some() {
            emit! {
                compiler;
                PopInstruction
            };
        }

        for jump in jump_over {
            compiler.set_destination_at(jump, compiler.next_address()).unwrap();
        }
    }

    /// (expression)
    fn visit_expParens(&mut self, ctx: &ExpParensContext<'input>) -> Self::Return {
        self.visit(ctx.expression().unwrap().as_ref())
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

    fn visit_valueNumber(&mut self, ctx: &ValueNumberContext<'input>) -> Self::Return {
        let number: f32 = ctx.NUMBER().unwrap().get_text().parse().unwrap();

        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            PushFloatInstruction { value: number }
        };
    }

    fn visit_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) -> Self::Return {
        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            PushBoolInstruction { value: true }
        };
    }

    fn visit_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) -> Self::Return {
        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            PushBoolInstruction { value: false }
        };
    }

    fn visit_valueVar(&mut self, ctx: &ValueVarContext<'input>) -> Self::Return {
        self.visit(ctx.variable().unwrap().as_ref())
    }

    fn visit_valueString(&mut self, ctx: &ValueStringContext<'input>) -> Self::Return {
        // [sic] stripping the " off the front and back actually is this what we want?
        let string_value = ctx
            .STRING()
            .unwrap()
            .get_text()
            .trim_matches('"')
            .to_owned();

        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            PushStringInstruction { value: string_value }
        };
    }

    /// all we need do is visit the function itself, it will handle everything
    fn visit_valueFunc(&mut self, ctx: &ValueFuncContext<'input>) -> Self::Return {
        self.visit(ctx.function_call().unwrap().as_ref())
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        let variable_name = ctx.VAR_ID().unwrap().get_text();

        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            PushVariableInstruction { variable_name }
        };
    }

    /// handles emitting the correct instructions for the function
    fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) -> Self::Return {
        // generate the instructions for all of the parameters
        let expressions = ctx.expression_all();
        for parameter in &expressions {
            self.visit(parameter.as_ref());
        }

        let function_name = ctx.FUNC_ID().unwrap().get_text();

        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            // push the number of parameters onto the stack
            PushFloatInstruction { value: expressions.len() as f32 },
            // then call the function itself
            CallFunctionInstruction { function_name }
        };
    }

    /// if statement ifclause (elseifclause)* (elseclause)? <<endif>>
    fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) -> Self::Return {
        // Implementation note: Idk what this is supposed to do. Looks like a noop.
        // context.AddErrorNode(null);

        // label to give us a jump point for when the if finishes
        let mut end_of_if_statements = Vec::new();

        // handle the if
        let if_clause = ctx.if_clause().unwrap();
        self.generate_code_for_clause(
            &mut end_of_if_statements,
            if_clause.as_ref(),
            &if_clause.statement_all(),
            if_clause.expression().unwrap(),
        );

        // all elseifs
        for else_if_clause in &ctx.else_if_clause_all() {
            self.generate_code_for_clause(
                &mut end_of_if_statements,
                else_if_clause.as_ref(),
                &else_if_clause.statement_all(),
                else_if_clause.expression().unwrap(),
            );
        }

        // the else, if there is one
        if let Some(else_clause) = ctx.else_clause() {
            self.generate_code_for_clause(
                &mut end_of_if_statements,
                else_clause.as_ref(),
                &else_clause.statement_all(),
                None,
            );
        }

        let compiler = self.compiler_listener.node_builder.as_mut().unwrap();
        for mut jump in end_of_if_statements {
            jump.set(JumpToInstruction { destination: compiler.next_address().try_into().unwrap() });
            compiler.set(jump);
        }
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

            self.visit(expression.clone().as_ref());

            self.generate_code_for_operation(
                op,
                operator_token.as_ref(),
                &r#type,
                &vec![variable.clone(), expression.clone()],
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

        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            StoreVariableInstruction { variable_name },
            PopInstruction
        };
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

        let compiler = self.compiler_listener.node_builder.as_mut().unwrap();

        // [sic] TODO: look into replacing this as it seems a bit odd
        match composed_string.as_str() {
            "stop" => {
                // "stop" is a special command that immediately stops
                // execution
                emit! {
                    compiler;
                    StopInstruction
                };
            }
            "return" => {
                // "return" is a special command that immediately returns from node
                emit! {
                    compiler;
                    ReturnInstruction
                };
            }
            _ => {
                emit! {
                    compiler;
                    RunCommandInstruction {
                        command_text: composed_string,
                        substitution_count: expression_count as i32
                    }
                };
            }
        }
    }

    /// for the shortcut options (-> line of text <<if expression>> indent statements dedent)+
    fn visit_shortcut_option_statement(
        &mut self,
        ctx: &Shortcut_option_statementContext<'input>,
    ) -> Self::Return {
        let mut options = Vec::new();
        let mut once_vars = Vec::new();

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

            let line_statement = shortcut.line_statement().unwrap();

            // Get the line ID from the hashtags if it has one
            let line_id = get_line_id(&line_statement);

            let (has_line_condition, once_name) = self.evaluate_line_condition(line_statement.deref(), line_id.clone().into());
            once_vars.push(once_name);

            // We can now prepare and add the option.

            // Start by figuring out the text that we want to add. This will
            // involve evaluating any inline expressions.
            let expression_count = self.generate_code_for_expressions_in_formatted_text(
                line_statement.line_formatted_text().unwrap().get_children(),
            );

            emit! {
                self.compiler_listener.node_builder.as_mut().unwrap();
                // And add this option to the list.
                push options = @AddOptionInstruction {
                    line_id: line_id.to_string(),
                    destination: -1,
                    substitution_count: expression_count as i32,
                    has_condition: has_line_condition
                }
            };
        }

        let token = ctx.stop();

        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            // All the options that we intend to show are now ready to go.
            ShowOptionsInstruction,
            // The top of the stack now contains the name of the label we want
            // to jump to. Jump to it now.
            PeekAndJumpInstruction
        };

        let mut groupEndJumps = Vec::new();

        // We'll now emit the labels and code associated with each option.
        for (option_count, shortcut) in ctx.shortcut_option_all().into_iter().enumerate() {
            // Emit the label for this option's codex

            let option = options[option_count];
            let compiler = self.compiler_listener.node_builder.as_mut().unwrap();
            let addr = compiler.next_address();
            compiler.set_destination_at(option, addr).unwrap();

            // If a "once" variable was set, the emit a store instruction setting the variable to true.
            if let Some(once_var) = once_vars[option_count].as_ref() {
                emit! {
                    self.compiler_listener.node_builder.as_mut().unwrap();
                    PushBoolInstruction { value: true },
                    StoreVariableInstruction { variable_name: once_var.clone() }
                };
            }

            // Run through all the children statements of the shortcut option
            for child in shortcut.statement_all() {
                self.visit(child.as_ref());
            }

            emit! {
                self.compiler_listener.node_builder.as_mut().unwrap();
                push groupEndJumps = @JumpToInstruction { destination: -1 }
            };
        }

        let compiler = self.compiler_listener.node_builder.as_mut().unwrap();

        // We made it to the end! Mark the end of the group, so we can jump to it
        for pos in groupEndJumps {
            let nr = compiler.next_address();
            compiler.set_destination_at(pos, nr).unwrap();
        }

        emit! {
                compiler;
                PopInstruction
        };
    }

    fn visit_line_group_statement(
        &mut self,
        ctx: &Line_group_statementContext<'input>,
    ) -> Self::Return {
        todo!()
    }

    fn visit_line_group_item(&mut self, ctx: &Line_group_itemContext<'input>) -> Self::Return {
        todo!()
    }

    fn visit_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) -> Self::Return {
        // Declare statements do not participate in code generation
    }

    /// A <<jump>> command, which immediately jumps to another node, given its name.
    fn visit_jumpToNodeName(&mut self, ctx: &JumpToNodeNameContext<'input>) -> Self::Return {
        if let Some(tracking_enabled) = self.tracking_enabled.clone() {
            Self::generate_tracking_code(self.compiler_listener.node_builder.as_mut().unwrap(), tracking_enabled);
        }

        let destination = ctx.destination.as_ref().unwrap_or_bug();
        self.emit_jump_to_named_node(
            destination.get_text().into(),
            false,
        );
    }

    /// A <<jump>> command, which immediately jumps to another node, given an
    /// expression that resolves to a node's name.
    fn visit_jumpToExpression(&mut self, ctx: &JumpToExpressionContext<'input>) -> Self::Return {
        if let Some(tracking_enabled) = self.tracking_enabled.clone() {
            Self::generate_tracking_code(self.compiler_listener.node_builder.as_mut().unwrap(), tracking_enabled);
        }

        let expr = ctx.expression().unwrap_or_bug();
        let expr = expr.as_ref();
        self.emit_jump_to_expression(expr, false)
    }

    /// A <<detour>> command, which immediately jumps to another node, given its name.
    fn visit_detourToNodeName(&mut self, ctx: &DetourToNodeNameContext<'input>) -> Self::Return {
        if let Some(tracking_enabled) = self.tracking_enabled.clone() {
            Self::generate_tracking_code(self.compiler_listener.node_builder.as_mut().unwrap(), tracking_enabled);
        }

        let destination = ctx.destination.as_ref().unwrap();
        self.emit_jump_to_named_node(destination.get_text().to_owned(), true);
    }

    /// A <<detour>> command, which immediately jumps to another node, given an
    /// expression that resolves to a node's name.
    fn visit_detourToExpression(
        &mut self,
        ctx: &DetourToExpressionContext<'input>,
    ) -> Self::Return {
        if let Some(tracking_enabled) = self.tracking_enabled.clone() {
            Self::generate_tracking_code(self.compiler_listener.node_builder.as_mut().unwrap(), tracking_enabled);
        }

        let expr = ctx.expression().unwrap_or_bug();
        let expr = expr.as_ref();
        self.emit_jump_to_expression(expr, false)
    }

    /// A <<return>> command, which immediately returns from a detour or exits a dialogue.
    fn visit_return_statement(&mut self, ctx: &Return_statementContext<'input>) -> Self::Return {
        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            ReturnInstruction
        };
    }
}

impl<'a, 'input: 'a> CodeGenerationVisitor<'a, 'input> {
    fn evaluate_line_condition(&mut self, ctx: &Line_statementContext<'input>, line_id: LineId) -> (bool, Option<String>) {
        let line_visited = self.compiler_listener.get_content_viewed_variable_name(line_id);

        match ctx.line_condition().as_ref().map(|c| c.deref()) {
            Some(Line_conditionContextAll::LineOnceConditionContext(once)) => {
                // Test to see if the 'once' variable for this content is
                // false

                emit! {
                    self.compiler_listener.node_builder.as_mut().unwrap();
                    PushVariableInstruction { variable_name: line_visited.clone() },
                };
                self.emit_operation(Operator::Not, &Type::Boolean, 1).unwrap();

                // If the condition has an expression, evaluate that too
                // and 'and' it with the 'once' test we just evaluated
                if let Some(expr) = once.expression() {
                    self.visit(once);
                    self.emit_operation(Operator::And, &Type::Boolean, 2).unwrap();
                }
                (true, Some(line_visited))
            }
            Some(Line_conditionContextAll::LineConditionContext(normal)) => {
                self.visit(normal);
                (true, None)
            },
            _ => (false, None)
        }
    }
}

impl<'a, 'input: 'a> CodeGenerationVisitor<'a, 'input> {
    fn generate_jump(&mut self, to: i32) -> usize {
        emit! {
            self.compiler_listener.node_builder.as_mut().unwrap();
            JumpToInstruction { destination: to }
        }
    }

    fn generate_function_call(
        &mut self,
        fn_name: String,
        args: &[Rc<StatementContext<'input>>],
    ) -> usize {
        for arg in args {
            self.visit(arg.deref());
        }

        let emitted = self.emit_function_call(fn_name, args.len());
        emitted + 2
    }

    fn generate_static_function_call(&mut self, fn_name: String, args: Vec<OperandValue>) -> usize {
        let builder = self.compiler_listener.node_builder.as_mut().unwrap();
        let mut emitted = 0;
        for arg in &args {
            emitted += match arg {
                OperandValue::StringValue(v) => {
                    emit! {
                        builder;
                        PushStringInstruction { value: v.to_owned() }
                    }
                }
                OperandValue::BoolValue(v) => {
                    emit! {
                        builder;
                        PushBoolInstruction { value: v.to_owned() }
                    }
                }
                OperandValue::FloatValue(v) => {
                    emit! {
                        builder;
                        PushFloatInstruction { value: v.to_owned() }
                    }
                }
            };
        }

        emitted += self.emit_function_call(fn_name, args.len());
        emitted
    }

    fn generate_code_for_random_choice(&mut self, choices: u32) -> usize {
        let emitted = self.generate_static_function_call(
            "random".to_owned(),
            vec![
                OperandValue::FloatValue(1.0),
                OperandValue::FloatValue(choices as f32),
            ],
        );

        let builder = self.compiler_listener.node_builder.as_mut().unwrap();
        let pos = builder.last_address().unwrap();

        emitted + { emit! {
            builder;
            // Add instruction number to choices
            PushFloatInstruction { value: pos.value() as f32 + 4.0 },
            PushFloatInstruction { value: 2.0 },
            CallFunctionInstruction { function_name: "Add".to_owned() },
            // Peek and Jump to instruction
            PeekAndJumpInstruction
        } }
    }

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
    fn emit_operation(
        &mut self,
        op: Operator,
        r#type: &Type,
        operands: usize,
    ) -> Result<(), Diagnostic> {
        let method_name = op.to_string();
        let has_method = r#type.has_method(&method_name);

        if !has_method {
            let msg = format!("Codegen failed to get implementation type for {} given input type {}.",
                              op,
                              r#type.name());
            return Err(Diagnostic::from_message(msg));
        }

        // Figure out the canonical name for the method that the VM should
        // invoke in order to perform this work
        assert!(
            has_method,
            "Codegen failed to get implementation type for {} given input type {}.",
            op,
            r#type.name(),
        );
        let function_name = r#type.get_canonical_name_for_method(&method_name);
        self.emit_function_call(function_name, operands);
        Ok(())
    }

    /// Emits code that calls a method appropriate for the operator
    fn generate_code_for_operation(
        &mut self,
        op: Operator,
        operator_token: &impl Token,
        r#type: &Type,
        operands: &Vec<Rc<<<Self as ParseTreeVisitorCompat<'input>>::Node as ParserNodeType<'input>>::Type>>,
    )  {
        // Generate code for each of the operands, so that their value is
        // now on the stack.
        for operand in operands {
            self.visit(operand.as_ref());
        }

        self.emit_operation(op, r#type, operands.len()).unwrap();
    }

    fn generate_code_for_clause(
        &mut self,
        jump_to_end: &mut Vec<InstructionCell>,
        ctx: &impl ParserRuleContext<'input>,
        children: &Vec<Rc<StatementContext<'input>>>,
        expression: impl Into<Option<Rc<ExpressionContextAll<'input>>>>,
    ) {
        let expression = expression.into();
        let mut jump_to_end_of_clause = None;

        // handling the expression (if it has one) will only be called on ifs and elseifs
        if let Some(expression) = expression.clone() {
            // Code-generate the expression
            self.visit(expression.as_ref());

            emit! {
                self.compiler_listener.node_builder.as_mut().unwrap();
                into jump_to_end_of_clause = @JumpIfFalseInstruction
            };
        }

        // running through all of the children statements
        for child in children {
            self.visit(child.as_ref());
        }

        let builder = self.compiler_listener.node_builder.as_mut().unwrap();

        emit! {
            builder;
            push &jump_to_end = JumpToInstruction { destination: -1 }
        };

        if let Some(clause) = jump_to_end_of_clause {
            builder.set_destination_at(clause, builder.next_address()).unwrap();
        }

        if let Some(expression) = expression {
            emit! {
                builder;
                PopInstruction
            };
        } else {
            assert!(jump_to_end_of_clause.is_none());
        }
    }
}
