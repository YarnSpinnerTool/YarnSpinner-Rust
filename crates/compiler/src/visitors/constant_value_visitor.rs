//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ConstantValueVisitor.cs>

use crate::parser::generated::yarnspinnerparser::{
    ValueFalseContext, ValueNullContext, ValueNumberContext, ValueStringContext,
};
use crate::prelude::generated::yarnspinnerparser::{
    ValueStringContextAttrs, ValueTrueContext, YarnSpinnerParserContextType,
};
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::Diagnostic;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use rusty_yarn_spinner_core::prelude::Value;
use std::ops::{Deref, DerefMut};

/// A visitor that visits any valid constant value, and returns a [`Value`].
/// Currently only supports terminals, not expressions,
/// even if those expressions would be constant.
#[derive(Debug, Clone)]
pub(crate) struct ConstantValueVisitor {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) file_name: String,
    _dummy: ConstantValue,
}

impl ConstantValueVisitor {
    #[allow(dead_code)] // TODO: Remove this once we have implemented `DeclarationVisitor`.
    pub(crate) fn new(file_name: impl Into<String>, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            file_name: file_name.into(),
            diagnostics,
            _dummy: ConstantValue::non_panicking_default(),
        }
    }
}

impl ParseTreeVisitorCompat<'_> for ConstantValueVisitor {
    type Node = YarnSpinnerParserContextType;
    type Return = ConstantValue;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl YarnSpinnerParserVisitorCompat<'_> for ConstantValueVisitor {
    fn visit_valueNull(&mut self, ctx: &ValueNullContext<'_>) -> Self::Return {
        let message = "Null is not a permitted type in Yarn Spinner 2.0 and later";
        self.diagnostics.push(
            Diagnostic::from_message(message)
                .with_file_name(&self.file_name)
                .read_parser_rule_context(ctx),
        );
        ConstantValue::non_panicking_default()
    }

    fn visit_valueNumber(&mut self, ctx: &ValueNumberContext<'_>) -> Self::Return {
        let text = ctx.get_text();
        if let Ok(result) = text.parse::<f32>() {
            Value::from(result).into()
        } else {
            let message = format!("Failed to parse {text} as a float",);
            self.diagnostics.push(
                Diagnostic::from_message(message)
                    .with_file_name(&self.file_name)
                    .read_parser_rule_context(ctx),
            );
            // This default value seems very "JavaScript-y" with the pseudo-sensible default value on errors.
            // But this is not so! We just pushed an error diagnostic, so there will be no program emitted from this compilation attempt.
            // All this does is allow the compiler to continue and potentially collect further useful diagnostics!
            Value::from(0.0).into()
        }
    }

    fn visit_valueString(&mut self, ctx: &ValueStringContext<'_>) -> Self::Return {
        let text = ctx.STRING().unwrap().get_text();
        Value::from(text.trim_matches('"')).into()
    }

    fn visit_valueFalse(&mut self, _ctx: &ValueFalseContext<'_>) -> Self::Return {
        Value::from(false).into()
    }

    fn visit_valueTrue(&mut self, _ctx: &ValueTrueContext<'_>) -> Self::Return {
        Value::from(true).into()
    }
}

#[derive(Debug, Clone)]
/// Needed because ANTLR needs visitors' return values to have a default.
/// While the C# implementation allows overriding a `DefaultResult` property,
/// the Rust implementation simply takes the `Default` implementation of the associated`Return` type.
/// However, `Value` should not have a `Default` implementation, because it would be a useless and invalid value.
/// Besides, we want to panic on `Default::default` anyways, so we use this wrapper to accomplish that.
///
/// This seems weird, I know. The original implementation writes a `Diagnostic` whenever the default value is constructed.
/// The thing is, the original code says, I quote:
/// >Default result is an exception - only specific parse nodes can
///  be visited by this visitor
///
/// We cannot write a diagnostic in the default because we lack access to the diagnostics vector at that point.
/// But, judging by the original wording, this case should not happen anyways and should be treated as an internal error / a bug.
/// Thus, we panic instead with a call to action to report the bug.
pub(crate) struct ConstantValue(Value);

impl Deref for ConstantValue {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConstantValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Value> for ConstantValue {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl Default for ConstantValue {
    fn default() -> Self {
        panic!("The `ConstantValueVisitor` was called in an unexpected context. This is a bug. Please report it at https://github.com/Mafii/rusty-yarn-spinner/issues/new")
    }
}

impl ConstantValue {
    /// Only use this for dummy assignments.
    fn non_panicking_default() -> Self {
        Self(Default::default())
    }
}
