use crate::camera::CursorRaycast;
use bevy::prelude::*;
use std::sync::Arc;

mod barrier_placement;
mod prop_placement;
mod utility;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            prop_placement::PropPlacementPlugin,
            barrier_placement::BarrierPlacementPlugin,
        ))
        .init_resource::<PlacementManager>()
        .add_event::<ChangePreview>()
        .add_event::<ClearPreview>()
        .add_event::<PlacePreview>()
        .add_systems(Update, (on_preview_change, on_click))
        .add_systems(Update, utility::handle_mesh_changes);
    }
}

/// Allows a type's preview to be shown and hidden
///
/// Show preview by writing a `ChangePreview` event
/// Hide preview by writing a `ClearPreview` event
///
/// The type is still expected to handle `PlacePreview` events to place the object into the world
pub trait PreviewData: Send + Sync + 'static {
    /// Spawn in a preview of the object and return a vector of preview entities created
    fn spawn_preview(&self, commands: &mut Commands) -> Vec<Entity>;
}

/// Resource to keep track of the current preview entities
#[derive(Resource, Default)]
pub struct PlacementManager {
    previews: Vec<Entity>,
}

impl PlacementManager {
    /// Despawns all currently stored previews in the placement manager
    fn despawn_previews(&mut self, commands: &mut Commands) {
        for entity in self.previews.drain(..) {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Request event to change the currently shown preview
#[derive(Event, Clone)]
pub struct ChangePreview {
    pub to: Arc<dyn PreviewData>,
    pub name: String,
}

/// Request event to clear the currently shown preview
#[derive(Event)]
pub struct ClearPreview;

/// Event sent to previewing systems to place their previews if they exist
#[derive(Event)]
struct PlacePreview;

fn on_preview_change(
    mut commands: Commands,
    mut manager: ResMut<PlacementManager>,

    mut changes: EventReader<ChangePreview>,
    mut clears: EventReader<ClearPreview>,
) {
    for _ in clears.iter() {
        manager.despawn_previews(&mut commands);
    }

    for change in changes.iter() {
        manager.despawn_previews(&mut commands);
        manager.previews = change.to.spawn_preview(&mut commands);
    }
}

fn on_click(
    cursor: CursorRaycast,
    placement_manager: Res<PlacementManager>,

    mouse: Res<Input<MouseButton>>,
    mut placements: EventWriter<PlacePreview>,
) {
    // TODO - improve validity check, should be handled independently by each placement system
    // no need to send place event if there are no previews
    if mouse.just_pressed(MouseButton::Left)
        && cursor.ground_point().is_some()
        && !placement_manager.previews.is_empty()
    {
        placements.send(PlacePreview);
    }
}
