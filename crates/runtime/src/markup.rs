//! Types handling the parsing of Yarn markup.
//! Yarn markup looks like this:
//! ```text
//! Mae: [shout]I'm a cat![/shout]!
//! Greg: You're a [size=12]cat[/size]!
//! ```
//! The parsing extracts the information that "Mae" and "Greg" are characters, that "shout" and "size" are attributes, and that "size" has a value of "12".
mod attribute_marker_processor;
mod line_parser;
mod markup_parse_error;
mod parsed_markup;

pub use self::line_parser::{
    CHARACTER_ATTRIBUTE, CHARACTER_ATTRIBUTE_NAME_PROPERTY, TRIM_WHITESPACE_PROPERTY,
};
pub(crate) use self::{attribute_marker_processor::*, line_parser::*};
pub use self::{markup_parse_error::*, parsed_markup::*};

#[cfg(test)]
mod tests {
    //! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/MarkupTests.cs>
    use super::*;
    use crate::prelude::{Language, Line};

    #[test]
    fn test_markup_parsing() {
        let line = "A [b]B[/b]";
        let markup = line_parser().parse_markup(line).unwrap();

        assert_eq!("A B", markup.text);
        assert_eq!(1, markup.attributes.len());
        assert_eq!("b", markup.attributes[0].name);
        assert_eq!(2, markup.attributes[0].position);
        assert_eq!(1, markup.attributes[0].length);
    }

    #[test]
    fn test_overlapping_attributes() {
        let line = "[a][b][c]X[/b][/a]X[/c]";
        let markup = line_parser().parse_markup(line).unwrap();

        assert_eq!(3, markup.attributes.len());
        assert_eq!("a", markup.attributes[0].name);
        assert_eq!("b", markup.attributes[1].name);
        assert_eq!("c", markup.attributes[2].name);
    }

