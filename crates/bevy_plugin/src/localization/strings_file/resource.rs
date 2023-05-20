use crate::localization::strings_file::StringsFile;
use bevy::prelude::*;

pub(crate) fn strings_file_resource_plugin(app: &mut App) {
    app.register_type::<CurrentStringsFile>()
        .init_resource::<CurrentStringsFile>();
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct CurrentStringsFile(pub(crate) Option<Handle<StringsFile>>);
