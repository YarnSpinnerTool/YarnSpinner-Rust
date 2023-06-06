use crate::assets::{font_handle, image_handle};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;

pub(crate) fn ui_setup_plugin(app: &mut App) {
    app.add_system(setup.on_startup());
}

#[derive(Debug, Default, Component)]
pub(crate) struct UiRootNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNode;

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
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            UiRootNode,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: UiImage {
                            // 29 pixels high
                            texture: image_handle::DIALOGUE_EDGE.typed(),
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::width(Val::Px(DIALOG_WIDTH)),
                                min_size: Size::height(Val::Px(50.0)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::FlexStart,
                                padding: UiRect::horizontal(Val::Px(TEXT_BORDER)),
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.8).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Dialog itself
                            parent.spawn((
                                TextBundle::from_section(String::new(), text_style::standard())
                                    .with_style(style::standard()),
                                DialogueNode,
                                Label,
                            ));
                        })
                        .with_children(|parent| {
                            // Options
                            parent.spawn((
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
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(ImageBundle {
                                image: UiImage {
                                    // 29 pixels high
                                    texture: image_handle::DIALOGUE_EDGE.typed(),
                                    flip_y: true,
                                    ..default()
                                },
                                style: Style {
                                    size: Size::width(Val::Px(DIALOG_WIDTH)),
                                    ..default()
                                },
                                ..default()
                            });
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ImageBundle {
                                    image: UiImage {
                                        // 27 x 27 pixels
                                        texture: image_handle::DIALOGUE_CONTINUE.typed(),
                                        ..default()
                                    },
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        position: UiRect::bottom(Val::Px(
                                            INITIAL_DIALOGUE_CONTINUE_BOTTOM,
                                        )),
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
        });
}

pub(crate) const INITIAL_DIALOGUE_CONTINUE_BOTTOM: f32 = -5.0;

pub(crate) fn create_dialog_text<'a>(
    name: impl Into<Option<&'a str>>,
    text: impl Into<String>,
    invisible: impl Into<String>,
) -> Text {
    let mut sections = Vec::new();
    if let Some(name) = name.into() {
        sections.push(TextSection {
            value: format!("{name}: "),
            style: text_style::name(),
        });
    }
    sections.extend([
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
    ]);
    Text::from_sections(sections)
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
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    OptionButton(option.id),
                ))
                .with_children(|parent| {
                    let sections = [
                        TextSection {
                            value: format!("{}:  ", i + 1),
                            style: text_style::option_id(),
                        },
                        TextSection {
                            value: option.line.text.clone(),
                            style: text_style::option_text(),
                        },
                    ];

                    parent.spawn((
                        TextBundle::from_sections(sections).with_style(style::options()),
                        Label,
                    ));
                });
        }
    });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.8;
const TEXT_BORDER: f32 = 40.0;

mod style {
    use super::*;
    pub(crate) fn standard() -> Style {
        Style {
            max_size: Size {
                width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER),
                height: Val::Undefined,
            },
            ..default()
        }
    }
    pub(crate) fn options() -> Style {
        Style {
            margin: UiRect::horizontal(Val::Px(TEXT_BORDER)),
            max_size: Size {
                width: Val::Px(DIALOG_WIDTH - 4.0 * TEXT_BORDER),
                height: Val::Undefined,
            },
            ..default()
        }
    }
}

mod text_style {
    use super::*;
    pub(crate) fn standard() -> TextStyle {
        TextStyle {
            font: font_handle::MEDIUM.typed(),
            font_size: 20.0,
            color: Color::WHITE,
        }
    }
    pub(crate) fn name() -> TextStyle {
        TextStyle {
            font: font_handle::BOLD.typed(),
            ..standard()
        }
    }

    pub(crate) fn option_id() -> TextStyle {
        TextStyle {
            font: font_handle::BOLD.typed(),
            color: Color::WHITE,
            ..option_text()
        }
    }

    pub(crate) fn option_text() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            color: Color::TOMATO,
            ..standard()
        }
    }
}
