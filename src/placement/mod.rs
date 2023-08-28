use crate::{camera::CursorRaycast, objects::Object, zoo::ZooBalanceChange};
use bevy::prelude::*;

mod fence_preview;
mod object_preview;
mod selectable;

pub use selectable::SelectObject;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Placement>()
            .add_event::<ChangePlacementObject>()
            .add_event::<PlaceObject>()
            // placement resource systems
            .add_systems(
                Update,
                (change_placement_object, handle_validity, handle_placement),
            )
            // object preview systems
            .add_systems(Startup, object_preview::setup)
            .add_systems(
                Update,
                (
                    object_preview::handle_movement,
                    object_preview::change_placement_object,
                    object_preview::place_object,
                ),
            )
            // fence preview systems
            .add_systems(
                Update,
                (
                    fence_preview::handle_movement,
                    fence_preview::change_placement_object,
                    fence_preview::place_object,
                ),
            )
            // object selection systems
            .add_event::<selectable::SelectObject>()
            .add_systems(Update, selectable::handle_selection);
    }
}

#[derive(Resource, Default)]
pub struct Placement {
    /// Handle to the object currently being placed, if it exists
    object: Option<Handle<Object>>,
    /// Validity of the current position of the placed object
    valid: bool,
}

impl Placement {
    /// Returns whether or not the placement resource is currently placing an object
    pub fn is_placing(&self) -> bool {
        self.object.is_some()
    }
}

/// Event for systems to change the object being placed
///
/// Pass in `Some(handle)` to set the object or `None` to leave placement mode
#[derive(Event)]
pub struct ChangePlacementObject {
    pub object: Option<Handle<Object>>,
}

/// Event for preview systems to place a permanent copy of their previews
#[derive(Event)]
pub struct PlaceObject {
    pub object: Handle<Object>,
    pub location: Vec3,
}

/// Handles changing the `Placement` resource on `ChangePlacementObject` events
fn change_placement_object(
    mut placement: ResMut<Placement>,
    mut object_changes: EventReader<ChangePlacementObject>,
) {
    for change_event in object_changes.iter() {
        placement.object = change_event.object.clone();
    }
}

// TODO - check for collisions as well, likely has to be done through events from preview systems
/// Handles keeping the `Placement` resource's valid placement field up to date
fn handle_validity(mut placement: ResMut<Placement>, cursor_raycast: CursorRaycast) {
    placement.valid = cursor_raycast.ground_point().is_some();
}

/// Handles writing `PlaceObject` events to preview systems when a valid placement occurs
fn handle_placement(
    placement: Res<Placement>,
    objects: Res<Assets<Object>>,

    mouse_buttons: Res<Input<MouseButton>>,
    cursor_raycast: CursorRaycast,

    mut object_placements: EventWriter<PlaceObject>,
    mut zoo_balance_change: EventWriter<ZooBalanceChange>,
) {
    if let Some(object_handle) = &placement.object {
        if placement.valid && mouse_buttons.just_pressed(MouseButton::Left) {
            // send object placement event to previews
            object_placements.send(PlaceObject {
                object: object_handle.clone(),
                location: cursor_raycast.ground_point().unwrap(),
            });

            // send zoo balance change event to zoo
            let object = objects.get(object_handle).unwrap();
            zoo_balance_change.send(ZooBalanceChange {
                amount: -object.cost,
            });
        }
    }
}
