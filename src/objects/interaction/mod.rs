use super::Object;
use bevy::prelude::*;

mod selectable;

pub fn build(app: &mut App) {
    app
        // object selection systems
        .add_event::<SelectObject>()
        .add_systems(Update, selectable::handle_selection);
}

/// Event for swapping the clicked-on object in the world
#[derive(Event)]
pub struct SelectObject {
    pub object: Option<Handle<Object>>,
}
