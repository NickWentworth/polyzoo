use super::{placement::Placement, Object};
use crate::{camera::CursorRaycast, zoo::ZooBalanceChange};
use bevy::prelude::*;

pub fn build(app: &mut App) {
    app.init_resource::<Selection>()
        .add_event::<SelectObject>()
        .add_event::<DeselectObject>()
        .add_event::<SellSelectedObject>()
        .add_event::<TransformGizmoSelectedObject>()
        .add_systems(
            Update,
            (handle_selection, on_select, on_deselect, on_sell, on_move),
        );
}

#[derive(Resource, Default)]
pub struct Selection {
    // Currently selected object and entity, if it exists
    pub selected: Option<(Entity, Handle<Object>)>,
}

/// Event for the clicked-on object in the world
#[derive(Event)]
pub struct SelectObject {
    pub entity: Entity,
    pub object: Handle<Object>,
}

/// Event for deselecting the currently selected object
#[derive(Event)]
pub struct DeselectObject;

/// Event for despawning the currently selected object
#[derive(Event)]
pub struct SellSelectedObject;

/// Event for showing move gizmo on the currently selected object
#[derive(Event)]
pub struct TransformGizmoSelectedObject;

pub fn handle_selection(
    placement: Res<Placement>,
    mouse_buttons: Res<Input<MouseButton>>,
    cursor_raycast: CursorRaycast,

    object_query: Query<(&Handle<Object>, Entity, &Children)>,
    mut object_selection: EventWriter<SelectObject>,
) {
    // TODO - add a proper interaction state tracker in case other systems use the cursor
    if placement.is_placing() || !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    // get the first entity clicked on from the camera, which should be a child mesh of an object parent
    let Some(mesh_entity) = cursor_raycast.first_entity() else { return };

    // get the object handle from the parent
    let object_check = object_query
        .iter()
        .find(|(_, _, children)| children.contains(&mesh_entity));
    let Some((object_handle, object_entity, _)) = object_check else { return };

    // if we get to here, an object was selected, send an event out
    object_selection.send(SelectObject {
        entity: object_entity,
        object: object_handle.clone(),
    });
}

fn on_select(mut selection: ResMut<Selection>, mut object_selections: EventReader<SelectObject>) {
    for select in object_selections.iter() {
        selection.selected = Some((select.entity, select.object.clone()));
    }
}

fn on_deselect(
    mut selection: ResMut<Selection>,
    mut object_deselections: EventReader<DeselectObject>,
) {
    for _ in object_deselections.iter() {
        selection.selected = None;
    }
}

fn on_sell(
    mut commands: Commands,
    selection: Res<Selection>,
    objects: Res<Assets<Object>>,
    mut object_sells: EventReader<SellSelectedObject>,

    mut object_deselections: EventWriter<DeselectObject>,
    mut zoo_balance_changes: EventWriter<ZooBalanceChange>,
) {
    for _ in object_sells.iter() {
        if let Some((entity, handle)) = &selection.selected {
            // get object data from assets
            let sold_object = objects.get(handle).unwrap();

            // send out events for selling
            object_deselections.send(DeselectObject);
            zoo_balance_changes.send(ZooBalanceChange {
                amount: sold_object.sell_value(),
            });

            // finally despawn object
            commands.entity(*entity).despawn_recursive();
        }
    }
}

fn on_move(selection: Res<Selection>, mut object_moves: EventReader<TransformGizmoSelectedObject>) {
    for _ in object_moves.iter() {
        if let Some((entity, _)) = &selection.selected {
            println!("TODO - show transform gizmo on {:?}", entity);
        }
    }
}
