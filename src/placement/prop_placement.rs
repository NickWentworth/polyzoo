use super::{
    utility::{RenderGltf, RenderGltfMode},
    PlacePreview, Preview, PreviewData,
};
use crate::{camera::CursorRaycast, objects::PropData};
use bevy::prelude::*;

// TODO - this should probably go elsewhere
/// Instance of a prop in the world
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
    fn spawn_preview(&self, commands: &mut Commands) {
        let prop_handle = self.clone();

        commands.add(|world: &mut World| {
            // get the prop data from assets collection
            let props = world.resource::<Assets<PropData>>();
            let prop_data = props.get(&prop_handle).unwrap();

            // store any required world-related data before inserting
            let mesh = prop_data.mesh.clone();
            let cost = prop_data.cost;

            // spawn prop preview
            world.spawn((
                SpatialBundle::default(),
                Prop { data: prop_handle },
                Preview { cost },
                RenderGltf {
                    handle: mesh,
                    mode: RenderGltfMode::Preview,
                },
            ));
        });
    }
}

fn handle_preview_movement(
    cursor: CursorRaycast,
    mut preview: Query<(&mut Transform, &mut Visibility), (With<Prop>, With<Preview>)>,
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
    preview: Query<(&Prop, &Transform), With<Preview>>,
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
