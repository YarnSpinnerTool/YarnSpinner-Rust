use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct ExampleYarnSlingerUiPlugin;

impl ExampleYarnSlingerUiPlugin {
    pub fn new() -> Self {
        Self::default()
    }
}

mod font_handle {
    use bevy::prelude::*;
    use bevy::reflect::TypeUuid;

    pub(crate) const MEDIUM: HandleUntyped =
        HandleUntyped::weak_from_u64(Font::TYPE_UUID, 2263821398159872327);

    pub(crate) const BOLD: HandleUntyped =
        HandleUntyped::weak_from_u64(Font::TYPE_UUID, 2165468797133218757);
}

impl Plugin for ExampleYarnSlingerUiPlugin {
    fn build(&self, app: &mut App) {
        use font_handle::{BOLD as FONT_BOLD_HANDLE, MEDIUM as FONT_MEDIUM_HANDLE};
        load_internal_binary_asset!(
            app,
            FONT_MEDIUM_HANDLE,
            "example_ui/assets/FiraMono-Medium.ttf",
            load_font
        );

        load_internal_binary_asset!(
            app,
            FONT_BOLD_HANDLE,
            "example_ui/assets/FiraSans-Bold.ttf",
            load_font
        );
        app.add_system(setup.on_startup());
    }
}

fn load_font(bytes: &[u8]) -> Font {
    Font::try_from_bytes(bytes.to_vec()).unwrap()
}

fn setup(mut commands: Commands) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(70.0)),
                        min_size: Size::height(Val::Px(100.0)),
                        border: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    background_color: Color::rgb(0.4, 0.4, 1.0).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                        ..default()
                    });
                });
        });
}
