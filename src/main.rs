use crate::parser::parse;

mod parser;
mod statement;

fn main() {
    let input = "# hello comment any hashtag content\ntitle: blub i am going until the end\n---\nHello Line of Text\n===\n";
    let (hashtags, dialogue) = parse(input);
    println!("File-Hashtags: {:?}", hashtags);
    println!("Dialogue: {}", dialogue);
}

#[cfg(test)]
mod test {
    use crate::parser::{parse, Dialogue, Header, Node};
    use crate::statement::Statement;

    #[test]
    fn yarn_spinner_example() {
        // https://docs.yarnspinner.dev/getting-started/writing-in-yarn/lines-nodes-and-options
        let plain_text_example = "title: Node_Title
---
Here are some lines!
Wow!
===
";
        let (hashtags, dialogue) = parse(plain_text_example);
        assert!(hashtags.is_empty());
        assert_eq!(
            Dialogue {
                nodes: vec![Node {
                    headers: vec![Header {
                        header_key: "title",
                        header_value: "Node_Title"
                    }],
                    body: vec![
                        Statement {
                            line_statement: "Here are some lines!"
                        },
                        Statement {
                            line_statement: "Wow!"
                        }
                    ]
                }]
            },
            dialogue
        );
    }

    #[test]
    fn crlf_works_like_lf() {
        let lf_input = "# hello comment any hashtag content\ntitle: blub i am going until the end\n---\nHello Line of Text\n===\n";
        let crlf_input = lf_input.clone().replace("\n", "\r\n");
        assert_eq!(parse(&lf_input), parse(&crlf_input));
    }
}
