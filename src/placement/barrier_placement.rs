use super::{
    utility::{ColliderMesh, RenderGltf, RenderGltfMode},
    PlacePreview, Preview, PreviewData,
};
use crate::{camera::CursorRaycast, objects::BarrierData, OBJECTS};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Instance of a barrier's post in the world
#[derive(Component, Clone)]
pub struct BarrierPost {
    data: Handle<BarrierData>,
    /// List of all connecting fence entities
    fences: Vec<Entity>,
}

/// Instance of a barrier's fence in the world
#[derive(Component, Clone)]
pub struct BarrierFence {
    data: Handle<BarrierData>,
    /// Connection between posts that this fence serves
    connection: [Entity; 2],
}

/// Marks a fence as changed if either of its connected posts are moved
fn mark_fence_movement(
    moved_posts: Query<&BarrierPost, Changed<Transform>>,
    mut fences: Query<&mut BarrierFence>,
) {
    for moved_post in moved_posts.iter() {
        for fence_entity in moved_post.fences.iter() {
            if let Ok(mut fence) = fences.get_mut(*fence_entity) {
                fence.set_changed();
            }
        }
    }
}

/// Handles adjusting the fence's transform if its `BarrierFence` component has been changed
fn handle_fence_movement(
    mut moved_fences: Query<(&BarrierFence, &mut Transform), Changed<BarrierFence>>,
    all_posts: Query<&Transform, (With<BarrierPost>, Without<BarrierFence>)>,
) {
    for (fence, mut transform) in moved_fences.iter_mut() {
        // get positions of both posts
        let Ok(from) = all_posts.get(fence.connection[0]).and_then(|t| Ok(t.translation)) else { continue };
        let Ok(to) = all_posts.get(fence.connection[1]).and_then(|t| Ok(t.translation)) else { continue };

        // and calculate transform values for this fence
        let midpoint = from.lerp(to, 0.5);
        let angle = -f32::atan2(to.z - from.z, to.x - from.x);
        let distance = from.distance(to);

        *transform = Transform {
            translation: midpoint,
            rotation: Quat::from_rotation_y(angle),
            scale: Vec3::new(distance, 1.0, 1.0),
        };
    }
}

#[derive(Clone)]
pub struct BarrierPlacementPlugin;
impl Plugin for BarrierPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_preview_movement,
                update_fence_preview_cost,
                on_preview_place,
            ),
        )
        // TEMP - temporary systems for fence movement that should be general for all fences, these should go elsewhere
        .add_systems(
            Update,
            (
                mark_fence_movement,
                handle_fence_movement.after(mark_fence_movement),
            ),
        );
    }
}

impl PreviewData for Handle<BarrierData> {
    fn spawn_preview(&self, commands: &mut Commands) {
        let barrier_handle = self.clone();

        commands.add(move |world: &mut World| {
            // get the barrier data from assets collection
            let barriers = world.resource::<Assets<BarrierData>>();
            let barrier_data = barriers.get(&barrier_handle).unwrap();

            // spawn barrier post preview
            world.spawn((
                SpatialBundle::default(),
                BarrierPost {
                    data: barrier_handle.clone(),
                    fences: Vec::new(),
                },
                Preview {
                    cost: barrier_data.post_cost,
                },
                RenderGltf {
                    handle: barrier_data.post_model.clone(),
                    mode: RenderGltfMode::Preview,
                },
                ColliderMesh {
                    mesh: barrier_data.post_collider.clone(),
                    rb: RigidBody::Fixed,
                    membership: OBJECTS,
                },
            ));

            // barrier fence preview will be spawned as needed later on
        })
    }
}

/// Handles movement of post preview and visibility of both post and fence previews
fn handle_preview_movement(
    cursor: CursorRaycast,
    mut previous_position: Local<Option<Vec3>>,

    mut previews: ParamSet<(
        // post preview
        Query<(&mut Transform, &mut Visibility), (With<BarrierPost>, With<Preview>)>,
        // fence preview
        Query<&mut Visibility, (With<BarrierPost>, With<Preview>)>,
    )>,
) {
    let ground_point = cursor.ground_point();

    // prevents needless mutating of preview transforms if position is the same
    if ground_point == *previous_position {
        return;
    } else {
        *previous_position = ground_point;
    }

    // update post preview's transform and visibility
    if let Ok((mut post_transform, mut post_visibility)) = previews.p0().get_single_mut() {
        match ground_point {
            Some(position) => {
                post_transform.translation = position;
                *post_visibility = Visibility::Visible;
            }

            None => *post_visibility = Visibility::Hidden,
        }
    }

    // only update fence preview's visibility
    // fence systems will handle updating fence movement automatically
    if let Ok(mut fence_visibility) = previews.p1().get_single_mut() {
        match ground_point {
            Some(_) => *fence_visibility = Visibility::Visible,
            None => *fence_visibility = Visibility::Hidden,
        }
    }
}

