use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
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
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: font_handle::MEDIUM.typed(),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::width(Val::Px(200.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        TextBundle::from_section(
                            "Scrolling list",
                            TextStyle {
                                font: font_handle::BOLD.typed(),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            size: Size::height(Val::Px(25.)),
                            ..default()
                        }),
                        Label,
                    ));
                    // List with hidden overflow
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                size: Size::height(Val::Percent(50.0)),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            background_color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            flex_direction: FlexDirection::Column,
                                            max_size: Size::UNDEFINED,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                ))
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn((
                                            TextBundle::from_section(
                                                format!("Item {i}"),
                                                TextStyle {
                                                    font: font_handle::BOLD.typed(),
                                                    font_size: 20.,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            Label,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ));
                                    }
                                });
                        });
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(210.0),
                            bottom: Val::Px(10.0),
                            ..default()
                        },
                        border: UiRect::all(Val::Px(20.0)),
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
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                ..default()
                            },
                            background_color: Color::rgb(1.0, 0.0, 0.0).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    // Take the size of the parent node.
                                    size: Size::all(Val::Percent(100.)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::all(Val::Percent(100.)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(40.0),
                                        bottom: Val::Px(40.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::rgb(1.0, 0.5, 0.5).into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::all(Val::Percent(100.)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(60.0),
                                        bottom: Val::Px(60.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::rgb(1.0, 0.7, 0.7).into(),
                                ..default()
                            });
                            // alpha test
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::all(Val::Percent(100.)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(80.0),
                                        bottom: Val::Px(80.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                ..default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                size: Size::width(Val::Px(500.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // alt text
                            parent
                                .spawn(TextBundle::from_section("Bevy logo", TextStyle::default()));
                        });
                });
        });
}
