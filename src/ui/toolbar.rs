use super::{tabs::TabButton, BlockCameraRaycast, UiTheme};
use crate::objects::ObjectDatabase;
use bevy::prelude::*;

/// Tab enum for different buy menus accessible from the toolbar
#[derive(Component, PartialEq)]
pub(super) enum BuyMenu {
    Build,
    Animal,
    Nature,
}

pub(super) fn setup_toolbar(
    mut commands: Commands,
    theme: Res<UiTheme>,
    objects: Res<ObjectDatabase>,
) {
    use Val::*;

    // popup windows controlled by toolbar buttons
    let popup_menu = NodeBundle {
        style: Style {
            padding: UiRect::all(Px(8.0)),
            row_gap: Px(8.0),
            column_gap: Px(8.0),
            display: Display::None,
            ..default()
        },
        background_color: theme.dark.into(),
        ..default()
    };

    // default button styling
    let button = ButtonBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Px(4.0)),
            ..default()
        },
        background_color: theme.dark.into(),
        ..default()
    };

    // default text styling
    let text_style = TextStyle {
        font_size: 18.0,
        color: theme.white,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Px(0.0),
                    left: Px(0.0),
                    right: Px(0.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: theme.medium.into(),
                ..default()
            },
            BlockCameraRaycast,
        ))
        .with_children(|parent| {
            // build menu
            parent
                .spawn((popup_menu.clone(), BuyMenu::Build))
                .with_children(|parent| {
                    for barrier in objects.barriers() {
                        parent.spawn(button.clone()).with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section(barrier.name, text_style.clone()));
                        });
                    }
                });

            // animal menu
            parent
                .spawn((popup_menu.clone(), BuyMenu::Animal))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Animals", text_style.clone()));
                });

            // nature menu
            parent
                .spawn((popup_menu.clone(), BuyMenu::Nature))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Nature", text_style.clone()));
                });

            // toolbar
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Px(8.0)),
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: theme.medium.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // currency panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_basis: Percent(33.3),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("$1,000", text_style.clone()));
                        });

                    // buttons panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_basis: Percent(33.3),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                column_gap: Px(4.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((button.clone(), TabButton(Some(BuyMenu::Build))))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Build",
                                        text_style.clone(),
                                    ));
                                });

                            parent
                                .spawn((button.clone(), TabButton(Some(BuyMenu::Animal))))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Animals",
                                        text_style.clone(),
                                    ));
                                });

                            parent
                                .spawn((button.clone(), TabButton(Some(BuyMenu::Nature))))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Nature",
                                        text_style.clone(),
                                    ));
                                });

                            parent
                                .spawn((button.clone(), TabButton::<BuyMenu>(None)))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section("X", text_style.clone()));
                                });
                        });

                    // zoo info panel
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_basis: Percent(33.3),
                                flex_direction: FlexDirection::RowReverse,
                                column_gap: Px(16.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("100 guests", text_style.clone()));
                        });
                });
        });
}
