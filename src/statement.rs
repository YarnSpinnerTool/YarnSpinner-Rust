/// ```txt
/// statement
///     : line_statement
///     | if_statement
///     | set_statement
///     | shortcut_option_statement
///     | call_statement
///     | command_statement
///     | declare_statement
///     | jump_statement
///     | INDENT statement* DEDENT
///     ;
/// ```
#[derive(Debug, PartialEq)]
pub struct Statement<'a> {
    // TODO: all variants
    pub line_statement: &'a str,
}
