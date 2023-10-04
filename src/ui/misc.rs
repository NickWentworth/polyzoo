use super::theme::UiTheme;
use crate::placement::{ChangePreview, ClearPreview};
use bevy::prelude::*;

/// Marker component for the message detailing the currently previewed object
#[derive(Component)]
pub struct PreviewMessage;

/// Show a small prompt at the top of the screen to show the previewed object and the hotkey to deselect it
pub fn setup_clear_preview_prompt(mut commands: Commands, theme: Res<UiTheme>) {
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
                PreviewMessage,
            ));
        });
}

pub fn clear_preview_prompt_interactions(
    keys: Res<Input<KeyCode>>,
    mut clear_previews: EventWriter<ClearPreview>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        clear_previews.send(ClearPreview);
    }
}

pub fn clear_preview_prompt_callbacks(
    mut clear_previews: EventReader<ClearPreview>,
    mut change_previews: EventReader<ChangePreview>,
    mut placement_message: Query<&mut Text, With<PreviewMessage>>,
) {
    for _ in clear_previews.iter() {
        placement_message.single_mut().sections[0].value = String::from("");
    }

    for change in change_previews.iter() {
        placement_message.single_mut().sections[0].value = format!(
            "Currently previewing: {}\nPress [esc] to deselect",
            change.name
        );
    }
}
