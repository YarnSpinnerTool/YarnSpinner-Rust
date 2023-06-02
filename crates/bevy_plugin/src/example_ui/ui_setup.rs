use crate::example_ui::assets::font_handle;
use bevy::prelude::*;

pub(crate) fn ui_setup_plugin(app: &mut App) {
    app.add_system(setup.on_startup());
}

fn setup(mut commands: Commands) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            const WIDTH: f32 = 800.0 * 0.7;
            const TEXT_BORDER: f32 = 10.0;
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(WIDTH)),
                        min_size: Size::height(Val::Px(180.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // text
                    parent.spawn((
                        TextBundle::from_section(
                            "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor.Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor.",
                            TextStyle {
                                font: font_handle::MEDIUM.typed(),
                                font_size: 20.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(TEXT_BORDER)),
                            max_size: Size {
                                width: Val::Px(WIDTH - 2.0 * TEXT_BORDER),
                                height: Val::Undefined,
                            },
                            ..default()
                        }),
                        Label,
                    ));
                });
        });
}
