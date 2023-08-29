use crate::objects::{ChangePlacementObject, Object};
use bevy::prelude::*;

use super::theme::UiTheme;

#[derive(Component)]
pub struct PlacementCloseMessage;

/// Show a small prompt at the top of the screen to show the selected object and the hotkey to deselect it
pub fn setup_deselect_prompt(mut commands: Commands, theme: Res<UiTheme>) {
    use Val::*;

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Px(0.0),
                left: Px(0.0),
                right: Px(0.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                theme
                    .white_text("", 16.0)
                    .with_text_alignment(TextAlignment::Center),
                PlacementCloseMessage,
            ));
        });
}

pub fn deselect_prompt_interactions(
    keys: Res<Input<KeyCode>>,
    mut preview_changes: EventWriter<ChangePlacementObject>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        preview_changes.send(ChangePlacementObject { object: None });
    }
}

pub fn deselect_prompt_callbacks(
    objects: Res<Assets<Object>>,
    mut preview_changes: EventReader<ChangePlacementObject>,
    mut placement_message: Query<&mut Text, With<PlacementCloseMessage>>,
) {
    for preview_change in preview_changes.iter() {
        placement_message.single_mut().sections[0].value = match &preview_change.object {
            Some(handle) => {
                let object = objects.get(handle).unwrap();
                format!(
                    "Currently placing: {}\nPress [esc] to deselect",
                    object.name
                )
            }
            None => String::from(""),
        }
    }
}
