use bevy::prelude::*;

pub fn setup_toolbar(mut commands: Commands) {
    use Val::*;

    let white = Color::hex("#E8EDED").unwrap();
    let medium = Color::hex("#4F6367").unwrap();
    let dark = Color::hex("#354345").unwrap();

    let text_style = TextStyle {
        font_size: 18.0,
        color: white,
        ..default()
    };

    let button_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Px(4.0)),
        ..default()
    };

    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Px(0.0),
                left: Px(0.0),
                right: Px(0.0),
                padding: UiRect::all(Px(8.0)),
                display: Display::Flex,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: medium.into(),
            ..default()
        },))
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
                        .spawn(ButtonBundle {
                            style: button_style.clone(),
                            background_color: dark.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Build", text_style.clone()));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: button_style.clone(),
                            background_color: dark.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Animals", text_style.clone()));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: button_style.clone(),
                            background_color: dark.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Nature", text_style.clone()));
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
                    parent.spawn(TextBundle::from_section("100 guests", text_style.clone()));
                });
        });
}
