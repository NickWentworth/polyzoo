use super::{ChangePlacementObject, PlaceObject};
use crate::{camera::CursorRaycast, objects::Object};
use bevy::prelude::*;

/// Marker component for a single and always existing preview entity
#[derive(Component)]
pub struct Preview;

pub fn setup(mut commands: Commands) {
    // spawn in preview entity that is hidden
    commands.spawn((
        SpatialBundle {
            visibility: Visibility::Hidden,
            ..default()
        },
        Handle::<Object>::default(),
        Preview,
    ));
}

pub fn handle_movement(
    cursor_raycast: CursorRaycast,
    mut preview: Query<(&mut Transform, &mut Visibility), With<Preview>>,
) {
    let (mut transform, mut visibility) = preview.single_mut();

    match cursor_raycast.ground_point() {
        Some(point) => {
            transform.translation = point;
            *visibility = Visibility::Visible;
        }

        None => *visibility = Visibility::Hidden,
    }
}

pub fn change_placement_object(
    mut commands: Commands,
    mut object_changes: EventReader<ChangePlacementObject>,
    preview: Query<Entity, With<Preview>>,
) {
    let preview_entity = preview.single();

    for change_event in object_changes.iter() {
        let handle = change_event.object.clone().unwrap_or(Handle::default());
        commands.entity(preview_entity).insert(handle);
    }
}

// FIXME - sometimes while placing barriers a state will occur where the cursor ground position
//         is not found and will return None, even if the cursor is still directly over ground
//
//         sometimes after the bug occurs, it can be fixed by hovering over the first-placed
//         post, this makes me think further that the entities are being remapped somehow and the
//         engine is confusing the ground for a barrier
//
//         this bug also only occurs when placing barriers, it seems to never happen when placing
//         single objects like rocks that require no extra spawning
pub fn place_object(
    mut commands: Commands,
    mut object_placements: EventReader<PlaceObject>,
    preview: Query<&Transform, With<Preview>>,
) {
    let preview_transform = preview.single();

    for place_event in object_placements.iter() {
        commands.spawn((
            SpatialBundle {
                transform: preview_transform.clone(),
                ..default()
            },
            place_event.object.clone(),
        ));
    }
}
