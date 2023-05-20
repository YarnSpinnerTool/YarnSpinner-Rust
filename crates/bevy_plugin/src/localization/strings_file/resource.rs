use crate::localization::strings_file::StringsFile;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub(crate) fn strings_file_resource_plugin(app: &mut App) {
    app.register_type::<LanguagesToStringsFiles>()
        .init_resource::<LanguagesToStringsFiles>();
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct LanguagesToStringsFiles(pub(crate) HashMap<Language, Handle<StringsFile>>);

impl LanguagesToStringsFiles {
    pub(crate) fn get_language(&self, handle: &Handle<StringsFile>) -> Option<&Language> {
        self.0
            .iter()
            .find_map(|(lang, h)| (h == handle).then_some(lang))
    }
}
