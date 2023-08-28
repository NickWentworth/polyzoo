use super::Placement;
use crate::{camera::CursorRaycast, objects::Object};
use bevy::prelude::*;

/// Event for swapping the clicked-on object in the world
#[derive(Event)]
pub struct SelectObject {
    pub object: Option<Handle<Object>>,
}

pub fn handle_selection(
    placement: Res<Placement>,
    mouse_buttons: Res<Input<MouseButton>>,
    cursor_raycast: CursorRaycast,

    object_query: Query<(&Handle<Object>, &Children)>,
    mut object_selection: EventWriter<SelectObject>,
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
            object_selection.send(SelectObject {
                object: Some(object_handle.clone()),
            })
        }
    }
}
