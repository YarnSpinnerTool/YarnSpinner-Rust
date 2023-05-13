//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/YarnSpinner/Dialogue.cs>, which was split into multiple files.

use crate::markup::AttributeMarkerProcessor;
use crate::pluralization::Pluralization;
use icu_plurals::PluralCategory;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Default, Debug, Clone)]
pub(crate) struct DialogueTextProcessor {
    pub(crate) language_code: Option<String>,
}

impl DialogueTextProcessor {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

/// A regex that matches any `%` as long as it's not preceded by a `\`.
static VALUE_PLACEHOLDER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<!\\)%").unwrap());

impl AttributeMarkerProcessor for DialogueTextProcessor {
    /// Returns the text that should be used to replace the
    /// contents of `marker`.
    ///
    /// ## Panics
    /// Panics when the string contains a `plural` or `ordinal` marker, but the specified value cannot be parsed as a number.
    fn replacement_text_for_marker(
        &mut self,
        marker: &crate::markup::MarkupAttributeMarker,
    ) -> String {
        let value_prop = marker
            .properties
            .get("value")
            .expect("Expected a property \"value\"");
        let value = value_prop.to_string();

        // Apply the "select" marker
        if marker.name.as_ref().unwrap() == "select" {
            let replacement_prop = marker
                .properties
                .get(&value)
                .unwrap_or_else(|| panic!("error: no replacement for {value}"));
            let replacement = replacement_prop.to_string();
            return VALUE_PLACEHOLDER_REGEX
                .replace_all(&replacement, value)
                .to_string();
        }

        // If it's not "select", then it's "plural" or "ordinal"

        let language_code = self.language_code.as_ref()
            .expect("Dialogue locale code is not set. 'plural' and 'ordinal' markers cannot be called unless one is set.");

        // Attempt to parse the value as a float, so we can determine its plural class
        let value_as_float = value
            .parse::<f32>()
            .unwrap_or_else(|_| panic!("Error while pluralising line: '{value}' is not a number"));

        // Implementation note: no need to fiddle with locales here because ICU already does fallbacks for us.

        // I would love to cache this, but `icu_plural::PluralRules` is not `Send` because it contains an `Rc`, so even a mutex can't help here :(
        let plural_case = match marker.name.as_ref().unwrap().as_str() {
            "plural" => Pluralization::new(language_code).get_cardinal_plural_case(value_as_float),
            "ordinal" => Pluralization::new(language_code).get_ordinal_plural_case(value_as_float),
            _ => panic!("Invalid marker name {:?}", marker.name),
        };
        let plural_case_name = plural_case_name(plural_case);

        // Now that we know the plural case, we can select the
        // appropriate replacement text for it
        let replacement_value = marker.properties.get(plural_case_name).unwrap_or_else(|| {
            panic!("error: no replacement for {value}'s plural case of {plural_case_name}")
        });
        let input = replacement_value.to_string();
        VALUE_PLACEHOLDER_REGEX
            .replace_all(&input, value)
            .to_string()
    }

    fn set_language_code(&mut self, language_code: String) {
        self.language_code.replace(language_code);
    }

    fn clone_box(&self) -> Box<dyn AttributeMarkerProcessor> {
        Box::new(self.clone())
    }
}

fn plural_case_name(plural_case: PluralCategory) -> &'static str {
    match plural_case {
        PluralCategory::Zero => "Zero",
        PluralCategory::One => "One",
        PluralCategory::Two => "Two",
        PluralCategory::Few => "Few",
        PluralCategory::Many => "Many",
        PluralCategory::Other => "Other",
    }
}
