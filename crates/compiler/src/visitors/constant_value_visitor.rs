//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ConstantValueVisitor.cs>

use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::*;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, VisitChildren};
use std::mem;
use std::ops::{Deref, DerefMut};
use yarn_slinger_core::prelude::InternalValue;

/// A visitor that visits any valid constant value, and returns a [`InternalValue`].
/// Currently only supports terminals, not expressions,
/// even if those expressions would be constant.
#[derive(Clone)]
pub(crate) struct ConstantValueVisitor<'input> {
    pub(crate) diagnostics: Vec<Diagnostic>,
    _dummy: ConstantValue,
    file: FileParseResult<'input>,
}

impl<'input> ConstantValueVisitor<'input> {
    pub(crate) fn new(diagnostics: Vec<Diagnostic>, file: FileParseResult<'input>) -> Self {
        Self {
            diagnostics,
            file,
            _dummy: ConstantValue::non_panicking_default(),
        }
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for ConstantValueVisitor<'input> {
    type Node = YarnSpinnerParserContextType;
    type Return = ConstantValue;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }

    fn visit(&mut self, node: &<Self::Node as ParserNodeType<'input>>::Type) -> Self::Return {
        // Calling `self.visit_node` resolves to the wrong trait, so we need to be explicit.
        VisitChildren::visit_node(self, node);
        // The default implementation uses `mem::take`, which replaces the value with the default.
        // However, we calling `default` on `ConstantValue` panics by design, so let's use the non-panicking version.
        mem::replace(self.temp_result(), ConstantValue::non_panicking_default())
    }
}

impl<'input> YarnSpinnerParserVisitorCompat<'input> for ConstantValueVisitor<'input> {
    fn visit_valueNumber(&mut self, ctx: &ValueNumberContext<'input>) -> Self::Return {
        let text = ctx.get_text();
        if let Ok(number) = text.parse::<f32>() {
            InternalValue::from(number).into()
        } else {
            let message = format!("Failed to parse {text} as a float",);
            self.diagnostics.push(
                Diagnostic::from_message(message)
                    .with_file_name(&self.file.name)
                    .read_parser_rule_context(ctx, self.file.tokens()),
            );
            // This default value seems very "JavaScript-y" with the pseudo-sensible default value on errors.
            // But this is not so! We just pushed an error diagnostic, so there will be no program emitted from this compilation attempt.
            // All this does is allow the compiler to continue and potentially collect further useful diagnostics!
            InternalValue::from(0.0).into()
        }
    }

    fn visit_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) -> Self::Return {
        InternalValue::from(true).into()
    }

    fn visit_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) -> Self::Return {
        InternalValue::from(false).into()
    }

    fn visit_valueString(&mut self, ctx: &ValueStringContext<'input>) -> Self::Return {
        let text = ctx.STRING().unwrap().get_text();
        InternalValue::from(text.trim_matches('"')).into()
    }

    fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>) -> Self::Return {
        let message = "Null is not a permitted type in Yarn Spinner 2.0 and later";
        self.diagnostics.push(
            Diagnostic::from_message(message)
                .with_file_name(&self.file.name)
                .read_parser_rule_context(ctx, self.file.tokens()),
        );
        ConstantValue::non_panicking_default()
    }
}

#[derive(Debug, Clone)]
/// Needed because ANTLR needs visitors' return values to have a default.
/// While the C# implementation allows overriding a `DefaultResult` property,
/// the Rust implementation simply takes the `Default` implementation of the associated `Return` type.
/// However, we don't have a default [`InternalValue`], which wouldn't make much sense, but panic when it would have been built by antl4rust,
/// so we use this wrapper to accomplish that.
///
/// This seems weird, I know. The original implementation writes a `Diagnostic` whenever the default value is constructed.
/// The thing is, the original code says the following:
/// > Default result is an exception - only specific parse nodes can
///  be visited by this visitor
///
/// We cannot write a diagnostic in the default implementation because we lack access to the diagnostics vector at that point.
/// But, judging by the original wording, this case should not happen anyways and should be treated as an internal error / a bug.
/// Thus, we panic instead with a call to action to report the bug.
pub(crate) struct ConstantValue(pub(crate) Option<InternalValue>);

impl Deref for ConstantValue {
    type Target = Option<InternalValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConstantValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<InternalValue> for ConstantValue {
    fn from(value: InternalValue) -> Self {
        Self(Some(value))
    }
}

impl Default for ConstantValue {
    /// The constant value visitor is meant to be used for very specific contexts, e.g. it is allowed to be called for a line but not for an entire dialog.
    /// This default implementation is called when the visitor is called in an unexpected way, which in the current implementation can indeed not happen.
    /// If we refactor the code wrongly, this panic will be reached and tell us.
    fn default() -> Self {
        unreachable!("The `ConstantValueVisitor` was called in an unexpected context. This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new")
    }
}

impl ConstantValue {
    /// Only use this for dummy assignments.
    fn non_panicking_default() -> Self {
        Self(None)
    }
}
