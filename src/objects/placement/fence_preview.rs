use super::{ChangePlacementObject, PlaceObject};
use crate::{
    camera::CursorRaycast,
    objects::{utility::ObjectUtility, Object, ObjectGroup},
};
use bevy::prelude::*;

// TODO - allow barrier posts to be selected and extended from

/// Marker component for a single and always existing fence preview entity
#[derive(Component)]
pub struct FencePreview {
    enabled: bool,
    /// Location that the fence preview will originate from, will always point to cursor
    from: Vec3,
}

pub fn setup(mut object_utility: ObjectUtility) {
    // spawn in preview entity that is hidden
    object_utility.spawn_object_with(
        &Handle::default(),
        (
            FencePreview {
                enabled: false,
                from: Vec3::ZERO,
            },
            Visibility::Hidden,
        ),
    );
}

pub fn handle_movement(
    cursor_raycast: CursorRaycast,
    mut preview: Query<(&mut Transform, &mut Visibility, &FencePreview)>,
) {
    let (mut transform, mut visibility, fence) = preview.single_mut();

    // hide preview if not enabled
    if !fence.enabled {
        *visibility = Visibility::Hidden;
        return;
    }

    match cursor_raycast.ground_point() {
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

pub fn change_placement_object(
    mut object_changes: EventReader<ChangePlacementObject>,
    mut preview: Query<&mut FencePreview>,
) {
    let mut fence = preview.single_mut();

    for _ in object_changes.iter() {
        // disable preview on placement object change
        fence.enabled = false;
    }
}

pub fn place_object(
    mut object_utility: ObjectUtility,
    objects: Res<Assets<Object>>,
    mut object_placements: EventReader<PlaceObject>,
    mut preview: Query<(&mut FencePreview, &Transform, Entity)>,
) {
    let (mut fence, transform, entity) = preview.single_mut();

    for place_event in object_placements.iter() {
        let placed_object = objects.get(&place_event.object).unwrap();

        match &placed_object.group {
            ObjectGroup::BarrierPost(fence_handle) => match fence.enabled {
                // if preview fence is enabled, spawn in a permanent fence at the preview's location
                true => {
                    // spawn fence at preview location
                    object_utility.spawn_object_with(fence_handle, transform.clone());

                    // and update preview component
                    fence.from = place_event.location;
                }

                // if preview fence is disabled, enable it and set its starting point
                false => {
                    // set preview's object
                    object_utility.set_object(entity, fence_handle);

                    // update preview component
                    fence.enabled = true;
                    fence.from = place_event.location;
                }
            },

            // if it wasn't a barrier that was placed, disable the fence preview
            _ => fence.enabled = false,
        }
    }
}
