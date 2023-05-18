use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn yarn_plugin_panic(In(result): In<SystemResult>) {
    if let Err(e) = result {
        panic!("Error in Yarn Slinger plugin: {e}");
    }
}
