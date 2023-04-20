# ANTLR (.g4) files

Currently copied from [YarnSpinnerTool](https://github.com/YarnSpinnerTool/YarnSpinner/tree/v2.3.0/YarnSpinner.Compiler):
* [Parser](https://raw.githubusercontent.com/YarnSpinnerTool/YarnSpinner/v2.3.0/YarnSpinner.Compiler/YarnSpinnerParser.g4)
* [Lexer](https://raw.githubusercontent.com/YarnSpinnerTool/YarnSpinner/v2.3.0/YarnSpinner.Compiler/YarnSpinnerLexer.g4)

Generated with [custom ANTLR version](https://github.com/rrevenantt/antlr4rust/releases/tag/antlr4-4.8-2-Rust0.3.0-beta) \
and the command `java -jar "path/to/antlr4-4.8-2-SNAPSHOT-complete.jar" -Dlanguage=Rust .\YarnSpinnerLexer.g4 -visitor`

## Adjustments
* `YarnSpinnerParserParserContext` replaced with `YarnSpinnerParserContext` - no idea what happened there during generation
* Disambiguate `visit_node` by explicitly using `antlr_rust::tree::VisitChildren::visit_node` (other variant would be `yarnspinnerparservisitor::YarnSpinnerParserVisitor::visit_node`)
* Autoformat files
* `generated.rs` now contains allows for `warnings` and `clippy` together with `#![cfg_attr(rustfmt, rustfmt_skip)]`