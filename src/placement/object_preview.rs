use super::{ChangePlacementObject, PlaceObject};
use crate::{camera::CursorRaycast, objects::ObjectUtility};
use bevy::prelude::*;

/// Marker component for a single and always existing preview entity
#[derive(Component)]
pub struct Preview;

pub fn setup(mut commands: Commands) {
    // spawn in preview entity that is hidden
    commands.spawn((
        SceneBundle {
            visibility: Visibility::Hidden,
            ..default()
        },
        Preview,
    ));
}

pub fn handle_movement(
    cursor_raycast: CursorRaycast,
    mut preview: Query<(&mut Transform, &mut Visibility), With<Preview>>,
) {
    let (mut transform, mut visibility) = preview.single_mut();

    match cursor_raycast.point() {
        Some(point) => {
            transform.translation = point;
            *visibility = Visibility::Visible;
        }

        None => *visibility = Visibility::Hidden,
    }
}

pub fn change_placement_object(
    mut object_utility: ObjectUtility,
    mut object_changes: EventReader<ChangePlacementObject>,

    preview: Query<Entity, With<Preview>>,
) {
    let preview_entity = preview.single();

    for change_event in object_changes.iter() {
        match &change_event.object {
            // if an object is set, swap out the child entities
            Some(handle) => {
                object_utility.set_object(handle, preview_entity);
            }

            // if no object, then set to default handle, which will not render anything
            None => {
                object_utility.clear_object(preview_entity);
            }
        };
    }
}

pub fn place_object(
    mut object_utility: ObjectUtility,
    mut commands: Commands,

    mut object_placements: EventReader<PlaceObject>,
    preview: Query<&Transform, With<Preview>>,
) {
    let preview_transform = preview.single();

    for place_event in object_placements.iter() {
        let object = object_utility.spawn_object(&place_event.object);
        commands.entity(object).insert(preview_transform.clone());
    }
}
