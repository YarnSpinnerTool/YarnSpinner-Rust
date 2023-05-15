//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/5944b0e03d319303cd185b08140772a5804a2762/Runtime/DialogueRunner.cs#L1169>

use crate::markup::normalize;
use yarn_slinger_core::prelude::YarnValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    name: Option<String>,
    parameters: Vec<YarnValue>,
    raw: String,
}

impl Command {
    pub(crate) fn parse(input: String) -> Self {
        todo!()
    }
}

/// Splits input into a number of non-empty sub-strings, separated
/// by whitespace, and grouping double-quoted strings into a single
/// sub-string.
///
/// This method behaves similarly to the [`String::split`] method with
/// the empty results filtered out, with the following differences:
///
/// - Text that appears inside a pair of double-quote characters will not be split.
/// - Text that appears after a double-quote character and
/// before the end of the input will not be split (that is, an
/// unterminated double-quoted string will be treated as though it
/// had been terminated at the end of the input.)
/// - When inside a pair of double-quote characters, the string
/// `\\` will be converted to `\`, and the string `\"` will be converted to `"`.
fn split_command_text(input: &str) -> Vec<String> {
    let input = normalize(input);
    let mut chars = input.chars().peekable();
    let mut results = Vec::new();
    let mut current_component = String::new();
    while let Some(mut char) = chars.next() {
        match char {
            _ if char.is_whitespace() => {
                if !current_component.is_empty() {
                    // We've reached the end of a run of visible
                    // characters. Add this run to the result list and
                    // prepare for the next one.
                    results.push(std::mem::take(&mut current_component));
                } else {
                    // We encountered a whitespace character, but
                    // didn't have any characters queued up. Skip this
                    // character.
                }
            }
            '\"' => {
                // We've entered a quoted string!
                loop {
                    char = match chars.next() {
                        Some(c) => c,
                        None => {
                            // Oops, we ended the input while parsing a
                            // quoted string! Dump our current word
                            // immediately and return.
                            results.push(current_component);
                            return results;
                        }
                    };
                    match char {
                        '\\' => {
                            // Possibly an escaped character!
                            match chars.peek() {
                                Some('\\') | Some('\"') => {
                                    // It's an escaped character! Consume it and add it to the current component.
                                    let next = chars.next().unwrap();
                                    current_component.push(next);
                                }
                                _ => {
                                    // Oops, an invalid escape. Add the \ and
                                    // whatever is after it.
                                    current_component.push(char);
                                }
                            }
                        }
                        '\"' => {
                            // The end of a string!
                            break;
                        }
                        _ => {
                            // Any other character. Add it to the buffer.
                            current_component.push(char);
                        }
                    }
                }
                results.push(std::mem::take(&mut current_component));
            }
            _ => {
                current_component.push(char);
            }
        }
    }
    if !current_component.is_empty() {
        results.push(current_component);
    }
    results
}

#[cfg(test)]
mod tests {
    //! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/5944b0e03d319303cd185b08140772a5804a2762/Tests/Runtime/DialogueRunnerTests/DialogueRunnerTests.cs#L465>
    use super::*;

    #[test]
    fn split_command_text_splits_text_correctly() {
        for (input, expected_components) in [
            ("one two three four", vec!["one", "two", "three", "four"]),
            ("one \"two three\" four", vec!["one", "two three", "four"]),
            ("one \"two three four", vec!["one", "two three four"]),
            (
                "one \"two \\\"three\" four",
                vec!["one", "two \"three", "four"],
            ),
            (
                "one \\two three four",
                vec!["one", "\\two", "three", "four"],
            ),
            (
                "one \"two \\\\ three\" four",
                vec!["one", "two \\ three", "four"],
            ),
            (
                "one \"two \\1 three\" four",
                vec!["one", "two \\1 three", "four"],
            ),
            ("one      two", vec!["one", "two"]),
        ] {
            let parsed_components = split_command_text(input);

            assert_eq!(expected_components, parsed_components);
        }
    }
}
