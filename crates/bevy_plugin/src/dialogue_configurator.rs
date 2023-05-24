use crate::prelude::*;

pub trait DialogueConfigurator {
    fn set_text_language(&mut self, language: impl Into<Option<Language>>) {
        self.text_provider_mut().set_language(language.into());
    }
    fn set_asset_language(&mut self, language: impl Into<Option<Language>>) {
        if let Some(asset_provider) = self.asset_provider_mut() {
            asset_provider.set_language(language.into());
        }
    }
    fn set_language(&mut self, language: impl Into<Option<Language>>) {
        let language = language.into();
        self.set_text_language(language.clone());
        self.set_asset_language(language);
    }
    #[must_use]
    fn text_language(&self) -> Option<Language> {
        self.text_provider().get_language()
    }
    #[must_use]
    fn asset_language(&self) -> Option<Language> {
        self.asset_provider()
            .map(|p| p.get_language())
            .unwrap_or_default()
    }
    #[must_use]
    fn lines_available(&self) -> bool {
        self.text_provider().lines_available()
    }
    #[must_use]
    fn assets_available(&self) -> bool {
        self.asset_provider()
            .map(|p| p.assets_available())
            .unwrap_or(true)
    }
    #[must_use]
    fn lines_and_assets_available(&self) -> bool {
        self.lines_available() && self.assets_available()
    }
    #[must_use]
    fn text_provider(&self) -> &dyn TextProvider;
    #[must_use]
    fn text_provider_mut(&mut self) -> &mut dyn TextProvider;
    #[must_use]
    fn asset_provider(&self) -> Option<&dyn AssetProvider>;
    #[must_use]
    fn asset_provider_mut(&mut self) -> Option<&mut dyn AssetProvider>;
    #[must_use]
    fn variable_storage(&self) -> &dyn VariableStorage;
    #[must_use]
    fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage;
    #[must_use]
    fn library(&self) -> &YarnFnLibrary;
    #[must_use]
    fn library_mut(&mut self) -> &mut YarnFnLibrary;
}
