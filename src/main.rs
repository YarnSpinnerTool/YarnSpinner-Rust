use crate::parser::parse;

mod parser;

fn main() {
    // TODO: allow non-newline at EOF
    let input = "# hellocomment\ntitle:\n";
    println!("{}", parse(input));
}
