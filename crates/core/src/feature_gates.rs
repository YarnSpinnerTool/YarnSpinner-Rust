#[cfg(feature = "bevy")]
pub(crate) use bevy::prelude::{FromReflect, Reflect};
#[cfg(all(feature = "bevy", feature = "serde"))]
pub(crate) use bevy::prelude::{ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "serde")]
pub(crate) use serde::{de::DeserializeOwned, Deserialize, Serialize};

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(all(feature="bevy", feature="serde"))] {
        pub trait FeatureTraits: Reflect + FromReflect + Serialize + DeserializeOwned {}
        impl<T> FeatureTraits for T where T: Reflect + FromReflect + Serialize + DeserializeOwned {}
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
