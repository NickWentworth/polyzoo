use super::components::Prop;
use crate::{
    camera::CursorRaycast,
    objects::{
        utility::{ColliderMesh, CollisionLayer, RenderGltf, RenderGltfMode},
        ObjectBundle, PropData,
    },
    placement::{PlacePreview, Preview, PreviewData},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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

            // spawn prop preview
            world.spawn((
                ObjectBundle {
                    object: Prop { data: prop_handle },
                    spatial: SpatialBundle::default(),
                    gltf: RenderGltf {
                        handle: prop_data.model.clone(),
                        mode: RenderGltfMode::Preview,
                    },
                    collider: ColliderMesh {
                        mesh: prop_data.collider.clone(),
                        rb: RigidBody::Fixed,
                        membership: CollisionLayer::None,
                    },
                },
                Preview {
                    cost: prop_data.cost,
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

        commands.spawn(ObjectBundle {
            object: Prop {
                data: preview.data.clone(),
            },
            spatial: SpatialBundle {
                transform: transform.clone(),
                ..default()
            },
            gltf: RenderGltf {
                handle: prop_data.model.clone(),
                mode: RenderGltfMode::Regular,
            },
            collider: ColliderMesh {
                mesh: prop_data.collider.clone(),
                rb: RigidBody::Fixed,
                membership: CollisionLayer::Object,
            },
        });
    }
}
