use crate::props::{Prop, SetPreviewProp, UnsetPreviewProp};
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
    mut unset_previews: EventWriter<UnsetPreviewProp>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        unset_previews.send(UnsetPreviewProp);
    }
}

pub fn deselect_prompt_callbacks<P: Prop>(
    props: Res<Assets<P>>,
    mut preview_sets: EventReader<SetPreviewProp<P>>,
    mut preview_unsets: EventReader<UnsetPreviewProp>,
    mut placement_message: Query<&mut Text, With<PlacementCloseMessage>>,
) {
    for _ in preview_unsets.iter() {
        placement_message.single_mut().sections[0].value = String::from("");
    }

    for set in preview_sets.iter() {
        let prop = props.get(&set.handle).unwrap();

        placement_message.single_mut().sections[0].value = format!(
            "Currently placing: {}\nPress [esc] to deselect",
            prop.name()
        );
    }
}
