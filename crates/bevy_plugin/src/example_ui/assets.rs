use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

pub(crate) fn ui_assets_plugin(app: &mut App) {
    use font_handle::{BOLD as FONT_BOLD_HANDLE, MEDIUM as FONT_MEDIUM_HANDLE};
    load_internal_binary_asset!(
        app,
        FONT_MEDIUM_HANDLE,
        "assets/FiraMono-Medium.ttf",
        load_font
    );

    load_internal_binary_asset!(app, FONT_BOLD_HANDLE, "assets/FiraSans-Bold.ttf", load_font);
}

fn load_font(bytes: &[u8]) -> Font {
    Font::try_from_bytes(bytes.to_vec()).unwrap()
}

pub(crate) mod font_handle {
    use bevy::prelude::*;
    use bevy::reflect::TypeUuid;

    pub(crate) const MEDIUM: HandleUntyped =
        HandleUntyped::weak_from_u64(Font::TYPE_UUID, 2263821398159872327);

    pub(crate) const BOLD: HandleUntyped =
        HandleUntyped::weak_from_u64(Font::TYPE_UUID, 2165468797133218757);
}