    #[test]
    fn test_text_extraction() {
        let line = "A [b]B [c]C[/c][/b]";
        let markup = line_parser().parse_markup(line).unwrap();

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
        let original_markup = line_parser().parse_markup(line).unwrap();

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

    #[test]
    fn test_finding_attributes() {
        let line = "A [b]B[/b] [b]C[/b]";
        let markup = line_parser().parse_markup(line).unwrap();

        let attribute = markup.get_attribute("b").unwrap();
        assert_eq!(&attribute, &markup.attributes[0]);
        assert_ne!(&attribute, &markup.attributes[1]);

        assert!(markup.get_attribute("c").is_none());
    }

    #[test]
    fn test_multibyte_character_parsing() {
        for input in [
            "á [á]S[/á]",
            "á [a]á[/a]",
            "á [a]S[/a]",
            "S [á]S[/á]",
            "S [a]á[/a]",
            "S [a]S[/a]",
        ] {
            let markup = line_parser().parse_markup(input).unwrap();

            // All versions of this string should have the same position
            // and length of the attribute, despite the presence of
            // multibyte characters
            assert_eq!(1, markup.attributes.len());
            assert_eq!(2, markup.attributes[0].position);
            assert_eq!(1, markup.attributes[0].length);
        }
    }

    #[test]
    fn test_unexpected_close_marker_throws() {
        for input in ["[a][/a][/b]", "[/b]", "[a][/][/b]"] {
            let markup = line_parser().parse_markup(input);

            assert!(markup.is_err());
        }
    }

    #[test]
    fn test_markup_shortcut_property_parsing() {
        let line = "[a=1]s[/a]";
        let markup = line_parser().parse_markup(line).unwrap();

        // Should have a single attribute, "a", at position 0 and length 1
        assert_eq!(1, markup.attributes.len());

        let attribute = &markup.attributes[0];
        assert_eq!("a", attribute.name);
        assert_eq!(0, attribute.position);
        assert_eq!(1, attribute.length);

        // Should have a single property on this attribute, "a". Value should be an integer, 1
        let value = attribute.properties.get("a").unwrap();

        assert_eq!(&MarkupValue::Integer(1), value);
    }

    #[test]
    fn test_markup_multiple_property_parsing() {
        let line = "[a p1=1 p2=2]s[/a]";
        let markup = line_parser().parse_markup(line).unwrap();

        assert_eq!(1, markup.attributes.len());

        let attribute = &markup.attributes[0];
        assert_eq!("a", attribute.name);
        assert_eq!(2, attribute.properties.len());

        let p1 = attribute.properties.get("p1").unwrap();
        assert_eq!(&MarkupValue::Integer(1), p1);

        let p2 = attribute.properties.get("p2").unwrap();
        assert_eq!(&MarkupValue::Integer(2), p2);
    }

    #[test]
    fn test_markup_property_parsing() {
        for (input, expected_value) in [
            ("[a p=\"string\"]s[/a]", MarkupValue::from("string")),
            ("[a p=\"str\\\"ing\"]s[/a]", "str\"ing".into()),
            ("[a p=string]s[/a]", "string".into()),
            ("[a p=42]s[/a]", 42.into()),
            ("[a p=13.37]s[/a]", 13.37.into()),
            ("[a p=true]s[/a]", true.into()),
            ("[a p=false]s[/a]", false.into()),
        ] {
            let markup = line_parser().parse_markup(input).unwrap();

            let attribute = &markup.attributes[0];
            let property_value = attribute.properties.get("p").unwrap();

            assert_eq!(&expected_value, property_value);
        }
    }

    #[test]
    fn test_multiple_attributes() {
        for input in [
            "A [b]B [c]C[/c][/b] D", // attributes can be closed
            "A [b]B [c]C[/b][/c] D", // attributes can be closed out of order
            "A [b]B [c]C[/] D",      // "[/]" closes all open attributes
        ] {
            let markup = line_parser().parse_markup(input).unwrap();

            assert_eq!("A B C D", markup.text);

            assert_eq!(2, markup.attributes.len());

            assert_eq!("b", markup.attributes[0].name);
            assert_eq!(2, markup.attributes[0].position);
            assert_eq!(2, markup.attributes[0].source_position);
            assert_eq!(3, markup.attributes[0].length);

            assert_eq!("c", markup.attributes[1].name);
            assert_eq!(4, markup.attributes[1].position);
            assert_eq!(7, markup.attributes[1].source_position);
            assert_eq!(1, markup.attributes[1].length);
        }
    }

    #[test]
    fn test_self_closing_attributes() {
        let line = "A [a/] B";
        let markup = line_parser().parse_markup(line).unwrap();

        assert_eq!("A B", markup.text);

        assert_eq!(1, markup.attributes.len());

        assert_eq!("a", markup.attributes[0].name);
        assert!(markup.attributes[0].properties.is_empty());
        assert_eq!(2, markup.attributes[0].position);
        assert_eq!(0, markup.attributes[0].length);
    }

    #[test]
    fn test_attributes_may_trim_trailing_whitespace() {
        for (input, expected_text) in [
            ("A [a/] B", "A B"),
            ("A [a trimwhitespace=true/] B", "A B"),
            ("A [a trimwhitespace=false/] B", "A  B"),
            ("A [nomarkup/] B", "A  B"),
            ("A [nomarkup trimwhitespace=false/] B", "A  B"),
            ("A [nomarkup trimwhitespace=true/] B", "A B"),
        ] {
            let markup = line_parser().parse_markup(input).unwrap();

            assert_eq!(expected_text, markup.text);
        }
    }

    #[test]
    fn test_implicit_character_attribute_parsing() {
        for input in [
            // character attribute can be implicit
            "Mae: Wow!",
            // character attribute can also be explicit
            "[character name=\"Mae\"]Mae: [/character]Wow!",
        ] {
            let markup = line_parser().parse_markup(input).unwrap();

            assert_eq!("Mae: Wow!", markup.text);
            assert_eq!(1, markup.attributes.len());

            let attribute = &markup.attributes[0];
            assert_eq!("character", attribute.name);
            assert_eq!(0, attribute.position);
            assert_eq!(5, attribute.length);

            assert_eq!(1, attribute.properties.len());
            assert_eq!(
                &MarkupValue::String("Mae".to_owned()),
                attribute.properties.get("name").unwrap()
            );
        }
    }

    #[test]
    fn test_no_markup_mode_parsing() {
        let line = "S [a]S[/a] [nomarkup][a]S;][/a][/nomarkup]";
        let markup = line_parser().parse_markup(line).unwrap();

        assert_eq!("S S [a]S;][/a]", markup.text);

        assert_eq!(2, markup.attributes.len());

        assert_eq!("a", markup.attributes[0].name);
        assert_eq!(2, markup.attributes[0].position);
        assert_eq!(1, markup.attributes[0].length);

        assert_eq!("nomarkup", markup.attributes[1].name);
        assert_eq!(4, markup.attributes[1].position);
        assert_eq!(10, markup.attributes[1].length);
    }

    #[test]
    fn test_markup_escaping() {
        let line = r"[a]hello \[b\]hello\[/b\][/a]";
        let markup = line_parser().parse_markup(line).unwrap();

        assert_eq!("hello [b]hello[/b]", markup.text);

        assert_eq!(1, markup.attributes.len());

        assert_eq!("a", markup.attributes[0].name);
        assert_eq!(0, markup.attributes[0].position);
        assert_eq!(18, markup.attributes[0].length);
    }

    #[test]
    fn test_numeric_properties() {
        let line = "[select value=1 1=one 2=two 3=three /]";
        let markup = line_parser().parse_markup(line).unwrap();

        assert_eq!(1, markup.attributes.len());

        let attribute = &markup.attributes[0];

        assert_eq!("select", attribute.name);
        assert_eq!(4, attribute.properties.len());
        assert_eq!(
            &MarkupValue::Integer(1),
            attribute.properties.get("value").unwrap()
        );
        assert_eq!(
            &MarkupValue::String("one".to_owned()),
            attribute.properties.get("1").unwrap()
        );
        assert_eq!(
            &MarkupValue::String("two".to_owned()),
            attribute.properties.get("2").unwrap()
        );
        assert_eq!(
            &MarkupValue::String("three".to_owned()),
            attribute.properties.get("3").unwrap()
        );

        assert_eq!("one", markup.text);
    }

    #[test]
    fn test_number_pluralisation() {
        for (value, locale, expected) in [
            (1, "en", "a single cat"),
            (2, "en", "2 cats"),
            (3, "en", "3 cats"),
            (1, "en-AU", "a single cat"),
            (2, "en-AU", "2 cats"),
            (3, "en-AU", "3 cats"),
        ] {
            let line = format!("[plural value={value} one=\"a single cat\" other=\"% cats\"/]",);

            let mut line_parser = line_parser();
            line_parser.set_language_code(Language::from(locale));
            let markup = line_parser.parse_markup(&line).unwrap();

            assert_eq!(expected, markup.text, "locale: {locale}");
        }
    }

    fn line_parser() -> LineParser {
        let dialogue_text_processor = Box::new(DialogueTextProcessor::new());

        LineParser::new()
            .register_marker_processor("select", dialogue_text_processor.clone())
            .register_marker_processor("plural", dialogue_text_processor.clone())
            .register_marker_processor("ordinal", dialogue_text_processor)
    }

    trait ParsedMarkupAsLine {
        fn as_line(&self) -> Line;

        fn get_attribute(&self, name: &str) -> Option<MarkupAttribute> {
            self.as_line().attribute(name).cloned()
        }

        fn text_for_attribute(&self, attribute: &MarkupAttribute) -> String {
            self.as_line().text_for_attribute(attribute).to_string()
        }

        fn delete_range(&self, attribute_to_delete: &MarkupAttribute) -> ParsedMarkup {
            let line = self.as_line().delete_range(attribute_to_delete);
            ParsedMarkup {
                text: line.text,
                attributes: line.attributes,
            }
        }
    }

    impl ParsedMarkupAsLine for ParsedMarkup {
        fn as_line(&self) -> Line {
            Line {
                id: "test".into(),
                text: self.text.clone(),
                attributes: self.attributes.clone(),
            }
        }
    }
}
