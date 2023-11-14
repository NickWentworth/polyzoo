use super::theme::UiTheme;
use crate::objects::{
    DeselectObject, Object, SelectObject, SellSelectedObject, TransformGizmoSelectedObject,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct SelectionPanel;
#[derive(Component)]
pub struct DeselectButton;
#[derive(Component)]
pub struct SellButton;
#[derive(Component)]
pub struct MoveButton;

pub fn on_object_selection(
    mut commands: Commands,
    theme: Res<UiTheme>,
    asset_server: Res<AssetServer>,

    objects: Res<Assets<Object>>,
    mut object_selections: EventReader<SelectObject>,
    mut object_deselections: EventReader<DeselectObject>,
    selection_panel: Query<Entity, With<SelectionPanel>>,
) {
    use Val::*;

    for selection in object_selections.iter() {
        // remove the current object selection panel
        if let Ok(selection_panel_entity) = selection_panel.get_single() {
            commands.entity(selection_panel_entity).despawn_recursive();
        }

        // update the panel based on selected object
        let object = objects.get(&selection.object).unwrap();

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Px(0.0),
                        right: Px(0.0),
                        margin: UiRect::all(Px(16.0)),
                        flex_direction: FlexDirection::Column,
                        min_width: Px(200.0),
                        ..default()
                    },
                    background_color: theme.medium.into(),
                    ..default()
                },
                SelectionPanel,
            ))
            .with_children(|parent| {
                // header
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            padding: UiRect::all(Px(4.0)),
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: theme.dark.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // object name
                        parent.spawn(theme.white_text(object.name, 18.0));

                        // close button
                        theme.spawn_light_icon_button(
                            parent,
                            Px(18.0),
                            asset_server.load("icons/close.png").into(),
                            DeselectButton,
                        );
                    });

                // TODO: body displaying further information based on object
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            padding: UiRect::all(Px(4.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            theme.white_text(&format!("Entity ID: {:?}", selection.entity), 16.0),
                        );
                    });

                // bottom buttons
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::End,
                            padding: UiRect::all(Px(4.0)),
                            column_gap: Px(4.0),
                            ..default()
                        },
                        background_color: theme.dark.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // move
                        theme.spawn_light_icon_button(
                            parent,
                            Px(18.0),
                            asset_server.load("icons/move.png").into(),
                            MoveButton,
                        );

                        // sell
                        theme.spawn_light_icon_button(
                            parent,
                            Px(18.0),
                            asset_server.load("icons/trash.png").into(),
                            SellButton,
                        );
                    });
            });
    }

    for _ in object_deselections.iter() {
        // remove the current object selection panel
        if let Ok(selection_panel_entity) = selection_panel.get_single() {
            commands.entity(selection_panel_entity).despawn_recursive();
        }
    }
}

pub fn selection_panel_interactions(
    deselect_button: Query<&Interaction, (Changed<Interaction>, With<DeselectButton>)>,
    mut object_deselections: EventWriter<DeselectObject>,

    move_button: Query<&Interaction, (Changed<Interaction>, With<MoveButton>)>,
    mut object_moves: EventWriter<TransformGizmoSelectedObject>,

    sell_button: Query<&Interaction, (Changed<Interaction>, With<SellButton>)>,
    mut object_sells: EventWriter<SellSelectedObject>,
) {
    // deselect button will deselect the current object
    for interaction in deselect_button.iter() {
        if *interaction == Interaction::Pressed {
            object_deselections.send(DeselectObject);
        }
    }

    // move button allows the object to be translated, rotated, scaled (maybe)
    for interaction in move_button.iter() {
        if *interaction == Interaction::Pressed {
            object_moves.send(TransformGizmoSelectedObject);
        }
    }

    // sell button allows the object to be sold
    for interaction in sell_button.iter() {
        if *interaction == Interaction::Pressed {
            object_sells.send(SellSelectedObject);
        }
    }
}
