//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/TypeCheckVisitor.cs>

use crate::parser_rule_context_ext::ParserRuleContextExt;
use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use crate::visitors::{CodeGenerationVisitor, KnownTypes};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use check_operation::*;
use std::path::Path;
use yarnspinner_core::prelude::*;
use yarnspinner_core::types::*;

mod check_operation;

/// A visitor that walks the parse tree, checking for type consistency
/// in expressions. Existing type information is provided via the
/// [`existing_declarations`] property. This visitor will also
/// attempt to infer the type of variables that don't have an explicit
/// declaration; for each of these, a new Declaration will be created
/// and made available via the [`new_declaration`] property.
pub(crate) struct TypeCheckVisitor<'input> {
    /// <summary>
    /// Gets the collection of all declarations - both the ones we received
    /// at the start, and the new ones we've derived ourselves.
    /// </summary>
    pub(crate) diagnostics: Vec<Diagnostic>,

    /// Gets the collection of new variable declarations that were
    /// found as a result of using this  [`TypeCheckVisitor`] to visit a [`ParserRuleContext`].
    pub(crate) new_declarations: Vec<Declaration>,

    // the list of variables we aren't actually sure about
    pub(crate) deferred_types: Vec<DeferredTypeDiagnostic>,

    // The collection of variable declarations we know about before
    // starting our work
    existing_declarations: Vec<Declaration>,

    // The name of the node that we're currently visiting.
    current_node_name: Option<String>,

    /// The type that this expression has been
    /// determined to be by a [`TypeCheckVisitor`]
    /// object.
    ///
    /// ## Implementation notes
    ///
    /// In the original implementation, this was implemented
    /// on the [`ValueContext`] directly using a `partial`.
    ///
    /// Careful, the original class has an unrelated member called `types`,
    /// but in this implementation, we replaced that member by [`Type::EXPLICITLY_CONSTRUCTABLE`].
    pub(crate) known_types: KnownTypes,

    /// A type hint for the expression.
    /// This is mostly used by [`TypeCheckVisitor`]
    /// to give a hint that can be used by functions to
    /// influence their type when set to use inference.
    /// Won't be used if a concrete type is already known.
    ///
    /// ## Implementation notes
    ///
    /// In the original implementation, this was implemented
    /// on the [`ValueContext`] directly using a `partial`
    hints: KnownTypes,

    file: FileParseResult<'input>,
    _dummy: Option<Type>,
}

impl<'input> TypeCheckVisitor<'input> {
    pub(crate) fn new(
        existing_declarations: Vec<Declaration>,
        file: FileParseResult<'input>,
    ) -> Self {
        Self {
            file,
            existing_declarations,
            diagnostics: Default::default(),
            new_declarations: Default::default(),
            deferred_types: Default::default(),
            current_node_name: Default::default(),
            known_types: Default::default(),
            hints: Default::default(),
            _dummy: Default::default(),
        }
    }

