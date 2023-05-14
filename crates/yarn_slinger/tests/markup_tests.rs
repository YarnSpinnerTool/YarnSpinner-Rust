//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/MarkupTests.cs>

use test_base::prelude::*;

mod test_base;

#[test]
fn test_markup_parsing() {
    let line = "A [b]B[/b]";
    let markup = TestBase::new().dialogue.parse_markup(line).unwrap();

    assert_eq!("A B", markup.text);
    assert_eq!(1, markup.attributes.len());
    assert_eq!("b", markup.attributes[0].name);
    assert_eq!(2, markup.attributes[0].position);
    assert_eq!(1, markup.attributes[0].length);
}

#[test]
fn test_overlapping_attributes() {
    let line = "[a][b][c]X[/b][/a]X[/c]";
    let markup = TestBase::new().dialogue.parse_markup(line).unwrap();

    assert_eq!(3, markup.attributes.len());
    assert_eq!("a", markup.attributes[0].name);
    assert_eq!("b", markup.attributes[1].name);
    assert_eq!("c", markup.attributes[2].name);
}

#[test]
fn test_text_extraction() {
    let line = "A [b]B [c]C[/c][/b]";
    let markup = TestBase::new().dialogue.parse_markup(line).unwrap();

    assert_eq!("B C", markup.text_for_attribute(&markup.attributes[0]));
    assert_eq!("C", markup.text_for_attribute(&markup.attributes[1]));
}
