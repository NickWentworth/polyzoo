use super::{
    utility::{RenderGltf, RenderGltfMode},
    PlacePreview, PreviewData,
};
use crate::{camera::CursorRaycast, objects::PropData};
use bevy::prelude::*;

// TODO - this should probably go elsewhere
/// Spawned instance of a prop in the world
#[derive(Component)]
pub struct Prop {
    data: Handle<PropData>,
}

pub struct PropPlacementPlugin;
impl Plugin for PropPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_preview_movement, on_preview_place));
    }
}

impl PreviewData for Handle<PropData> {
    fn spawn_preview(&self, commands: &mut Commands) -> Vec<Entity> {
        vec![commands.spawn_prop_preview(self.clone())]
    }
}

trait PropCommandsExtension {
    fn spawn_prop_preview(&mut self, prop_handle: Handle<PropData>) -> Entity;
}

impl PropCommandsExtension for Commands<'_, '_> {
    fn spawn_prop_preview(&mut self, prop_handle: Handle<PropData>) -> Entity {
        self.spawn_empty()
            .add(|id, world: &mut World| {
                // get the prop data from assets collection
                let props = world.resource::<Assets<PropData>>();
                let prop_data = props.get(&prop_handle).unwrap();

                // store any required world-related data before inserting
                let mesh = prop_data.mesh.clone();

                // insert all components into created entity
                world.entity_mut(id).insert((
                    SpatialBundle::default(),
                    PropPreview { data: prop_handle },
                    RenderGltf {
                        handle: mesh,
                        mode: RenderGltfMode::Preview,
                    },
                ));
            })
            .id()
    }
}

/// Component marking the currently previewed prop
#[derive(Component)]
struct PropPreview {
    data: Handle<PropData>,
}

fn handle_preview_movement(
    cursor: CursorRaycast,
    mut preview: Query<(&mut Transform, &mut Visibility), With<PropPreview>>,
) {
    if let Ok((mut transform, mut visibility)) = preview.get_single_mut() {
        match cursor.ground_point() {
            Some(position) => {
                transform.translation = position;
                *visibility = Visibility::Visible;
            }

            None => *visibility = Visibility::Hidden,
        }
    }
}

fn on_preview_place(
    mut commands: Commands,
    props: Res<Assets<PropData>>,

    mut placements: EventReader<PlacePreview>,
    preview: Query<(&PropPreview, &Transform)>,
) {
    for _ in placements.iter() {
        let Ok((preview, transform)) = preview.get_single() else { return };
        let Some(prop_data) = props.get(&preview.data) else { return };

        commands.spawn((
            SpatialBundle {
                transform: transform.clone(),
                ..default()
            },
            Prop {
                data: preview.data.clone(),
            },
            RenderGltf {
                handle: prop_data.mesh.clone(),
                mode: RenderGltfMode::Regular,
            },
        ));
    }
}
