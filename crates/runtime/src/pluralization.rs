use crate::prelude::Language;
use fixed_decimal::{DoublePrecision, FixedDecimal};
use icu_plurals::{PluralCategory, PluralRuleType};
use icu_plurals::{PluralOperands, PluralRules};

#[derive(Debug)]
pub(crate) struct Pluralization {
    cardinal_rules: PluralRules,
    ordinal_rules: PluralRules,
}

impl Pluralization {
    pub(crate) fn new(language: impl Into<Language>) -> Self {
        let language = language.into();
        let locale = language.0.into();
        let cardinal_rules = PluralRules::try_new(&locale, PluralRuleType::Cardinal).unwrap();
        let ordinal_rules = PluralRules::try_new(&locale, PluralRuleType::Ordinal).unwrap();
        Self {
            cardinal_rules,
            ordinal_rules,
        }
    }

    pub(crate) fn get_cardinal_plural_case(&self, value: f32) -> PluralCategory {
        let value = get_into_plural_operand(value);
        self.cardinal_rules.category_for(value)
    }

    pub(crate) fn get_ordinal_plural_case(&self, value: f32) -> PluralCategory {
        let value = get_into_plural_operand(value);
        self.ordinal_rules.category_for(value)
    }
}

fn get_into_plural_operand(value: f32) -> PluralOperands {
    let rounded = value.round();
    let floating_point = (rounded - value).abs();
    if floating_point < 1e-5 {
        (value as isize).into()
    } else {
        (&FixedDecimal::try_from_f64(value as f64, DoublePrecision::Floating).unwrap()).into()
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

        for (locale, value, expected_category) in cardinal_tests.into_iter() {
            let result = Pluralization::new(locale).get_cardinal_plural_case(value);
            assert_eq!(
                expected_category, result,
                "locale: {locale}, value: {value}, type: Cardinal"
            );
        }

        for (locale, value, expected_category) in ordinal_tests.into_iter() {
            let result = Pluralization::new(locale).get_ordinal_plural_case(value);
            assert_eq!(
                expected_category, result,
                "locale: {locale}, value: {value}, type: Ordinal"
            );
        }
    }
}
