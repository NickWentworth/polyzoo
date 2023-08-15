use bevy::prelude::*;

use crate::camera::CursorRaycast;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_placement)
            .add_systems(Update, handle_placement);
    }
}

/// Marker trait to denote the currently previewed object
#[derive(Component)]
struct Preview;

fn setup_placement(mut commands: Commands, assets: Res<AssetServer>) {
    // spawn the first preview post
    commands.spawn((
        SceneBundle {
            scene: assets.load("barriers/wooden_barrier.glb#Scene0"),
            ..default()
        },
        Preview,
    ));
}

fn handle_placement(
    mut commands: Commands,
    assets: Res<AssetServer>,

    mut preview_post: Query<(&mut Transform, &mut Visibility, Entity), With<Preview>>,

    cursor_raycast: CursorRaycast,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let (mut transform, mut visibility, entity) = preview_post.single_mut();

    match cursor_raycast.point() {
        // if the mouse is hovering over terrain, show the preview post
        Some(point) => {
            // set it to visible and update translation
            *visibility = Visibility::Visible;
            transform.translation = point;

            // if the place button is pressed, spawn a new post, leaving the old post where it last was
            if mouse_buttons.just_pressed(MouseButton::Left) {
                commands.spawn((
                    SceneBundle {
                        scene: assets.load("barriers/wooden_barrier.glb#Scene0"),
                        transform: Transform::from_translation(point),
                        ..default()
                    },
                    Preview,
                ));

                // remove the preview tag from old post
                commands.entity(entity).remove::<Preview>();
            }
        }

        // if the mouse is not over terrain, hide the preview post
        None => *visibility = Visibility::Hidden,
    }
}
