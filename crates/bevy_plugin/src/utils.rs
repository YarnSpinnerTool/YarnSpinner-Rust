use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn panic_on_err(In(result): In<SystemResult>) {
    if let Err(e) = result {
        panic!("Error in Yarn Spinner plugin: {e}");
    }
}

pub(crate) fn log_error(In(result): In<SystemResult>) {
    if let Err(e) = result {
        error!("Error in Yarn Spinner plugin: {e}");
    }
}
