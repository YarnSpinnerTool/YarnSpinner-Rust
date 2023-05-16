#[cfg(feature = "bevy")]
pub(crate) use bevy::prelude::{FromReflect, Reflect};
#[cfg(feature = "serde")]
pub(crate) use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(all(feature="bevy", feature="serde"))] {
        pub trait FeatureTraits<'de>: Reflect + FromReflect + Serialize + Deserialize<'de> {}
        impl<'de, T> FeatureTraits<'de> for T where T: Reflect + FromReflect + Serialize + Deserialize<'de> {}
    }
    else if #[cfg(feature="bevy")] {
        pub trait FeatureTraits: Reflect + FromReflect {}
        impl<T> FeatureTraits for T where T: Reflect + FromReflect {}
    }
    else {
        pub trait FeatureTraits {}
        impl<T> FeatureTraits for T {}
    }
}
