use crate::assets::{font_handle, image_handle};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;

pub(crate) fn ui_setup_plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

/// Marker for the [`Node`] that is the root of the UI
#[derive(Debug, Default, Component)]
pub struct UiRootNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNameNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueContinueNode;

#[derive(Debug, Default, Component)]
pub(crate) struct OptionsNode;

#[derive(Debug, Component)]
pub(crate) struct OptionButton(pub OptionId);

fn setup(mut commands: Commands) {
    // root node
    commands
        .spawn((
            fmt_name("root"),
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::End,
                    justify_content: JustifyContent::SpaceAround,
                    grid_auto_flow: GridAutoFlow::Row,
                    grid_template_columns: vec![RepeatedGridTrack::minmax(
                        1,
                        MinTrackSizingFunction::Auto,
                        MaxTrackSizingFunction::Px(DIALOG_WIDTH),
                    )],
                    grid_auto_rows: vec![GridTrack::min_content()],
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            UiRootNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                fmt_name("name"),
                TextBundle {
                    text: Text::from_section(String::new(), text_style::name()),
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(TEXT_BORDER_HORIZONTAL / 2.0),
                            bottom: Val::Px(-8.0),
                            ..default()
                        },
                        ..default()
                    },
                    z_index: ZIndex::Local(1),
                    ..default()
                },
                DialogueNameNode,
                Label,
            ));

            parent
                .spawn((
                    fmt_name("dialogue"),
                    NodeBundle {
                        style: Style {
                            min_height: Val::Px(50.0 + TEXT_BORDER_VERTICAL * 2.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::FlexStart,
                            padding: UiRect {
                                top: Val::Px(TEXT_BORDER_VERTICAL),
                                bottom: Val::Px(TEXT_BORDER_VERTICAL + 10.),
                                left: Val::Px(TEXT_BORDER_HORIZONTAL),
                                right: Val::Px(TEXT_BORDER_HORIZONTAL),
                            },
                            ..default()
                        },
                        background_color: Color::BLACK.with_alpha(0.8).into(),
                        border_radius: BorderRadius::all(Val::Px(20.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Dialog itself
                    parent.spawn((
                        fmt_name("text"),
                        TextBundle::from_section(String::new(), text_style::standard())
                            .with_style(style::standard()),
                        DialogueNode,
                        Label,
                    ));
                })
                .with_children(|parent| {
                    // Options
                    parent.spawn((
                        fmt_name("options"),
                        NodeBundle {
                            style: Style {
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::FlexStart,
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        OptionsNode,
                    ));
                });

            parent
                .spawn((
                    fmt_name("bottom"),
                    NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        fmt_name("continue indicator image"),
                        ImageBundle {
                            image: UiImage {
                                // 27 x 27 pixels
                                texture: image_handle::CONTINUE_INDICATOR,
                                ..default()
                            },
                            style: Style {
                                margin: UiRect {
                                    top: Val::Px(-18.),
                                    bottom: Val::Px(25.),
                                    ..default()
                                },
                                ..default()
                            },
                            z_index: ZIndex::Local(1),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        DialogueContinueNode,
                    ));
                });
        });
}

fn fmt_name(name: &str) -> Name {
    Name::new(format!("Yarn Spinner example dialogue view {name} node"))
}

pub(crate) fn create_dialog_text(text: impl Into<String>, invisible: impl Into<String>) -> Text {
    Text::from_sections([
        TextSection {
            value: text.into(),
            style: text_style::standard(),
        },
        TextSection {
            value: invisible.into(),
            style: TextStyle {
                color: Color::NONE,
                ..text_style::standard()
            },
        },
    ])
}

pub(crate) fn spawn_options<'a, T>(entity_commands: &mut EntityCommands, options: T)
where
    T: IntoIterator<Item = &'a DialogueOption>,
    <T as IntoIterator>::IntoIter: 'a,
{
    entity_commands.with_children(|parent| {
        for (i, option) in options.into_iter().enumerate() {
            parent
                .spawn((
                    fmt_name("option button"),
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        image: UiImage::default().with_color(Color::NONE),
                        ..default()
                    },
                    OptionButton(option.id),
                ))
                .with_children(|parent| {
                    let sections = [
                        TextSection {
                            value: format!("{}: ", i + 1),
                            style: text_style::option_id(),
                        },
                        TextSection {
                            value: option.line.text.clone(),
                            style: text_style::option_text(),
                        },
                    ];

                    parent.spawn((
                        fmt_name("option text"),
                        TextBundle::from_sections(sections).with_style(style::options()),
                        Label,
                    ));
                });
        }
    });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const TEXT_BORDER_HORIZONTAL: f32 = 120.0;
const TEXT_BORDER_VERTICAL: f32 = 30.0;

mod style {
    use super::*;
    pub(crate) fn standard() -> Style {
        Style {
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER_HORIZONTAL),
            ..default()
        }
    }
    pub(crate) fn options() -> Style {
        const INDENT_MODIFIER: f32 = 1.0;
        Style {
            margin: UiRect::horizontal(Val::Px((INDENT_MODIFIER - 1.0) * TEXT_BORDER_HORIZONTAL)),
            max_width: Val::Px(DIALOG_WIDTH - 2.0 * INDENT_MODIFIER * TEXT_BORDER_HORIZONTAL),
            ..default()
        }
    }
}

mod text_style {
    use super::*;
    use bevy::color::palettes::css;
    pub(crate) fn standard() -> TextStyle {
        TextStyle {
            font: font_handle::MEDIUM,
            font_size: 20.0,
            color: Color::WHITE,
        }
    }
    pub(crate) fn name() -> TextStyle {
        TextStyle {
            font: font_handle::MEDIUM,
            font_size: 18.0,
            ..standard()
        }
    }

    pub(crate) fn option_id() -> TextStyle {
        TextStyle {
            font: font_handle::MEDIUM,
            color: css::ALICE_BLUE.into(),
            ..option_text()
        }
    }

    pub(crate) fn option_text() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            color: css::TOMATO.into(),
            ..standard()
        }
    }
}
