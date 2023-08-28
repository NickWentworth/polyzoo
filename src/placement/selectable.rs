use super::Placement;
use crate::{camera::CursorRaycast, objects::Object};
use bevy::prelude::*;

pub fn handle_selection(
    placement: Res<Placement>,
    mouse_buttons: Res<Input<MouseButton>>,
    cursor_raycast: CursorRaycast,

    objects: Res<Assets<Object>>,
    object_query: Query<(&Handle<Object>, &Children)>,
) {
    // TODO - add a proper interaction state tracker in case other systems use the cursor
    if placement.is_placing() || !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(entity) = cursor_raycast.first_entity() {
        if let Some((object_handle, _)) = object_query
            .iter()
            .find(|(_, children)| children.contains(&entity))
        {
            let object = objects.get(object_handle).unwrap();
            println!("Just selected: {}", object.name);
        }
    }
}
