use super::{tabs::TabButton, theme::UiTheme, BlockCameraRaycast};
use crate::{
    objects::{Object, ObjectGroup},
    placement::Placement,
};
use bevy::prelude::*;

// TODO - add information panel to buy menu

/// Component for a buy button to store the contained object
#[derive(Component)]
pub struct BuyButton(Handle<Object>);

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
    objects: Res<Assets<Object>>,
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
                    // map all barriers into purchase buttons
                    for (handle_id, object) in objects
                        .iter()
                        .filter(|(_, obj)| matches!(obj.group, ObjectGroup::Barrier(_)))
                    {
                        buy_button(object, objects.get_handle(handle_id), parent, &theme);
                    }
                });

            // animal menu
            parent
                .spawn((popup_menu.clone(), BuyMenu::Animal))
                .with_children(|parent| {
                    parent.spawn(theme.white_text("Animals", 18.0));
                });

            // nature menu
            parent
                .spawn((popup_menu.clone(), BuyMenu::Nature))
                .with_children(|parent| {
                    for (handle_id, object) in objects
                        .iter()
                        .filter(|(_, obj)| obj.group == ObjectGroup::Rock)
                    {
                        buy_button(object, objects.get_handle(handle_id), parent, &theme);
                    }
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
                            parent.spawn(theme.white_text("$1,000", 18.0));
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
                                .spawn((theme.dark_button(), TabButton(Some(BuyMenu::Build))))
                                .with_children(|parent| {
                                    parent.spawn(theme.white_text("Build", 18.0));
                                });

                            parent
                                .spawn((theme.dark_button(), TabButton(Some(BuyMenu::Animal))))
                                .with_children(|parent| {
                                    parent.spawn(theme.white_text("Animals", 18.0));
                                });

                            parent
                                .spawn((theme.dark_button(), TabButton(Some(BuyMenu::Nature))))
                                .with_children(|parent| {
                                    parent.spawn(theme.white_text("Nature", 18.0));
                                });

                            parent
                                .spawn((theme.dark_button(), TabButton::<BuyMenu>(None)))
                                .with_children(|parent| {
                                    parent.spawn(theme.white_text("X", 18.0));
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
                            parent.spawn(theme.white_text("100 guests", 18.0));
                        });
                });
        });
}

/// Spawns in a buy button built for a given object
fn buy_button(
    object: &Object,
    handle: Handle<Object>,
    parent: &mut ChildBuilder,
    theme: &UiTheme,
) -> Entity {
    use Val::*;

    parent
        .spawn(theme.light_button())
        .insert((
            Style {
                padding: UiRect::all(Px(4.0)),
                row_gap: Px(4.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BuyButton(handle),
        ))
        .with_children(|parent| {
            // barrier image
            parent.spawn(ImageBundle {
                image: object.image.clone().into(),
                style: Style {
                    width: Px(80.0),
                    height: Px(80.0),
                    ..default()
                },
                ..default()
            });

            // barrier cost
            parent.spawn(theme.dark_text(&object.formatted_cost(), 16.0));

            // TEMP - name and other info can be displayed in a side panel, image and cost is enough for the button
            parent.spawn(theme.dark_text(object.name, 16.0));
        })
        .id()
}

pub fn toolbar_interactions(
    buy_buttons: Query<(&Interaction, &BuyButton), Changed<Interaction>>,
    mut placement: ResMut<Placement>,
) {
    for (interaction, BuyButton(object_handle)) in buy_buttons.iter() {
        if *interaction == Interaction::Pressed {
            placement.object = Some(object_handle.clone());
        }
    }
}
