//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/ConstantValueVisitor.cs>

use crate::parser::generated::yarnspinnerparser::ValueNullContext;
use crate::prelude::generated::yarnspinnerparser::{
    YarnSpinnerParserContext, YarnSpinnerParserContextType,
};
use crate::prelude::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitorCompat;
use crate::prelude::Diagnostic;

use antlr_rust::tree::{ParseTreeVisitorCompat};

use rusty_yarn_spinner_core::prelude::Value;

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// A visitor that visits any valid constant value, and returns a [`Value`].
/// Currently only supports terminals, not expressions,
/// even if those expressions would be constant.
#[derive(Debug, Clone)]
pub(crate) struct ConstantValueVisitor<'a, 'input: 'a, P: YarnSpinnerParserContext<'input>> {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) file_name: String,
    pub(crate) rule_context: &'a P,
    _phantom_data: PhantomData<&'input ()>,
    _dummy: ConstantValue,
}

impl<'a, 'input: 'a, P: YarnSpinnerParserContext<'input>> ConstantValueVisitor<'a, 'input, P> {
    pub(crate) fn new(
        file_name: impl Into<String>,
        rule_context: &'a P,
        diagnostics: Vec<Diagnostic>,
    ) -> Self {
        Self {
            file_name: file_name.into(),
            rule_context,
            diagnostics,
            _phantom_data: PhantomData,
            _dummy: ConstantValue::non_panicking_default(),
        }
    }
}

impl<'a, 'input: 'a, P: YarnSpinnerParserContext<'input>> ParseTreeVisitorCompat<'input>
    for ConstantValueVisitor<'a, 'input, P>
{
    type Node = YarnSpinnerParserContextType;
    type Return = ConstantValue;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self._dummy
    }
}

impl<'a, 'input: 'a, P: YarnSpinnerParserContext<'input>> YarnSpinnerParserVisitorCompat<'input>
    for ConstantValueVisitor<'a, 'input, P>
{
    fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>) -> Self::Return {
        let message = "Null is not a permitted type in Yarn Spinner 2.0 and later";
        self.diagnostics.push(
            Diagnostic::from_message(message)
                .with_file_name(&self.file_name)
                .read_parser_rule_context(ctx),
        );
        ConstantValue::non_panicking_default()
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
