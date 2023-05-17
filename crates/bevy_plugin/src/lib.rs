mod assets;
mod config;
mod plugin;

pub mod prelude {
    //! Everything you need to get starting using Yarn Slinger.
    pub use crate::{config::*, plugin::*};
    pub use yarn_slinger::prelude::YarnFile;
}
