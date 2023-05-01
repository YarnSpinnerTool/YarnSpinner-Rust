use crate::parser_rule_context_ext::ParserRuleContextExt;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::*;
use crate::visitors::type_check_visitor::{
    format_cannot_determine_variable_type_error, get_filename, DefaultValue,
};
use crate::visitors::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use better_any::TidExt;
use rusty_yarn_spinner_core::prelude::Operator;
use rusty_yarn_spinner_core::types::{SubTypeOf, Type, TypeFormat};
use std::cmp::Ordering;
use std::ops::Deref;
use std::rc::Rc;

impl<'input> TypeCheckVisitor<'input> {
    /// ok so what do we actually need to do in here?
    /// we need to do a few different things
    /// basically we need to go through the various types in the expression
    /// if any are known we need to basically log that
    /// then at the end if there are still unknowns we check if the operation itself forces a type
    /// so if we have say Undefined = Undefined + Number then we know that only one operation supports + Number and that is Number + Number
    /// so we can slot the type into the various parts
    pub(super) fn check_operation(
        &mut self,
        context: &impl ParserRuleContext<'input>,
        terms: Vec<Term<'input>>,
        operation_type: impl Into<Option<Operator>>,
        operation_description: &str,
        permitted_types: Vec<Type>,
    ) -> Option<Type> {
        let operation_type = operation_type.into();
        let mut term_types = Vec::new();
        let mut expression_type = None;
        for expression in &terms {
            // Visit this expression, and determine its type.
            let r#type = self.visit(expression.deref());
            if let Some(r#type) = r#type.clone() {
                if expression_type.is_none() {
                    // This is the first concrete type we've seen. This
                    // will be our expression type.
                    expression_type = Some(r#type.clone());
                }
                term_types.push(r#type);
            }
        }
        if permitted_types.len() == 1 && expression_type.is_none() {
            // If we aren't sure of the expression type from
            // parameters, but we only have one permitted one, then
            // assume that the expression type is the single permitted
            // type.

            // Guaranteed to be `Some`
            expression_type = permitted_types.first().cloned();
        }

        if expression_type.is_none() {
            // We still don't know what type of expression this is, and
            // don't have a reasonable guess.

            // Last-ditch effort: is the operator that we were given
            // valid in exactly one type? In that case, we'll decide
            // it's that type.
            if let Some(operation_type) = operation_type {
                let operation_type_name = operation_type.to_string();
                let types_implementing_method: Vec<_> = Type::EXPLICITLY_CONSTRUCTABLE
                    .iter()
                    .filter(|t| t.has_method(&operation_type_name))
                    .collect();
                match types_implementing_method.len().cmp(&1_usize) {
                    Ordering::Equal => {
                        // Only one type implements the operation we were
                        // given. Given no other information, we will assume
                        // that it is this type.

                        // Guaranteed to be `Some`
                        expression_type = types_implementing_method.first().cloned().cloned();
                    }
                    Ordering::Greater => {
                        // Multiple types implement this operation.
                        let type_names = types_implementing_method
                            .iter()
                            .map(|t| t.format())
                            .collect::<Vec<_>>()
                            .join(", or ");
                        let message = format!(
                        "Type of expression \"{}\" can't be determined without more context (the compiler thinks it could be {type_names}). Use a type cast on at least one of the terms (e.g. the string(), number(), bool() functions)",
                        context.get_text_with_whitespace(self.file.tokens()),
                    );
                        let diagnostic = Diagnostic::from_message(message)
                            .with_file_name(&self.file.name)
                            .read_parser_rule_context(context, self.file.tokens());
                        self.diagnostics.push(diagnostic);
                        return None;
                    }
                    Ordering::Less => {
                        // No types implement this operation (??) [sic]
                        let message = format!(
                        "Type of expression \"{}\" can't be determined without more context. Use a type cast on at least one of the terms (e.g. the string(), number(), bool() functions)",
                        context.get_text_with_whitespace(self.file.tokens()),
                    );
                        let diagnostic = Diagnostic::from_message(message)
                            .with_file_name(&self.file.name)
                            .read_parser_rule_context(context, self.file.tokens());
                        self.diagnostics.push(diagnostic);
                        return None;
                    }
                }
            }
        }

        // to reach this point we have either worked out the final type of the expression
        // or had to give up, and if we gave up we have nothing left to do
        // there are then two parts to this, first we need to declare the implicit type of any variables (that appears to be working)
        // or the implicit type of any function.
        // annoyingly the function will already have an implicit definition created for it
        // we will have to strip that out and add in a new one with the new return type
        for term in &terms {
            let Term::Expression(expression) = term else { continue; };
            let ExpressionContextAll::ExpValueContext(value_context) = expression.as_ref() else { continue; };
            let Some(value) = value_context.value() else { continue; };
            let ValueContextAll::ValueFuncContext(func_context) = value.as_ref() else { continue; };

            let id = func_context
                .function_call()
                .unwrap()
                .FUNC_ID()
                .unwrap()
                .get_text();

            let function_type = self
                .new_declarations
                .iter_mut()
                .filter(|decl| decl.name == id)
                .find_map(|decl| {
                    if let Some(Type::Function(ref mut func)) = decl.r#type {
                        Some(func)
                    } else {
                        None
                    }
                });
            if let Some(func) = function_type {
                if func.return_type.is_some() {
                    continue;
                }
                func.return_type = Box::new(expression_type.clone());
            } else {
                self.visit(term.deref());
            }
        }
        // Were any of the terms variables for which we don't currently
        // have a declaration for?

        // Start by building a list of all terms that are variables.
        // These are either variable values, or variable names . (The
        // difference between these two is that a ValueVarContext
        // occurs in syntax where the value of the variable is used
        // (like an expression), while a VariableContext occurs in
        // syntax where it's just a variable name (like a set
        // statements)

        // All VariableContexts in the terms of this expression (but
        // not in the children of those terms)
        let variable_contexts = terms
            .iter()
            .filter_map(|term| {
                term.child_of_type_unsized::<ValueContextAll>(0)
                    .and_then(|value_context| {
                        if let ValueContextAll::ValueVarContext(context) = value_context.as_ref() {
                            context.variable()
                        } else {
                            None
                        }
                    })
            })
            .chain(
                terms
                    .iter()
                    .find_map(|term| term.child_of_type_unsized::<VariableContext>(0)),
            )
            .chain(
                terms.iter().filter_map(|term| {
                    term.generic_context().downcast_rc::<VariableContext>().ok()
                }),
            )
            .chain(
                terms
                    .iter()
                    .filter_map(|term| term.generic_context().downcast_rc::<ValueContextAll>().ok())
                    .filter_map(|value_context| {
                        if let ValueContextAll::ValueVarContext(context) = value_context.as_ref() {
                            context.variable()
                        } else {
                            None
                        }
                    }),
            );

        // Build the list of variable contexts that we don't have a
        // declaration for. We'll check for explicit declarations first.
        let mut undefined_variable_contexts: Vec<_> = variable_contexts
            .filter(|v| {
                !self
                    .declarations()
                    .iter()
                    .any(|d| d.name == v.VAR_ID().unwrap().get_text())
            })
            .collect();
        // Implementation note: The original compares by reference here. The interval should be unique for each context, so let's use that instead.
        undefined_variable_contexts.sort_by_key(|v| v.get_hashable_interval());
        undefined_variable_contexts.dedup_by_key(|v| v.get_hashable_interval());

        for undefined_variable_context in undefined_variable_contexts {
            // We have references to variables that we don't have a an
            // explicit declaration for! Time to create implicit
            // references for them!

            let var_name = undefined_variable_context.VAR_ID().unwrap().get_text();
            // We can only create an implicit declaration for a variable
            // if we have a default value for it, because all variables
            // are required to have a value. If we can't, it's generally
            // because we couldn't figure out a concrete type for the
            // variable given the context.
            if let Some(default_value) = expression_type.default_value() {
                let file_name = get_filename(&self.file.name);
                let node = self
                    .current_node_name
                    .as_ref()
                    .map(|name| format!(", node {name}"))
                    .unwrap_or_default();
                let decl = Declaration::default()
                    .with_name(&var_name)
                    .with_description(format!("Implicitly declared in {file_name}{node}"))
                    .with_type(expression_type.clone())
                    .with_default_value(default_value)
                    .with_source_file_name(self.file.name.clone())
                    .with_source_node_name_optional(self.current_node_name.clone())
                    .with_range(undefined_variable_context.range(self.file.tokens()))
                    .with_implicit();
                self.new_declarations.push(decl);
            } else {
                // If we can't produce this, then we can't generate the
                // declaration.
                let diagnostic = Diagnostic::from_message(
                    format_cannot_determine_variable_type_error(&var_name),
                )
                .with_file_name(&self.file.name)
                .read_parser_rule_context(undefined_variable_context.as_ref(), self.file.tokens());
                self.diagnostics.push(diagnostic);
                continue;
            }
        }

        // All types must be same as the expression type (which is the
        // first defined type we encountered when going through the
        // terms)
        if !term_types
            .iter()
            .all(|t| Some(t) == expression_type.as_ref())
        {
            // Not all the term types we found were the expression
            // type.
            let type_list = term_types
                .iter()
                .map(|t| t.format())
                .collect::<Vec<_>>()
                .join(", ");
            let message =
                format!("All terms of {operation_description} must be the same, not {type_list}");
            let diagnostic = Diagnostic::from_message(message)
                .with_file_name(&self.file.name)
                .read_parser_rule_context(context, self.file.tokens());
            self.diagnostics.push(diagnostic);
            return None;
        }

        // We've now determined that this expression is of
        // expressionType. In case any of the terms had an undefined
        // type, we'll define it now.
        for term in terms {
            if let Term::Expression(expression) = term {
                if self.known_types.get(expression.as_ref()).is_none() {
                    self.known_types
                        .insert(expression.as_ref(), expression_type.clone());
                }
                // Guaranteed to be Some
                let expression = self.known_types.get_mut(expression.as_ref()).unwrap();
                if let Type::Function(ref mut function_type) = expression {
                    function_type.set_return_type(expression_type.clone());
                }
            }
        }
        if let Some(operation_type) = operation_type {
            // We need to validate that the type we've selected actually
            // implements this operation.

            // By the logic of this function, this is guaranteed to be Some
            let expression_type = expression_type.as_ref().unwrap();
            let implements_method = expression_type.has_method(&operation_type.to_string());
            if !implements_method {
                let message = format!(
                    "{} has no implementation defined for {operation_description}",
                    expression_type.format(),
                );
                let diagnostic = Diagnostic::from_message(message)
                    .with_file_name(&self.file.name)
                    .read_parser_rule_context(context, self.file.tokens());
                self.diagnostics.push(diagnostic);
                return None;
            }
        }

        // Is this expression is required to be one of the specified types?
        if !permitted_types.is_empty() {
            // Is the type that we've arrived at compatible with one of
            // the permitted types?
            if permitted_types
                .iter()
                .any(|t| expression_type.is_sub_type_of(t))
            {
                // It's compatible! Great, return the type we've
                // determined.
                return expression_type;
            }
            // The expression type wasn't valid!
            let permitted_types_list = permitted_types
                .iter()
                .map(|t| t.format())
                .collect::<Vec<_>>()
                .join(" or ");
            let type_list = term_types
                .iter()
                .map(|t| t.format())
                .collect::<Vec<_>>()
                .join(", ");
            let message = format!(
                "Terms of '{operation_description}' must be {permitted_types_list}, not {type_list}",
            );
            let diagnostic = Diagnostic::from_message(message)
                .with_file_name(&self.file.name)
                .read_parser_rule_context(context, self.file.tokens());
            self.diagnostics.push(diagnostic);
            return None;
        }
        // We weren't given a specific type. The expression type is
        // therefore only valid if it can use the provided
        // operator.

        // Find a type in 'expressionType's hierarchy that
        // implements this method.

        let has_method = expression_type
            .as_ref()
            .and_then(|exp| operation_type.map(|op| exp.has_method(&op.to_string())))
            .unwrap_or_default();
        if !has_method {
            // The type doesn't have a method for handling this
            // operator, and neither do any of its supertypes. This
            // expression is therefore invalid.
            let message = format!(
                "Operator {operation_description} cannot be used with {} values",
                expression_type.format()
            );
            self.diagnostics.push(
                Diagnostic::from_message(message)
                    .with_file_name(&self.file.name)
                    .read_parser_rule_context(context, self.file.tokens()),
            );
            return None;
        }
        expression_type
    }
}

/// Bandaid enum to allow static type checks that work via dynamic dispatch on C#
pub(super) enum Term<'input> {
    Expression(Rc<ExpressionContextAll<'input>>),
    Variable(Rc<VariableContextAll<'input>>),
}

impl<'input> Term<'input> {
    pub(super) fn generic_context(&self) -> Rc<ActualParserContext<'input>> {
        match self {
            Term::Expression(ctx) => ctx.clone() as Rc<ActualParserContext<'input>>,
            Term::Variable(ctx) => ctx.clone(),
        }
    }
}

impl<'input> Deref for Term<'input> {
    type Target = ActualParserContext<'input>;

    fn deref(&self) -> &Self::Target {
        match self {
            Term::Expression(ctx) => ctx.as_ref() as &ActualParserContext<'input>,
            Term::Variable(ctx) => ctx.as_ref(),
        }
    }
}

impl<'input> From<Rc<ExpressionContextAll<'input>>> for Term<'input> {
    fn from(ctx: Rc<ExpressionContextAll<'input>>) -> Self {
        Self::Expression(ctx)
    }
}

impl<'input> From<Rc<VariableContextAll<'input>>> for Term<'input> {
    fn from(ctx: Rc<VariableContextAll<'input>>) -> Self {
        Self::Variable(ctx)
    }
}
