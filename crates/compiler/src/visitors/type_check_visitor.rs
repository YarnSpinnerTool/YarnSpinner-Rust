use crate::prelude::generated::yarnspinnerlexer;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use rusty_yarn_spinner_core::types::{BooleanType, BuiltinType, NumberType, StringType, Type};

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

    tokens: &'a ActualTokenStream<'input>,
    _dummy: Option<BuiltinType>,
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
}

impl<'a, 'input: 'a> ParseTreeVisitorCompat<'input> for TypeCheckVisitor<'a, 'input> {
    type Node = YarnSpinnerParserContextType;

    type Return = Option<BuiltinType>;

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

        Some(BuiltinType::Undefined)
    }

    fn visit_valueString(&mut self, _ctx: &ValueStringContext<'input>) -> Self::Return {
        Some(BuiltinType::String(StringType))
    }

    fn visit_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) -> Self::Return {
        Some(BuiltinType::Boolean(BooleanType))
    }

    fn visit_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) -> Self::Return {
        Some(BuiltinType::Boolean(BooleanType))
    }

    fn visit_valueNumber(&mut self, _ctx: &ValueNumberContext<'input>) -> Self::Return {
        Some(BuiltinType::Number(NumberType))
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
            return Some(BuiltinType::Undefined)
        };
        let name = var_id.get_text();
        if let Some(declaration) = self
            .declarations()
            .into_iter()
            .find(|decl| decl.name == name)
        {
            return Some(declaration.r#type);
        }

        // do we already have a potential warning about this?
        // no need to make more
        if self
            .deferred_types
            .iter()
            .any(|deferred_type| deferred_type.name == name)
        {
            return Some(BuiltinType::Undefined);
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
        Some(BuiltinType::Undefined)
    }
}

/// {0} = variable name
fn format_cannot_determine_variable_type_error(name: &str) -> String {
    format!("Can't figure out the type of variable {name} given its context. Specify its type with a <<declare>> statement.")
}
