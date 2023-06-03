use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn ui_updating_plugin(app: &mut App) {
    app.add_system(foo.after(YarnSlingerSystemSet));
}

fn foo() {}
