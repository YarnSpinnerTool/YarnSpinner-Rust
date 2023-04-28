use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use antlr_rust::interval_set::Interval;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use rusty_yarn_spinner_core::prelude::Operator;
use rusty_yarn_spinner_core::types::{FunctionType, SubTypeOf, Type, TypeOptionFormat};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// A visitor that walks the parse tree, checking for type consistency
/// in expressions. Existing type information is provided via the
/// [`existing_declarations`] property. This visitor will also
/// attempt to infer the type of variables that don't have an explicit
/// declaration; for each of these, a new Declaration will be created
/// and made available via the [`new_declaration`] property.
pub(crate) struct TypeCheckVisitor<'a, 'input: 'a> {
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

    // The name of the file that we're currently in.
    source_file_name: String,

    /// Gets or sets a type hint for the expression.
    /// This is mostly used by [`TypeCheckVisitor`]
    /// to give a hint that can be used by functions to
    /// influence their type when set to use inference.
    /// Won't be used if a concrete type is already known.
    ///
    /// ## Implementation notes
    ///
    /// In the original implementation, this was implemented
    /// on the [`ValueContext`] directly using a `partial`
    hints: HashMap<HashableInterval, Type>,

    tokens: &'a ActualTokenStream<'input>,
    _dummy: Option<Type>,
}

impl<'a, 'input: 'a> TypeCheckVisitor<'a, 'input> {
    pub(crate) fn new(
        source_file_name: String,
        existing_declarations: Vec<Declaration>,
        tokens: &'a ActualTokenStream<'input>,
    ) -> Self {
        Self {
            existing_declarations,
            source_file_name,
            tokens,
            diagnostics: Default::default(),
            new_declarations: Default::default(),
            deferred_types: Default::default(),
            current_node_name: Default::default(),
            hints: Default::default(),
            _dummy: Default::default(),
        }
    }

    /// Gets the collection of all declarations - both the ones we received
    /// at the start, and the new ones we've derived ourselves.
    pub(crate) fn declarations(&self) -> Vec<Declaration> {
        self.existing_declarations
            .iter()
            .cloned()
            .chain(self.new_declarations.iter().cloned())
            .collect()
    }

    fn get_hint(&self, ctx: &impl ParserRuleContext<'input>) -> Option<&Type> {
        let interval = ctx.get_source_interval();
        let hashable_interval = HashableInterval(interval);
        self.hints.get(&hashable_interval)
    }

    fn set_hint(
        &mut self,
        ctx: &impl ParserRuleContext<'input>,
        hint: impl Into<Option<Type>>,
    ) -> Option<Type> {
        let hint = hint.into()?;
        let interval = ctx.get_source_interval();
        let hashable_interval = HashableInterval(interval);
        self.hints.insert(hashable_interval, hint)
    }

    /// ok so what do we actually need to do in here?
    /// we need to do a few different things
    /// basically we need to go through the various types in the expression
    /// if any are known we need to basically log that
    /// then at the end if there are still unknowns we check if the operation itself forces a type
    /// so if we have say Undefined = Undefined + Number then we know that only one operation supports + Number and that is Number + Number
    /// so we can slot the type into the various parts
    fn check_operation(
        &mut self,
        context: &impl ParserRuleContext<'input>,
        terms: Vec<
            Rc<
                dyn ParserRuleContext<
                    'input,
                    Ctx = YarnSpinnerParserContextType,
                    TF = CommonTokenFactory,
                >,
            >,
        >,
        operation_type: Operator,
        operation_description: String,
        permitted_types: Vec<Type>,
    ) -> Type {
        todo!()
    }
}

impl<'a, 'input: 'a> ParseTreeVisitorCompat<'input> for TypeCheckVisitor<'a, 'input> {
    type Node = YarnSpinnerParserContextType;

    type Return = Option<Type>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'a, 'input: 'a> YarnSpinnerParserVisitorCompat<'input> for TypeCheckVisitor<'a, 'input> {
    fn visit_node(&mut self, ctx: &NodeContext<'input>) -> Self::Return {
        for header in ctx.header_all() {
            let key = header.header_key.as_ref().unwrap().get_text();
            if key == "title" {
                let value = header.header_value.as_ref().unwrap().get_text();
                self.current_node_name = Some(value.to_owned());
            }
        }
        if let Some(body) = ctx.body() {
            self.visit(&*body);
        }
        None
    }

    fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>) -> Self::Return {
        self.diagnostics.push(
            Diagnostic::from_message("Null is not a permitted type in Yarn Spinner 2.0 and later")
                .with_file_name(&self.source_file_name)
                .read_parser_rule_context(ctx, self.tokens),
        );

        None
    }

    fn visit_valueString(&mut self, _ctx: &ValueStringContext<'input>) -> Self::Return {
        Some(Type::String)
    }

