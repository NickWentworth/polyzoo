use super::{formatted_currency, tabs::TabButton, theme::UiTheme, BlockCameraRaycast};
use crate::{
    props::{Object, Prop, SetPreviewProp, UnsetPreviewProp},
    zoo::{OnZooBalanceChanged, Zoo},
};
use bevy::prelude::*;

// TODO - add information panel to buy menu

#[derive(Component)]
pub struct ZooBalanceText;

/// Component for a buy button to store the contained object
#[derive(Component)]
pub struct BuyButton<P: Prop> {
    handle: Handle<P>,
}

/// Tab enum for different buy menus accessible from the toolbar
#[derive(Component, PartialEq)]
pub(super) enum BuyMenu {
    Build,
    Animal,
    Nature,
}

pub(super) fn setup_toolbar(
    mut commands: Commands,
    objects: Res<Assets<Object>>,
    // barriers: Res<Assets<Barrier>>,
    zoo: Res<Zoo>,
    theme: Res<UiTheme>,
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
                    // TODO - add barriers back in
                    // for (handle_id, object) in barriers.iter() {
                    //     buy_button(object, objects.get_handle(handle_id), parent, &theme);
                    // }
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
                    // TODO - nature should probably be sorted by biome when its added
                    // sort objects by name
                    let mut sorted_objects = objects.iter().collect::<Vec<_>>();
                    sorted_objects.sort_by_key(|(_, object)| object.name());

                    for (handle_id, object) in sorted_objects.into_iter() {
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
                            parent.spawn((
                                theme.white_text(&formatted_currency(zoo.balance()), 18.0),
                                ZooBalanceText,
                            ));
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
fn buy_button<P: Prop>(
    prop: &P,
    handle: Handle<P>,
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
            BuyButton { handle },
        ))
        .with_children(|parent| {
            // barrier image
            parent.spawn(ImageBundle {
                image: prop.icon().clone().into(),
                style: Style {
                    width: Px(80.0),
                    height: Px(80.0),
                    ..default()
                },
                ..default()
            });

            // barrier cost
            parent.spawn(theme.dark_text(&formatted_currency(prop.cost()), 16.0));

            // TEMP - name and other info can be displayed in a side panel, image and cost is enough for the button
            parent.spawn(theme.dark_text(prop.name(), 16.0));
        })
        .id()
}

pub fn toolbar_interactions<P: Prop>(
    buy_buttons: Query<(&Interaction, &BuyButton<P>), Changed<Interaction>>,
    mut set_preview: EventWriter<SetPreviewProp<P>>,
    mut unset_preview: EventWriter<UnsetPreviewProp>,
) {
    for (interaction, button) in buy_buttons.iter() {
        if *interaction == Interaction::Pressed {
            // TODO - add a system param for "ChangePreview<P>" that unsets then sets
            unset_preview.send(UnsetPreviewProp);
            set_preview.send(SetPreviewProp {
                handle: button.handle.clone(),
            });
        }
    }
}

pub fn toolbar_callbacks(
    mut on_balance_changed: EventReader<OnZooBalanceChanged>,
    mut zoo_balance_text: Query<&mut Text, With<ZooBalanceText>>,
) {
    for balance_changed in on_balance_changed.iter() {
        let mut text = zoo_balance_text.single_mut();
        text.sections[0].value = formatted_currency(balance_changed.balance);
    }
}
