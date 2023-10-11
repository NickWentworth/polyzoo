use super::{
    tabs::{tab_group, HideTabButton, ShowTabButton},
    theme::{ButtonColor, UiTheme},
    BlockCameraRaycast, UiDisplay,
};
use crate::{
    objects::{BarrierData, PropData},
    placement::{ChangePreview, ClearPreview, PreviewData},
    zoo::{OnZooBalanceChanged, Zoo},
};
use bevy::prelude::*;
use std::sync::Arc;

pub struct ToolbarPlugin;
impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_toolbar).add_systems(
            Update,
            (
                tab_group::<BuyMenu>,
                on_buy_button_press,
                on_preview_change,
                on_zoo_balance_changed,
            ),
        );
    }
}

// TODO - add information panel to buy menu

#[derive(Component)]
pub struct ZooBalanceText;

/// Component for a buy button that stores the event to send when clicked
#[derive(Component)]
pub struct BuyButton {
    on_click_event: ChangePreview,
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
    barriers: Res<Assets<BarrierData>>,
    props: Res<Assets<PropData>>,
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
                    for (handle_id, barrier) in barriers.iter() {
                        buy_button(barrier, barriers.get_handle(handle_id), parent, &theme);
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
                .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
                    // TODO - nature should probably be sorted by biome when its added
                    // sort props by name
                    let mut sorted_props = props.iter().collect::<Vec<_>>();
                    sorted_props.sort_by_key(|(_, prop)| prop.name());

                    for (handle_id, prop) in sorted_props.into_iter() {
                        buy_button(prop, props.get_handle(handle_id), parent, &theme);
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
                                theme.white_text(&format!("${:.0}", zoo.balance()), 18.0),
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
                                .spawn((
                                    theme.dark_button(),
                                    ShowTabButton {
                                        tab: BuyMenu::Build,
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(theme.white_text("Build", 18.0));
                                });

                            parent
                                .spawn((
                                    theme.dark_button(),
                                    ShowTabButton {
                                        tab: BuyMenu::Animal,
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(theme.white_text("Animals", 18.0));
                                });

                            parent
                                .spawn((
                                    theme.dark_button(),
                                    ShowTabButton {
                                        tab: BuyMenu::Nature,
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(theme.white_text("Nature", 18.0));
                                });

                            parent
                                .spawn((theme.dark_button(), HideTabButton::<BuyMenu>::default()))
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

/// Spawns in a buy button built for a given `UiDisplay`-able object with corresponding `PreviewData`
fn buy_button(
    displayable: &impl UiDisplay,
    preview_data: impl PreviewData,
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
            BuyButton {
                on_click_event: ChangePreview {
                    to: Arc::new(preview_data),
                    name: displayable.name(),
                },
            },
        ))
        .with_children(|parent| {
            // placeable object's image
            parent.spawn(ImageBundle {
                image: displayable.image(),
                style: Style {
                    width: Px(80.0),
                    height: Px(80.0),
                    ..default()
                },
                ..default()
            });

            // placeable object's text
            parent.spawn(theme.dark_text(&displayable.text(), 16.0));

            // TEMP - name and other info can be displayed in a side panel, image and text is enough for the button
            parent.spawn(theme.dark_text(&displayable.name(), 16.0));
        })
        .id()
}

pub fn on_buy_button_press(
    buy_buttons: Query<(&Interaction, &BuyButton), Changed<Interaction>>,
    mut change_preview: EventWriter<ChangePreview>,
) {
    for (interaction, button) in buy_buttons.iter() {
        if *interaction == Interaction::Pressed {
            change_preview.send(button.on_click_event.clone());
        }
    }
}

pub fn on_preview_change(
    theme: Res<UiTheme>,
    mut clears: EventReader<ClearPreview>,
    mut changes: EventReader<ChangePreview>,
    mut buy_buttons: Query<(&BuyButton, &mut ButtonColor)>,
) {
    for _ in clears.iter() {
        for (_, mut button_color) in buy_buttons.iter_mut() {
            // revert to default button color if clearing a preview
            button_color.revert();
        }
    }

    for change in changes.iter() {
        for (button, mut button_color) in buy_buttons.iter_mut() {
            // detect if the button was pressed based on if the object's name is the same as the button's stored change event
            // if objects have the same name, this will incorrectly set multiple buy buttons as previewing
            if change.name == button.on_click_event.name {
                button_color.update(theme.accent);
            } else {
                button_color.revert();
            }
        }
    }
}

pub fn on_zoo_balance_changed(
    mut on_balance_changed: EventReader<OnZooBalanceChanged>,
    mut zoo_balance_text: Query<&mut Text, With<ZooBalanceText>>,
) {
    for balance_changed in on_balance_changed.iter() {
        let mut text = zoo_balance_text.single_mut();
        text.sections[0].value = format!("${:.0}", balance_changed.balance);
    }
}