    fn visit_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) -> Self::Return {
        Some(Type::Boolean)
    }

    fn visit_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) -> Self::Return {
        Some(Type::Boolean)
    }

    fn visit_valueNumber(&mut self, _ctx: &ValueNumberContext<'input>) -> Self::Return {
        Some(Type::Number)
    }

    fn visit_valueVar(&mut self, ctx: &ValueVarContext<'input>) -> Self::Return {
        let variable = ctx.variable().unwrap();
        self.visit_variable(&*variable)
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        // The type of the value depends on the declared type of the
        // variable
        let Some(var_id) = ctx.get_token(yarnspinnerlexer::VAR_ID, 0) else {
                // We don't have a variable name for this Variable context.
                // The parser will have generated an error for us in an
                // earlier stage; here, we'll bail out.
            return None
        };
        let name = var_id.get_text();
        if let Some(declaration) = self
            .declarations()
            .into_iter()
            .find(|decl| decl.name == name)
        {
            return declaration.r#type;
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
                .with_file_name(&self.source_file_name)
                .read_parser_rule_context(ctx, self.tokens);
        self.deferred_types
            .push(DeferredTypeDiagnostic { name, diagnostic });

        // We don't have a declaration for this variable. Return
        // Undefined. Hopefully, other context will allow us to infer a
        // type.
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
            .into_iter()
            .find(|decl| decl.name == function_name);
        let hint = self.get_hint(ctx).cloned();
        let function_type = if let Some(function_declaration) = function_declaration {
            let Some(Type::Function(mut function_type)) = function_declaration.r#type.clone() else {
                 unreachable!("Internal error: function declaration is not of type Function. This is a bug. Please report it at https://github.com/Mafii/rusty-yarn-spinner/issues/new")
            };

            // we have an existing function but its undefined
            // if we also have a type hint we can use that to update it
            if function_type.return_type.is_none() {
                if let Some(hint) = hint {
                    if let Some(index) = self
                        .new_declarations
                        .iter()
                        .filter_map(|decl| decl.eq(&function_declaration, 1e-4).ok())
                        .position(|eq| eq)
                    {
                        self.new_declarations.remove(index);
                    }
                    function_type.set_return_type(hint);
                    let new_declaration = Declaration {
                        r#type: Some(Type::from(function_type.clone())),
                        ..function_declaration
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
            let line = ctx.start().get_line();
            let column = ctx.start().get_column();
            let function_declaration = Declaration::default()
                .with_type(Type::from(function_type.clone()))
                .with_name(&function_name)
                .with_description(format!(
                    "Implicit declaration of function at {}:{}:{}",
                    self.source_file_name, line, column
                ))
                // All positions are +1 compared to original implementation, but the result is the same.
                // I suspect the C# ANTLR implementation is 1-based while antlr4rust is 0-based.
                .with_range(
                    Position {
                        line: line as usize,
                        character: column as usize + 1,
                    }..=Position {
                        line: line as usize,
                        character: column as usize + 1 + ctx.stop().get_text().len(),
                    },
                )
                .with_implicit();

            // Create the array of parameters for this function based
            // on how many we've seen in this call. Set them all to be
            // undefined; we'll bind their type shortly.
            let expressions = ctx.function_call().unwrap().expression_all();
            let parameter_types = expressions.iter().map(|_| None);
            for parameter_type in parameter_types {
                function_type.add_parameter(parameter_type);
            }
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
                "Function {} expects {} {}, but received {}",
                function_name,
                expected_parameter_types.len(),
                parameters,
                supplied_parameters.len()
            ))
            .with_file_name(&self.source_file_name)
            .read_parser_rule_context(ctx, self.tokens);
            self.diagnostics.push(diagnostic);
            return *function_type.return_type;
        }

        for (i, (supplied_parameter, mut expected_type)) in supplied_parameters
            .iter()
            .cloned()
            .zip(expected_parameter_types.iter())
            .enumerate()
        {
            let supplied_type = self.visit(&*supplied_parameter);
            if expected_type.is_none() {
                // The type of this parameter hasn't yet been bound.
                // Bind this parameter type to what we've resolved the
                // type to.
                expected_type = &supplied_type;
            }
            if !expected_type.is_sub_type_of(&supplied_type) {
                let diagnostic = Diagnostic::from_message(format!(
                    "{} parameter {} expects a {}, not a {}",
                    function_name,
                    i + 1,
                    expected_type.format(),
                    supplied_type.format()
                ))
                .with_file_name(&self.source_file_name)
                .read_parser_rule_context(ctx, self.tokens);
                self.diagnostics.push(diagnostic);
            }
        }
        // Cool, all the parameters check out!

        // Finally, return the return type of this function.
        *function_type.return_type
    }

    fn visit_expValue(&mut self, ctx: &ExpValueContext<'input>) -> Self::Return {
        // passing the hint from the expression down into the values within
        let hint = self.get_hint(ctx).cloned();
        let value = ctx.value().unwrap();
        self.set_hint(&*value, hint);
        // Value expressions have the type of their inner value
        let r#type = self.visit(&*value);
        self.set_hint(ctx, r#type.clone());
        r#type
    }

    fn visit_expParens(&mut self, ctx: &ExpParensContext<'input>) -> Self::Return {
        // Parens expressions have the type of their inner expression
        let r#type = self.visit(&*ctx.expression().unwrap());
        self.set_hint(ctx, r#type.clone());
        r#type
    }

    fn visit_expAndOrXor(&mut self, ctx: &ExpAndOrXorContext<'input>) -> Self::Return {
        let expressions: Vec<_> = ctx
            .expression_all()
            .into_iter()
            .map(|expr| {
                expr as Rc<
                    dyn ParserRuleContext<
                        'input,
                        Ctx = YarnSpinnerParserContextType,
                        TF = CommonTokenFactory,
                    >,
                >
            })
            .collect();
        let operator: Operator = todo!();
        let description = ctx.op.unwrap().get_text().to_owned();
        let r#type = self.check_operation(ctx, expressions, operator, description, vec![]);
    }

    fn visit_set_statement(&mut self, ctx: &Set_statementContext<'input>) -> Self::Return {
        todo!()
    }
}

/// {0} = variable name
fn format_cannot_determine_variable_type_error(name: &str) -> String {
    format!("Can't figure out the type of variable {name} given its context. Specify its type with a <<declare>> statement.")
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HashableInterval(Interval);

impl From<Interval> for HashableInterval {
    fn from(interval: Interval) -> Self {
        Self(interval)
    }
}

impl Hash for HashableInterval {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.a.hash(state);
        self.0.b.hash(state);
    }
}

impl Deref for HashableInterval {
    type Target = Interval;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HashableInterval {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
