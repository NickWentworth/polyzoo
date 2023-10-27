use crate::{camera::CursorRaycast, Currency};
use bevy::prelude::*;
use std::sync::Arc;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangePreview>()
            .add_event::<ClearPreview>()
            .add_event::<PlacePreview>()
            .add_systems(Update, (on_preview_change, on_escape_press, on_click));
    }
}

/// Allows a type's preview to be shown and hidden
///
/// Show preview by writing a `ChangePreview` event
/// Hide preview by writing a `ClearPreview` event
///
/// The type is still expected to handle `PlacePreview` events to place the object into the world
pub trait PreviewData: Send + Sync + 'static {
    /// Spawn in any entities for previewing the object, marked with a `Preview` component
    fn spawn_preview(&self, commands: &mut Commands);
}

/// Component that should exist on all preview entities, stores important required data for placement
#[derive(Component)]
pub struct Preview {
    pub cost: Currency,
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
pub struct PlacePreview;

fn on_preview_change(
    mut commands: Commands,
    previews: Query<Entity, With<Preview>>,

    mut changes: EventReader<ChangePreview>,
    mut clears: EventReader<ClearPreview>,
) {
    for _ in clears.iter() {
        for entity in previews.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    for change in changes.iter() {
        for entity in previews.iter() {
            commands.entity(entity).despawn_recursive();
        }

        change.to.spawn_preview(&mut commands);
    }
}

/// Clears the preview when the escape key is pressed
pub fn on_escape_press(keys: Res<Input<KeyCode>>, mut clears: EventWriter<ClearPreview>) {
    if keys.just_pressed(KeyCode::Escape) {
        clears.send(ClearPreview);
    }
}

fn on_click(
    mouse: Res<Input<MouseButton>>,
    cursor: CursorRaycast,
    mut placements: EventWriter<PlacePreview>,
) {
    // TODO - improve validity check, should be handled independently by each placement system
    // no need to send place event if there are no previews
    if mouse.just_pressed(MouseButton::Left) && cursor.ground_point().is_some() {
        placements.send(PlacePreview);
    }
}
