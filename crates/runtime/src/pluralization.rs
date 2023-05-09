use fixed_decimal::{DoublePrecision, FixedDecimal};
use icu_locid::Locale;
use icu_plurals::PluralRules;
pub use icu_plurals::{PluralCategory, PluralRuleType};
use icu_provider::DataLocale;

#[derive(Debug, Default)]
pub struct Pluralization {
    locale: Option<DataLocale>,
    rule_type: Option<PluralRuleType>,
    rules: Option<PluralRules>,
}

impl Pluralization {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_locale(&mut self, locale: &str) -> &mut Self {
        let locale: Locale = locale.parse().unwrap();
        let locale: Option<DataLocale> = Some(locale.into());
        if self.locale == locale {
            return self;
        }
        self.locale = locale;
        if let Some(rule_type) = self.rule_type.as_ref() {
            self.rules = Some(
                PluralRules::try_new_unstable(
                    &icu_testdata::unstable(),
                    &self.locale.as_ref().unwrap(),
                    *rule_type,
                )
                .unwrap(),
            );
        }
        self
    }

    pub fn with_parsed_locale(mut self, locale: &str) -> Self {
        self.parse_locale(locale);
        self
    }

    pub fn set_rule_type(&mut self, rule_type: PluralRuleType) -> &mut Self {
        if self.rule_type == Some(rule_type) {
            return self;
        }
        self.rule_type = Some(rule_type);
        if let Some(locale) = self.locale.as_ref() {
            self.rules = Some(
                PluralRules::try_new_unstable(&icu_testdata::unstable(), locale, rule_type)
                    .unwrap(),
            );
        }
        self
    }

    pub fn with_rule_type(mut self, rule_type: PluralRuleType) -> Self {
        self.set_rule_type(rule_type);
        self
    }

    pub fn get_plural_case(&self, value: f32) -> PluralCategory {
        let value = FixedDecimal::try_from_f64(value as f64, DoublePrecision::Floating).unwrap();
        if let Some(rules) = self.rules.as_ref() {
            return rules.category_for(&value);
        } else {
            let uncalled_fns = [
                ("with_rule_type", self.rule_type.is_none()),
                ("parse_locale", self.locale.is_none()),
            ];
            let uncalled_fns = uncalled_fns
                .into_iter()
                .filter_map(|(name, uncalled)| uncalled.then_some(name))
                .collect::<Vec<_>>()
                .join(", ");
            panic!("Pluralization::get_plural_case() called without enough setup. Please call the following functions first: {}", uncalled_fns);
        }
    }
}

#[cfg(test)]
mod tests {
    //! Adapted from `TestNumberPlurals` in <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/LanguageTests.cs>

    use super::*;

    #[test]
    fn test_number_plurals() {
        let cardinal_tests = [
            // English
            ("en", 1.0_f32, PluralCategory::One),
            ("en", 2.0, PluralCategory::Other),
            ("en", 1.1, PluralCategory::Other),
            // Arabic
            ("ar", 0.0, PluralCategory::Zero),
            ("ar", 1.0, PluralCategory::One),
            ("ar", 2.0, PluralCategory::Two),
            ("ar", 3.0, PluralCategory::Few),
            ("ar", 11.0, PluralCategory::Many),
            ("ar", 100.0, PluralCategory::Other),
            ("ar", 0.1, PluralCategory::Other),
            // Polish
            ("pl", 1.0, PluralCategory::One),
            ("pl", 2.0, PluralCategory::Few),
            ("pl", 3.0, PluralCategory::Few),
            ("pl", 4.0, PluralCategory::Few),
            ("pl", 5.0, PluralCategory::Many),
            ("pl", 1.1, PluralCategory::Other),
            // Icelandic
            ("is", 1.0, PluralCategory::One),
            ("is", 21.0, PluralCategory::One),
            ("is", 31.0, PluralCategory::One),
            ("is", 41.0, PluralCategory::One),
            ("is", 51.0, PluralCategory::One),
            ("is", 0.0, PluralCategory::Other),
            ("is", 4.0, PluralCategory::Other),
            ("is", 100.0, PluralCategory::Other),
            ("is", 3.0, PluralCategory::Other),
            ("is", 4.0, PluralCategory::Other),
            ("is", 5.0, PluralCategory::Other),
            // Russian
            ("ru", 1.0, PluralCategory::One),
            ("ru", 2.0, PluralCategory::Few),
            ("ru", 3.0, PluralCategory::Few),
            ("ru", 4.0, PluralCategory::Few),
            ("ru", 5.0, PluralCategory::Many),
            ("ru", 1.1, PluralCategory::Other),
        ];

        let ordinal_tests = [
            // English
            ("en", 1.0, PluralCategory::One),
            ("en", 2.0, PluralCategory::Two),
            ("en", 3.0, PluralCategory::Few),
            ("en", 4.0, PluralCategory::Other),
            ("en", 11.0, PluralCategory::Other),
            ("en", 21.0, PluralCategory::One),
            // Welsh
            ("cy", 0.0, PluralCategory::Zero),
            ("cy", 7.0, PluralCategory::Zero),
            ("cy", 1.0, PluralCategory::One),
            ("cy", 2.0, PluralCategory::Two),
            ("cy", 3.0, PluralCategory::Few),
            ("cy", 4.0, PluralCategory::Few),
            ("cy", 5.0, PluralCategory::Many),
            ("cy", 10.0, PluralCategory::Other),
        ];

        let mut pluralization = Pluralization::new().with_rule_type(PluralRuleType::Cardinal);
        for (locale, value, expected_category) in cardinal_tests.into_iter() {
            let result = pluralization.parse_locale(locale).get_plural_case(value);
            assert_eq!(
                expected_category, result,
                "locale: {locale}, value: {value}, type: Cardinal"
            );
        }

        let mut pluralization = Pluralization::new().with_rule_type(PluralRuleType::Ordinal);
        for (locale, value, expected_category) in ordinal_tests.into_iter() {
            let result = pluralization.parse_locale(locale).get_plural_case(value);
            assert_eq!(
                expected_category, result,
                "locale: {locale}, value: {value}, type: Ordinal"
            );
        }
    }
}
