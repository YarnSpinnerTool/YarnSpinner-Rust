use crate::example_ui::assets::font_handle;
use crate::prelude::{DialogueOption, OptionId};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub(crate) fn ui_setup_plugin(app: &mut App) {
    app.add_system(setup.on_startup());
}

#[derive(Debug, Default, Component)]
pub(crate) struct UiRootNode;

#[derive(Debug, Default, Component)]
pub(crate) struct DialogueNode;

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
                ..default()
            },
            UiRootNode,
        ))
        .insert(Visibility::Hidden)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(DIALOG_WIDTH)),
                        min_size: Size::height(Val::Px(150.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::all(Val::Px(TEXT_BORDER)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
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
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::None,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::FlexEnd,
                                    align_items: AlignItems::FlexStart,
                                    margin: UiRect::top(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            OptionsNode,
                        ))
                        .insert(Visibility::Hidden);
                });
        });
}

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
                        TextBundle::from_sections(sections).with_style(style::options()),
                        Label,
                    ));
                });
        }
    });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.7;
const TEXT_BORDER: f32 = 10.0;

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
            color: Color::BLACK,
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
            ..option_text()
        }
    }

    pub(crate) fn option_text() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            color: Color::DARK_GRAY,
            ..standard()
        }
    }
}
