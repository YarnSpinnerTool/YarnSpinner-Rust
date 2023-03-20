use crate::parser::parse;

mod parser;

fn main() {
    println!("{}", parse(r#"# hello comment"#));
}
