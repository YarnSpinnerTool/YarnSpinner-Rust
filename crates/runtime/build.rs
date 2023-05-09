use icu::plurals::provider::*;
use icu_datagen::prelude::*;
use icu_provider_adapters::fallback::provider::*;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let mod_directory = PathBuf::from(out_dir).join("icu");

    let mut options = BakedOptions::default();
    // Overwrite the baked data if it was already present:
    options.overwrite = true;
    let should_filter_locales = None;
    icu_datagen::datagen(
        should_filter_locales,
        &[
            // For pluralization
            CardinalV1Marker::KEY,
            OrdinalV1Marker::KEY,
            // For locale fallback provider, i.e. "en-GB" -> "en"
            // Keys can be read from <https://unicode-org.github.io/icu4x/docs/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#impl-LocaleFallbackProvider%3CP%3E>
            LocaleFallbackLikelySubtagsV1Marker::KEY,
            LocaleFallbackParentsV1Marker::KEY,
            CollationFallbackSupplementV1Marker::KEY,
        ],
        &SourceData::default()
            .with_cldr_for_tag(SourceData::LATEST_TESTED_CLDR_TAG, CldrLocaleSubset::Modern)
            .unwrap(),
        vec![Out::Baked {
            mod_directory,
            options,
        }],
    )
    .unwrap();
}
