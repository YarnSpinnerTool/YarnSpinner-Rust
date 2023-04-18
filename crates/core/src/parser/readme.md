# ANTLR (.g4) files

Currently copied from [YarnSpinnerTool](https://github.com/YarnSpinnerTool/YarnSpinner/tree/v2.3.0/YarnSpinner.Compiler):
* [Parser](https://raw.githubusercontent.com/YarnSpinnerTool/YarnSpinner/v2.3.0/YarnSpinner.Compiler/YarnSpinnerParser.g4)
* [Lexer](https://raw.githubusercontent.com/YarnSpinnerTool/YarnSpinner/v2.3.0/YarnSpinner.Compiler/YarnSpinnerLexer.g4)

Generated with [custom ANTLR version](https://github.com/rrevenantt/antlr4rust/releases/tag/antlr4-4.8-2-Rust0.3.0-beta) \
and the command `java -jar "path/to/antlr4-4.8-2-SNAPSHOT-complete.jar" -Dlanguage=Rust .\YarnSpinnerLexer.g4`

## Adjustments
* `YarnSpinnerParserParserContext` replaced with `YarnSpinnerParserContext` - no idea what happened there during generation
* Autoformat files