    /// Gets the collection of all declarations - both the ones we received
    /// at the start, and the new ones we've derived ourselves.
    pub(crate) fn declarations(&self) -> impl Iterator<Item = &Declaration> + '_ {
        self.existing_declarations
            .iter()
            .chain(self.new_declarations.iter())
    }

    /// Gets the collection of all declarations mutably - both the ones we received
    /// at the start, and the new ones we've derived ourselves.
    pub(crate) fn declarations_mut(&mut self) -> impl Iterator<Item = &mut Declaration> + '_ {
        self.existing_declarations
            .iter_mut()
            .chain(self.new_declarations.iter_mut())
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for TypeCheckVisitor<'input> {
    type Node = YarnSpinnerParserContextType;

    type Return = Option<Type>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for TypeCheckVisitor<'input> {
    fn visit_node(&mut self, ctx: &NodeContext<'input>) -> Self::Return {
        for header in ctx.header_all() {
            let key = header.header_key.as_ref().unwrap().get_text();
            if key == "title" {
                let value = header.header_value.as_ref().unwrap().get_text();
                self.current_node_name = Some(value.to_owned());
            }
        }
        if let Some(body) = ctx.body() {
            self.visit(body.as_ref());
        }
        None
    }

    fn visit_expParens(&mut self, ctx: &ExpParensContext<'input>) -> Self::Return {
        // Parens expressions have the type of their inner expression
        let r#type = self.visit(ctx.expression().unwrap().as_ref());
        self.known_types.insert(ctx, r#type.clone());
        r#type
    }

    fn visit_expMultDivMod(&mut self, ctx: &ExpMultDivModContext<'input>) -> Self::Return {
        let expressions: Vec<_> = ctx.expression_all().into_iter().map(|e| e.into()).collect();
        let op = ctx.op.as_ref().unwrap();
        let operator = CodeGenerationVisitor::token_to_operator(op.token_type);
        // *, /, % all support numbers only
        // ## Implementation notes
        // The original passes no permitted types, but judging by the comment above, this seems like a bug
        let r#type =
            self.check_operation(ctx, &expressions, operator, op.get_text(), &[Type::Number]);
        self.known_types.insert(ctx, r#type.clone());
        r#type
    }

    fn visit_expComparison(&mut self, ctx: &ExpComparisonContext<'input>) -> Self::Return {
        let expressions: Vec<_> = ctx.expression_all().into_iter().map(|e| e.into()).collect();
        let op = ctx.op.as_ref().unwrap();
        let operator = CodeGenerationVisitor::token_to_operator(op.token_type);
        let r#type = self.check_operation(ctx, &expressions, operator, op.get_text(), &[]);
        self.known_types.insert(ctx, r#type);
        // Comparisons always return bool
        Some(Type::Boolean)
    }

    fn visit_expNegative(&mut self, ctx: &ExpNegativeContext<'input>) -> Self::Return {
        let expressions = &[ctx.expression().unwrap().into()];
        let op = ctx.op.as_ref().unwrap();
        let operator = CodeGenerationVisitor::token_to_operator(op.token_type);
        let r#type = self.check_operation(ctx, expressions, operator, op.get_text(), &[]);
        self.known_types.insert(ctx, r#type.clone());
        r#type
    }

    fn visit_expAndOrXor(&mut self, ctx: &ExpAndOrXorContext<'input>) -> Self::Return {
        let expressions: Vec<_> = ctx.expression_all().into_iter().map(Term::from).collect();
        let operator_context = ctx.op.as_ref().unwrap();
        let operator =
            CodeGenerationVisitor::token_to_operator(operator_context.token_type).unwrap();
        let description = operator_context.get_text();
        let r#type = self.check_operation(ctx, &expressions, operator, description, &[]);
        self.known_types.insert(ctx, r#type.clone());
        r#type
    }

    fn visit_expAddSub(&mut self, ctx: &ExpAddSubContext<'input>) -> Self::Return {
        let expressions: Vec<_> = ctx.expression_all().into_iter().map(|e| e.into()).collect();
        let op = ctx.op.as_ref().unwrap();
        let operator = CodeGenerationVisitor::token_to_operator(op.token_type);
        let r#type = self.check_operation(ctx, &expressions, operator, op.get_text(), &[]);
        self.known_types.insert(ctx, r#type.clone());
        r#type
    }

    fn visit_expNot(&mut self, ctx: &ExpNotContext<'input>) -> Self::Return {
        let expressions = &[ctx.expression().unwrap().into()];
        let op = ctx.op.as_ref().unwrap();
        let operator = CodeGenerationVisitor::token_to_operator(op.token_type);
        // ! supports only bool types
        // ## Implementation notes
        // The original passes no permitted types, but judging by the comment above, this seems like a bug
        let r#type =
            self.check_operation(ctx, expressions, operator, op.get_text(), &[Type::Boolean]);
        self.known_types.insert(ctx, r#type.clone());
        r#type
    }

    fn visit_expValue(&mut self, ctx: &ExpValueContext<'input>) -> Self::Return {
        // passing the hint from the expression down into the values within
        let hint = self.hints.get(ctx).cloned();
        let value = ctx.value().unwrap();
        self.hints.insert(value.as_ref(), hint);
        // Value expressions have the type of their inner value
        let r#type = self.visit(value.as_ref());
        self.known_types.insert(ctx, r#type.clone());
        r#type
    }

    fn visit_expEquality(&mut self, ctx: &ExpEqualityContext<'input>) -> Self::Return {
        let expressions: Vec<_> = ctx.expression_all().into_iter().map(|e| e.into()).collect();
        let op = ctx.op.as_ref().unwrap();
        let operator = CodeGenerationVisitor::token_to_operator(op.token_type);
        // == and != support any defined type, as long as terms are the
        // same type
        let r#type = self.check_operation(ctx, &expressions, operator, op.get_text(), &[]);
        self.known_types.insert(ctx, r#type);
        // Equality always returns bool
        Some(Type::Boolean)
    }

    fn visit_valueNumber(&mut self, _ctx: &ValueNumberContext<'input>) -> Self::Return {
        Some(Type::Number)
    }

    fn visit_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) -> Self::Return {
        Some(Type::Boolean)
    }

    fn visit_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) -> Self::Return {
        Some(Type::Boolean)
    }

    fn visit_valueVar(&mut self, ctx: &ValueVarContext<'input>) -> Self::Return {
        let variable = ctx.variable().unwrap();
        self.visit_variable(&variable)
    }

    fn visit_valueString(&mut self, _ctx: &ValueStringContext<'input>) -> Self::Return {
        Some(Type::String)
    }

    fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>) -> Self::Return {
        self.diagnostics.push(
            Diagnostic::from_message("Null is not a permitted type in Yarn Spinner 2.0 and later")
                .with_file_name(&self.file.name)
                .with_parser_context(ctx, self.file.tokens()),
        );

        None
    }

    fn visit_valueFunc(&mut self, ctx: &ValueFuncContext<'input>) -> Self::Return {
        let function_name = ctx
            .function_call()
            .unwrap()
            .get_token(yarnspinnerlexer::FUNC_ID, 0)
            .unwrap()
            .get_text();

        let function_declaration = self
            .declarations()
            .find(|decl| decl.name == function_name)
            .cloned(); // Cloning to avoid borrow checker issues
        let hint = self.hints.get(ctx).cloned();
        let function_type = if let Some(function_declaration) = function_declaration {
            let Type::Function(mut function_type) = function_declaration.r#type.clone() else {
                unreachable!("Internal error: function declaration is not of type Function. This is a bug. Please report it at https://github.com/YarnSpinnerTool/YarnSpinner-Rust/issues/new")
            };

            // we have an existing function but its undefined
            // if we also have a type hint we can use that to update it
            if function_type.return_type.is_none() {
                if let Some(hint) = hint {
                    self.new_declarations.find_remove(&function_declaration);
                    function_type.set_return_type(hint);
                    let new_declaration = Declaration {
                        r#type: Type::from(function_type.clone()),
                        ..function_declaration.clone()
                    };
                    self.new_declarations.push(new_declaration);
                }
            }
            function_type
        } else {
            // We don't have a declaration for this function. Create an
            // implicit one.
            let mut function_type = FunctionType::default();
            // because it is an implicit declaration we will use the type hint to give us a return type
            function_type.set_return_type(hint);

            // Create the array of parameters for this function based
            // on how many we've seen in this call. Set them all to be
            // undefined; we'll bind their type shortly.
            let expressions = ctx.function_call().unwrap().expression_all();
            let parameter_types = expressions.iter().map(|_| None);
            for parameter_type in parameter_types {
                function_type.add_parameter(parameter_type);
            }

            let line = ctx.start().get_line_as_usize();
            let column = ctx.start().get_column_as_usize();
            let function_declaration =
                Declaration::new(function_name.clone(), function_type.clone())
                    .with_description(format!(
                        "Implicit declaration of function at {}:{}:{}",
                        self.file.name, line, column
                    ))
                    .with_range(ctx.range())
                    .with_implicit();
            self.new_declarations.push(function_declaration);
            function_type
        };
        // Check each parameter of the function
        let supplied_parameters = ctx.function_call().unwrap().expression_all();
        let expected_parameter_types = function_type.parameters;

        if supplied_parameters.len() != expected_parameter_types.len() {
            // Wrong number of parameters supplied
            let parameters = if expected_parameter_types.len() == 1 {
                "parameter"
            } else {
                "parameters"
            };
            let diagnostic = Diagnostic::from_message(format!(
                "Function \"{}\" expects {} {}, but received {}",
                function_name,
                expected_parameter_types.len(),
                parameters,
                supplied_parameters.len()
            ))
            .with_file_name(&self.file.name)
            .with_parser_context(ctx, self.file.tokens());
            self.diagnostics.push(diagnostic);
            return *function_type.return_type;
        }

        for (i, (supplied_parameter, mut expected_type)) in supplied_parameters
            .iter()
            .cloned()
            .zip(expected_parameter_types.iter())
            .enumerate()
        {
            let supplied_type = self.visit(supplied_parameter.as_ref());
            if expected_type.is_none() {
                // The type of this parameter hasn't yet been bound.
                // Bind this parameter type to what we've resolved the
                // type to.
                let declaration = self
                    .declarations_mut()
                    .find(|decl| decl.name == function_name)
                    .unwrap(); // Guaranteed to be Some
                let Type::Function(function_type) = &mut declaration.r#type else {
                    unreachable!();
                };
                function_type.parameters[i].clone_from(&supplied_type);
                expected_type = &supplied_type;
            }
            if !supplied_type.is_sub_type_of(expected_type) {
                let diagnostic = Diagnostic::from_message(format!(
                    "{} parameter {} expects a {}, not a {}",
                    function_name,
                    i + 1,
                    expected_type.format(),
                    supplied_type.format()
                ))
                .with_file_name(&self.file.name)
                .with_parser_context(ctx, self.file.tokens());
                self.diagnostics.push(diagnostic);
                return *function_type.return_type;
            }
        }
        // Cool, all the parameters check out!

        // Finally, return the return type of this function.
        *function_type.return_type
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        // The type of the value depends on the declared type of the
        // variable
        // the parser will have generated an error for us in an
        // earlier stage if we don't have a variable name for
        // this Variable context; here, we'll bail out.
        let var_id = ctx.get_token(yarnspinnerlexer::VAR_ID, 0)?;
        let name = var_id.get_text();
        if let Some(declaration) = self.declarations().find(|decl| decl.name == name) {
            return Some(declaration.r#type.clone());
        }

        // do we already have a potential warning about this?
        // no need to make more
        if self
            .deferred_types
            .iter()
            .any(|deferred_type| deferred_type.name == name)
        {
            return None;
        }

        // creating a new diagnostic for us having an undefined variable
        // this won't get added into the existing diags though because its possible a later pass will clear it up
        // so we save this as a potential diagnostic for the compiler itself to resolve
        let diagnostic =
            Diagnostic::from_message(format_cannot_determine_variable_type_error(&name))
                .with_file_name(&self.file.name)
                .with_parser_context(ctx, self.file.tokens());
        self.deferred_types
            .push(DeferredTypeDiagnostic { name, diagnostic });

        // We don't have a declaration for this variable. Return
        // Undefined. Hopefully, other context will allow us to infer a
        // type.
        None
    }

    fn visit_if_clause(&mut self, ctx: &If_clauseContext<'input>) -> Self::Return {
        ParseTreeVisitorCompat::visit_children(self, ctx);
        // If clauses are required to be boolean
        let expressions = &[ctx.expression().unwrap().into()];
        self.check_operation(ctx, expressions, None, "if statement", &[Type::Boolean])
    }

    fn visit_else_if_clause(&mut self, ctx: &Else_if_clauseContext<'input>) -> Self::Return {
        ParseTreeVisitorCompat::visit_children(self, ctx);
        // Else if clauses are required to be boolean
        let expressions = &[ctx.expression().unwrap().into()];
        self.check_operation(ctx, expressions, None, "elseif statement", &[Type::Boolean])
    }

    fn visit_set_statement(&mut self, ctx: &Set_statementContext<'input>) -> Self::Return {
        let variable_context = ctx.variable()?;
        let expression_context = ctx.expression()?;
        let variable_type = self.visit(variable_context.as_ref());
        if let Some(variable_type) = variable_type.as_ref() {
            // giving the expression a hint just in case it is needed to help resolve any ambiguity on the expression
            // currently this is only useful in situations where we have a function as the rvalue of a known lvalue
            self.hints
                .insert(expression_context.as_ref(), variable_type.clone());
        }
        let mut expression_type = self.visit(expression_context.as_ref());
        let variable_name = variable_context.get_text();
        let terms: &[Term] = &[
            variable_context.clone().into(),
            expression_context.clone().into(),
        ];

        let op = ctx.op.as_ref().unwrap();
        match op.token_type {
            yarnspinnerlexer::OPERATOR_ASSIGNMENT => {
                // Straight assignment supports any assignment, as long
                // as it's consistent; we already know the type of the
                // expression, so let's check to see if it's assignable
                // to the type of the variable.
                match (variable_type.as_ref(), expression_type.as_ref()) {
                    (Some(variable_type), _) if !expression_type.is_sub_type_of(variable_type) => {
                        let diagnostic = Diagnostic::from_message(format!(
                            "{variable_name} ({}) cannot be assigned a {}",
                            variable_type.format(),
                            expression_type.format(),
                        ))
                        .with_file_name(&self.file.name)
                        .with_parser_context(ctx, self.file.tokens());
                        self.diagnostics.push(diagnostic);
                    }
                    (None, Some(expression_type)) => {
                        // This variable was undefined, but we have a
                        // defined type for the value it was set to. Create
                        // an implicit declaration for the variable!

                        // Attempt to get a default value for the given type. If
                        // we can't get one, we can't create the definition.
                        if let Some(default_value) = expression_type.default_value() {
                            // Generate a declaration for this variable here.
                            let decl = Declaration::new(variable_name, expression_type.clone())
                                .with_description(format!(
                                    "Implicitly declared in {}, node {}",
                                    get_filename(&self.file.name),
                                    self.current_node_name.as_ref().unwrap()
                                ))
                                .with_default_value(default_value)
                                .with_source_file_name(self.file.name.clone())
                                .with_source_node_name_optional(self.current_node_name.clone())
                                .with_range(variable_context.range())
                                .with_implicit();
                            self.new_declarations.push(decl);
                        } else {
                            self.diagnostics.push(
                                Diagnostic::from_message(
                                    format_cannot_determine_variable_type_error(&variable_name),
                                )
                                .with_file_name(&self.file.name)
                                .with_parser_context(ctx, self.file.tokens()),
                            )
                        }
                    }
                    _ => {
                        // Implementation note: Apparently, this is unhandled? Maybe it's unreachable? Idk.
                    }
                }
            }
            yarnspinnerlexer::OPERATOR_MATHS_ADDITION_EQUALS => {
                // += supports strings and numbers
                let operator =
                    CodeGenerationVisitor::token_to_operator(yarnspinnerlexer::OPERATOR_MATHS_ADDITION).unwrap();
                expression_type = self.check_operation(ctx, terms, operator, op.get_text(), &[]);
            }
            yarnspinnerlexer::OPERATOR_MATHS_SUBTRACTION_EQUALS => {
                // -=, *=, /=, %= supports only numbers
                let operator =
                    CodeGenerationVisitor::token_to_operator(yarnspinnerlexer::OPERATOR_MATHS_SUBTRACTION).unwrap();
                expression_type = self.check_operation(ctx, terms, operator, op.get_text(), &[]);
            }
            yarnspinnerlexer::OPERATOR_MATHS_MULTIPLICATION_EQUALS => {
                let operator =
                    CodeGenerationVisitor::token_to_operator(yarnspinnerlexer::OPERATOR_MATHS_MULTIPLICATION).unwrap();
                expression_type = self.check_operation(ctx, terms, operator, op.get_text(), &[]);
            }
            yarnspinnerlexer::OPERATOR_MATHS_DIVISION_EQUALS => {
                let operator =
                    CodeGenerationVisitor::token_to_operator(yarnspinnerlexer::OPERATOR_MATHS_DIVISION).unwrap();
                expression_type = self.check_operation(ctx, terms, operator, op.get_text(), &[]);
            }
            yarnspinnerlexer::OPERATOR_MATHS_MODULUS_EQUALS => {
                let operator = CodeGenerationVisitor::token_to_operator(yarnspinnerlexer::OPERATOR_MATHS_MODULUS).unwrap();
                expression_type = self.check_operation(ctx, terms, operator, op.get_text(), &[]);
            }
            _ => panic!("Internal error: `visit_set_statement` got unexpected operand {}. This is a bug. Please report it at https://github.com/YarnSpinnerTool/YarnSpinner-Rust/issues/new", op.get_text())
        }
        if variable_type.is_none() && expression_type.is_none() {
            self.diagnostics.push(
                            Diagnostic::from_message(
                                format!("Type of expression \"{}\" can't be determined without more context. Please declare one or more terms.", ctx.get_text_with_whitespace(self.file.tokens())))
                                .with_file_name(&self.file.name)
                                .with_parser_context(ctx, self.file.tokens()));
        }
        // at this point we have either fully resolved the type of the expression or been unable to do so
        // we return the type of the expression regardless and rely on either elements to catch the issue
        expression_type
    }

    fn visit_jumpToExpression(&mut self, ctx: &JumpToExpressionContext<'input>) -> Self::Return {
        let expressions = &[ctx.expression().unwrap().into()];
        // The expression's type must resolve to a string.
        self.check_operation(ctx, expressions, None, "jump statement", &[Type::String])
    }
}

trait DeclarationVecExt {
    fn position(&self, declaration: &Declaration) -> Option<usize>;
    fn find_remove(&mut self, declaration: &Declaration);
}

impl DeclarationVecExt for Vec<Declaration> {
    fn position(&self, declaration: &Declaration) -> Option<usize> {
        self.iter().position(|decl| decl.eq(declaration, 1e-4))
    }

    fn find_remove(&mut self, declaration: &Declaration) {
        if let Some(index) = self.position(declaration) {
            self.remove(index);
        }
    }
}

/// {0} = variable name
fn format_cannot_determine_variable_type_error(name: &str) -> String {
    format!("Can't figure out the type of variable {name} given its context. Specify its type with a <<declare>> statement.")
}

fn get_filename(path: &str) -> &str {
    if let Some(os_str) = Path::new(path).file_name() {
        if let Some(file_name) = os_str.to_str() {
            return file_name;
        }
    }
    path
}

trait DefaultValue {
    fn default_value(&self) -> Option<YarnValue>;
}
impl DefaultValue for Type {
    fn default_value(&self) -> Option<YarnValue> {
        match self {
            Type::String => Some(YarnValue::String(Default::default())),
            Type::Number => Some(YarnValue::Number(Default::default())),
            Type::Boolean => Some(YarnValue::Boolean(Default::default())),
            _ => None,
        }
    }
}

impl DefaultValue for Option<Type> {
    fn default_value(&self) -> Option<YarnValue> {
        self.as_ref()?.default_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_valid_assignments() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
<<declare $foo to 1>>
<<set $foo to 2>>
<<declare $bar to true>>
<<declare $baz to \"hello\">>
<<set $bar to false>>
<<set $baz to \"world\">>
==="
            .to_string(),
        };
        let _result = Compiler {
            files: vec![file],
            library: Default::default(),
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        }
        .compile()
        .unwrap();
    }

    #[test]
    fn catches_invalid_assignments() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
<<declare $foo to 1>>
<<set $foo to \"invalid\">>
<<declare $bar to true>>
<<declare $baz to \"hello\">>
<<set $bar to -15>>
<<set $baz to false>>
==="
            .to_string(),
        };
        let result = Compiler {
            files: vec![file],
            library: Default::default(),
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        }
        .compile();

        let diagnostics = result.unwrap_err().0;

        assert_eq!(3, diagnostics.len());

        assert_contains(
            &diagnostics,
            &Diagnostic::from_message("$foo (Number) cannot be assigned a String")
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 3,
                        character: 0,
                    }..Position {
                        line: 3,
                        character: 25,
                    },
                ),
        );

        assert_contains(
            &diagnostics,
            &Diagnostic::from_message("$bar (Bool) cannot be assigned a Number")
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 6,
                        character: 0,
                    }..Position {
                        line: 6,
                        character: 19,
                    },
                ),
        );

        assert_contains(
            &diagnostics,
            &Diagnostic::from_message("$baz (String) cannot be assigned a Bool")
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 7,
                        character: 0,
                    }..Position {
                        line: 7,
                        character: 21,
                    },
                ),
        );
    }

    fn assert_contains(diagnostics: &[Diagnostic], expected: &Diagnostic) {
        assert!(
            // Does not factor in context or start line because these are subject to frequent change
            diagnostics.iter().any(|d| d.file_name == expected.file_name
                && d.message == expected.message
                && d.range == expected.range),
            "Expected diagnostics:\n{}\nto contain:\n- {:?}",
            diagnostics
                .iter()
                .map(|d| format!("- {d:?}"))
                .collect::<Vec<_>>()
                .join("\n"),
            expected
        );
    }

    #[test]
    fn allows_valid_math() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
