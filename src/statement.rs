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
#[allow(dead_code)]
pub enum Statement<'a> {
    LineStatement(&'a str),
    IfStatement(),
    SetStatement(),
    ShortcutOptionStatement(),
    CallStatement(),
    CommandStatement(),
    DeclareStatement(),
    JumpStatement(),
    SubStatements(),
}
