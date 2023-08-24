use super::{ChangePlacementObject, PlaceObject};
use crate::{
    camera::CursorRaycast,
    objects::{Object, ObjectGroup},
};
use bevy::prelude::*;

// TODO - allow barrier posts to be selected and extended from

/// Single and sometimes existing fence preview entity, if placing barriers
#[derive(Component)]
pub struct FencePreview {
    /// Location that the fence preview will originate from, will always point to cursor
    from: Vec3,
}

pub fn handle_movement(
    cursor_raycast: CursorRaycast,
    mut preview: Query<(&mut Transform, &mut Visibility, &FencePreview)>,
) {
    if let Some((mut transform, mut visibility, fence)) = preview.get_single_mut().ok() {
        match cursor_raycast.point() {
            Some(point) => {
                // calculate angle and scale for the fence preview
                let length = fence.from.distance(point);
                let angle = -f32::atan2(point.z - fence.from.z, point.x - fence.from.x);
                *transform = Transform {
                    translation: fence.from,
                    rotation: Quat::from_rotation_y(angle),
                    scale: Vec3::new(length, 1.0, 1.0),
                };

                // also set it visible
                *visibility = Visibility::Visible;
            }

            // if no cursor point in world, hide preview
            None => *visibility = Visibility::Hidden,
        }
    }
}

pub fn change_placement_object(
    mut commands: Commands,
    mut object_changes: EventReader<ChangePlacementObject>,
    preview: Query<Entity, With<FencePreview>>,
) {
    // despawn preview on placement object change
    for _ in object_changes.iter() {
        if let Some(entity) = preview.get_single().ok() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn place_object(
    mut commands: Commands,
    objects: Res<Assets<Object>>,
    mut object_placements: EventReader<PlaceObject>,
    mut preview: Query<(&mut FencePreview, &Transform, Entity)>,
) {
    for place_event in object_placements.iter() {
        let object = objects.get(&place_event.object).unwrap();

        match &object.group {
            ObjectGroup::Barrier(fence_model) => match preview.get_single_mut().ok() {
                // if preview fence exists, spawn in a permanent fence at the preview's location
                Some((mut fence, transform, _)) => {
                    // spawn fence
                    commands.spawn(SceneBundle {
                        scene: fence_model.clone(),
                        transform: transform.clone(),
                        ..default()
                    });

                    // update preview
                    fence.from = place_event.location;
                }

                // if preview fence doesn't exist, create a new one from given location
                None => {
                    commands.spawn((
                        SceneBundle {
                            scene: fence_model.clone(),
                            ..default()
                        },
                        FencePreview {
                            from: place_event.location,
                        },
                    ));
                }
            },

            // if it wasn't a barrier that was placed, despawn the fence preview if it exists
            _ => {
                if let Some((_, _, entity)) = preview.get_single().ok() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
