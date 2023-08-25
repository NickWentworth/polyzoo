use super::{ChangePlacementObject, PlaceObject};
use crate::{
    camera::CursorRaycast,
    objects::{Object, ObjectGroup, ObjectUtility},
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
                // calculate midpoint, angle, and length for the fence preview
                let midpoint = fence.from.lerp(point, 0.5);
                let angle = -f32::atan2(point.z - fence.from.z, point.x - fence.from.x);
                let length = fence.from.distance(point);

                *transform = Transform {
                    translation: midpoint,
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
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn place_object(
    mut object_utility: ObjectUtility,
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
                Some((mut fence_preview, transform, _)) => {
                    // spawn fence
                    let fence_entity = object_utility.spawn_mesh(fence_model);
                    commands.entity(fence_entity).insert(transform.clone());

                    // update preview
                    fence_preview.from = place_event.location;
                }

                // if preview fence doesn't exist, create a new one from given location
                None => {
                    let fence_entity = object_utility.spawn_mesh(fence_model);
                    commands.entity(fence_entity).insert((
                        FencePreview {
                            from: place_event.location,
                        },
                        Visibility::Hidden,
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
