use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageType};

pub(crate) fn ui_assets_plugin(app: &mut App) {
    use font_handle::{BOLD as FONT_BOLD_HANDLE, MEDIUM as FONT_MEDIUM_HANDLE};
    load_internal_binary_asset!(
        app,
        FONT_MEDIUM_HANDLE,
        "assets/FiraMono-Medium.ttf",
        load_font
    );

    load_internal_binary_asset!(app, FONT_BOLD_HANDLE, "assets/FiraSans-Bold.ttf", load_font);

    use image_handle::{DIALOGUE_EDGE as DIALOGUE_EDGE_HANDLE, DIALOGUE_CONTINUE as DIALOGUE_CONTINUE_HANDLE};
    load_internal_binary_asset!(
        app,
        DIALOGUE_EDGE_HANDLE,
        "assets/dialogue_edge.png",
        load_image
    );
    
    load_internal_binary_asset!(
        app,
        DIALOGUE_CONTINUE_HANDLE,
        "assets/dialogue_continue.png",
        load_image
    );
}

fn load_font(bytes: &[u8]) -> Font {
    Font::try_from_bytes(bytes.to_vec()).unwrap()
}
fn load_image(bytes: &[u8]) -> Image {
    const IS_SRGB: bool = true;
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        IS_SRGB,
    )
    .unwrap()
}

pub(crate) mod font_handle {
    use bevy::prelude::*;
    use bevy::reflect::TypeUuid;

    pub(crate) const MEDIUM: HandleUntyped =
        HandleUntyped::weak_from_u64(Font::TYPE_UUID, 2263821398159872327);

    pub(crate) const BOLD: HandleUntyped =
        HandleUntyped::weak_from_u64(Font::TYPE_UUID, 2165468797133218757);
}

pub(crate) mod image_handle {
    use bevy::prelude::*;
    use bevy::reflect::TypeUuid;

    pub(crate) const DIALOGUE_EDGE: HandleUntyped =
        HandleUntyped::weak_from_u64(Image::TYPE_UUID, 8465132165468742313);
    
    pub(crate) const DIALOGUE_CONTINUE: HandleUntyped =
        HandleUntyped::weak_from_u64(Image::TYPE_UUID, 5464879846123416874);
}
