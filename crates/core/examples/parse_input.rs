use rusty_yarn_spinner::parser::parse;

fn main() {
    let input = "# hello comment any hashtag content\ntitle: blub i am going until the end\n---\nHello Line of Text\n===\n";
    let (hashtags, dialogue) = parse(input);
    println!("File-Hashtags: {:?}", hashtags);
    println!("Dialogue: {}", dialogue);
}
