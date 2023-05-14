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

#[test]
fn test_attribute_removal() {
    // A test string with the following attributes:
    // a: Covers the entire string
    // b: Starts outside X, ends inside
    // c: Same start and end point as X
    // d: Starts inside X, ends outside
    // e: Starts and ends outside X
    let line = "[a][b]A [c][X]x[/b] [d]x[/X][/c] B[/d] [e]C[/e][/a]";
    let original_markup = TestBase::new().dialogue.parse_markup(line).unwrap();

    // Remove the "X" attribute
    assert_eq!("X", original_markup.attributes[3].name);
    let trimmed_markup = original_markup.delete_range(&original_markup.attributes[3]);

    assert_eq!("A x x B C", original_markup.text);
    assert_eq!(6, original_markup.attributes.len());

    assert_eq!("A  B C", trimmed_markup.text);
    assert_eq!(4, trimmed_markup.attributes.len());

    assert_eq!("a", trimmed_markup.attributes[0].name);
    assert_eq!(0, trimmed_markup.attributes[0].position);
    assert_eq!(6, trimmed_markup.attributes[0].length);

    assert_eq!("b", trimmed_markup.attributes[1].name);
    assert_eq!(0, trimmed_markup.attributes[1].position);
    assert_eq!(2, trimmed_markup.attributes[1].length);

    // "c" will have been removed along with "X" because it had a
    // length of >0 before deletion, and was reduced to zero characters

    assert_eq!("d", trimmed_markup.attributes[2].name);
    assert_eq!(2, trimmed_markup.attributes[2].position);
    assert_eq!(2, trimmed_markup.attributes[2].length);

    assert_eq!("e", trimmed_markup.attributes[3].name);
    assert_eq!(5, trimmed_markup.attributes[3].position);
    assert_eq!(1, trimmed_markup.attributes[3].length);
}
