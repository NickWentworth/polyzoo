use super::{ChangePlacementObject, PlaceObject};
use crate::{
    camera::CursorRaycast,
    objects::{Object, ObjectGroup},
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

pub fn setup(mut commands: Commands) {
    // spawn in preview entity that is hidden
    commands.spawn((
        SpatialBundle {
            visibility: Visibility::Hidden,
            ..default()
        },
        Handle::<Object>::default(),
        FencePreview {
            enabled: false,
            from: Vec3::ZERO,
        },
    ));
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
    mut commands: Commands,
    mut object_changes: EventReader<ChangePlacementObject>,
    mut preview: Query<(&mut FencePreview, Entity)>,
) {
    let (mut fence, preview_entity) = preview.single_mut();

    for _ in object_changes.iter() {
        // disable preview on placement object change
        commands
            .entity(preview_entity)
            .insert(Handle::<Object>::default());
        fence.enabled = false;
    }
}

pub fn place_object(
    mut commands: Commands,
    objects: Res<Assets<Object>>,
    mut object_placements: EventReader<PlaceObject>,
    mut preview: Query<(&mut FencePreview, &Transform, Entity)>,
) {
    let (mut fence, transform, preview_entity) = preview.single_mut();

    for place_event in object_placements.iter() {
        let placed_object = objects.get(&place_event.object).unwrap();

        match &placed_object.group {
            ObjectGroup::BarrierPost(fence_handle) => {
                match fence.enabled {
                    // if preview fence is enabled, spawn in a permanent fence at preview location
                    true => {
                        commands.spawn((
                            SpatialBundle {
                                transform: transform.clone(),
                                ..default()
                            },
                            fence_handle.clone(),
                        ));
                    }

                    // if preview fence is disabled, enable it for the next placed post
                    false => {
                        commands.entity(preview_entity).insert(fence_handle.clone());
                    }
                };

                // update preview component
                fence.enabled = true;
                fence.from = place_event.location;
            }

            // if it wasn't a barrier that was placed, disable the fence preview
            _ => fence.enabled = false,
        }
    }
}
