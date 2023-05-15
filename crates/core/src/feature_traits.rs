#[cfg(feature = "bevy")]
use bevy_reflect::{FromReflect, Reflect};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature="bevy")] {
        pub trait FeatureTraits: Reflect {}
        impl<T> FeatureTraits for T where T: Reflect {}
    } else {
        pub trait FeatureTraits {}
        impl<T> FeatureTraits for T {}
    }
}

cfg_if! {
    if #[cfg(feature="bevy")] {
        pub trait SizedFeatureTraits: Reflect + FromReflect {}
        impl<T> SizedFeatureTraits for T where T: Reflect + FromReflect  {}
    } else {
        pub trait SizedFeatureTraits {}
        impl<T> SizedFeatureTraits for T {}
    }
}
