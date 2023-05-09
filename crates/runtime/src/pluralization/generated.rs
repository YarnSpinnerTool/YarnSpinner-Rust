extern crate alloc;

pub struct UnstableProvider;
include!("./generated_output/mod.rs");
impl_data_provider!(UnstableProvider);
