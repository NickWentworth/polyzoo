use super::{ChangePlacementObject, PlaceObject};
use crate::{camera::CursorRaycast, objects::Object};
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
    objects: Res<Assets<Object>>,
    mut object_changes: EventReader<ChangePlacementObject>,
    mut preview: Query<&mut Handle<Scene>, With<Preview>>,
) {
    let mut preview_scene = preview.single_mut();

    for change_event in object_changes.iter() {
        match &change_event.object {
            // if an object is set, get its base model scene handle
            Some(handle) => {
                let object = objects.get(&handle).unwrap();
                *preview_scene = object.model.clone();
            }

            // if no object, then set to default handle, which will not render anything
            None => *preview_scene = Handle::default(),
        }
    }
}

pub fn place_object(
    mut commands: Commands,
    objects: Res<Assets<Object>>,
    mut object_placements: EventReader<PlaceObject>,
    preview: Query<&Transform, With<Preview>>,
) {
    let preview_transform = preview.single();

    for place_event in object_placements.iter() {
        let object = objects.get(&place_event.object).unwrap();

        // place an object with the same orientation as the preview
        commands.spawn(SceneBundle {
            scene: object.model.clone(),
            transform: preview_transform.clone(),
            ..default()
        });
    }
}
