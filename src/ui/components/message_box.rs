use super::theme::UiTheme;
use crate::placement::{ChangePreview, ClearPreview};
use bevy::prelude::*;

pub struct MessageBoxPlugin;
impl Plugin for MessageBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_message_box)
            .add_systems(Update, on_preview_change);
    }
}

/// Marker component for the message box's text bundle
#[derive(Component)]
struct MessageBox;

fn setup_message_box(mut commands: Commands, theme: Res<UiTheme>) {
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
                MessageBox,
            ));
        });
}

/// Sets or clears the message box when a preview object is selected
fn on_preview_change(
    mut changes: EventReader<ChangePreview>,
    mut clears: EventReader<ClearPreview>,
    mut message_box: Query<&mut Text, With<MessageBox>>,
) {
    for _ in clears.iter() {
        message_box.single_mut().sections[0].value = "".into();
    }

    for change in changes.iter() {
        message_box.single_mut().sections[0].value = format!(
            "Currently previewing: {}\nPress [esc] to deselect",
            change.name,
        );
    }
}