<<declare $foo to 1>>
<<declare $bar = 2>>
<<set $foo to $foo + $bar>>
<<set $bar to $bar * $foo>>
==="
            .to_string(),
        };
        let _result = Compiler {
            files: vec![file],
            library: Default::default(),
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        }
        .compile()
        .unwrap();
    }

    #[test]
    fn catches_invalid_math() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
<<declare $foo to 1>>
<<declare $bar = \"invalid\">>
<<set $foo to $foo + $bar>>
<<set $foo to $foo * \"invalid\">>
==="
            .to_string(),
        };
        let result = Compiler {
            files: vec![file],
            library: Default::default(),
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        }
        .compile();

        let diagnostics = result.unwrap_err().0;

        assert_eq!(4, diagnostics.len());

        assert_contains(
            &diagnostics,
            &Diagnostic::from_message("$foo (Number) cannot be assigned a undefined")
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 4,
                        character: 0,
                    }..Position {
                        line: 4,
                        character: 27,
                    },
                ),
        );

        assert_contains(
            &diagnostics,
            &Diagnostic::from_message("$foo (Number) cannot be assigned a undefined")
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 5,
                        character: 0,
                    }..Position {
                        line: 5,
                        character: 32,
                    },
                ),
        );

        assert_contains(
            &diagnostics,
            &Diagnostic::from_message("All terms of + must be the same, not Number, String")
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 4,
                        character: 14,
                    }..Position {
                        line: 4,
                        character: 25,
                    },
                ),
        );

        assert_contains(
            &diagnostics,
            &Diagnostic::from_message("All terms of * must be the same, not Number, String")
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 5,
                        character: 14,
                    }..Position {
                        line: 5,
                        character: 30,
                    },
                ),
        );
    }
}