/// Handles updating the fence's preview cost when its transform changes
fn update_fence_preview_cost(
    barriers: Res<Assets<BarrierData>>,
    mut fence_preview: Query<(&mut Preview, &BarrierFence, &Transform), Changed<Transform>>,
) {
    let Ok((mut preview, fence, transform)) = fence_preview.get_single_mut() else { return };
    let Some(barrier_data) = barriers.get(&fence.data) else { return };

    preview.cost = transform.scale.x * barrier_data.fence_cost;
}

fn on_preview_place(
    mut commands: Commands,
    barriers: Res<Assets<BarrierData>>,
    mut placements: EventReader<PlacePreview>,

    mut post_preview_query: Query<(Entity, &mut BarrierPost, &Transform), With<Preview>>,
    mut fence_preview_query: Query<(&mut BarrierFence, &Transform), With<Preview>>,
) {
    for _ in placements.iter() {
        // there should at least be a preview post to place, if any
        let Ok((post_preview_entity, mut post_preview, post_preview_transform)) = post_preview_query.get_single_mut() else { return };
        let Some(barrier_data) = barriers.get(&post_preview.data) else { return };

        // spawn in empty entities, as their components rely upon eachother
        let placed_post_entity = commands.spawn_empty().id();
        let placed_fence_entity = commands.spawn_empty().id();

        // insert required components for permanent placed fence
        commands.entity(placed_post_entity).insert((
            SpatialBundle {
                transform: post_preview_transform.clone(),
                ..default()
            },
            BarrierPost {
                data: post_preview.data.clone(),
                fences: vec![placed_fence_entity],
            },
            RenderGltf {
                handle: barrier_data.post_model.clone(),
                mode: RenderGltfMode::Regular,
            },
            ColliderMesh {
                mesh: barrier_data.post_collider.clone(),
                rb: RigidBody::Fixed,
                membership: OBJECTS,
            },
        ));

        // fence spawning has varying behaviors
        match fence_preview_query.get_single_mut() {
            Ok((mut fence_preview, fence_preview_transform)) => {
                // spawn in a permanent fence if there is an existing preview
                commands.entity(placed_fence_entity).insert((
                    SpatialBundle {
                        transform: fence_preview_transform.clone(),
                        ..default()
                    },
                    BarrierFence {
                        data: fence_preview.data.clone(),
                        connection: [fence_preview.connection[0], placed_post_entity],
                    },
                    RenderGltf {
                        handle: barrier_data.fence_model.clone(),
                        mode: RenderGltfMode::Regular,
                    },
                    ColliderMesh {
                        mesh: barrier_data.fence_collider.clone(),
                        rb: RigidBody::Fixed,
                        membership: OBJECTS,
                    },
                ));

                // update the fence preview's first connection point to the newly placed post
                fence_preview.connection[0] = placed_post_entity;
                // the second connection point should be the preview post
                debug_assert_eq!(fence_preview.connection[1], post_preview_entity);
            }

            Err(_) => {
                // if there is no preview, spawn one in
                commands.entity(placed_fence_entity).insert((
                    SpatialBundle::default(),
                    BarrierFence {
                        data: post_preview.data.clone(),
                        connection: [placed_post_entity, post_preview_entity],
                    },
                    Preview { cost: 0.0 },
                    RenderGltf {
                        handle: barrier_data.fence_model.clone(),
                        mode: RenderGltfMode::Preview,
                    },
                    ColliderMesh {
                        mesh: barrier_data.fence_collider.clone(),
                        rb: RigidBody::Fixed,
                        membership: OBJECTS,
                    },
                ));

                // set the post preview's fences to only include the preview fence
                post_preview.fences = vec![placed_fence_entity];
            }
        }
    }
}
