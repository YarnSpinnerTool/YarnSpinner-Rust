extern crate alloc;

use icu_plurals::provider::{CardinalV1Marker, OrdinalV1Marker};
use icu_provider_adapters::fallback::LocaleFallbackProvider;

struct UnstableProvider;
include!("icu/mod.rs");

impl_data_provider!(UnstableProvider);

pub(crate) fn generate_provider(
) -> impl DataProvider<OrdinalV1Marker> + DataProvider<CardinalV1Marker> {
    LocaleFallbackProvider::try_new_unstable(UnstableProvider).unwrap()
}
