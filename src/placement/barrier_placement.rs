use super::{
    utility::{ColliderMesh, CollisionLayer, RenderGltf, RenderGltfMode},
    PlacePreview, Preview, PreviewData,
};
use crate::{camera::CursorRaycast, objects::BarrierData};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct ObjectBundle<O: Component> {
    pub object: O,
    pub spatial: SpatialBundle,
    pub gltf: RenderGltf,
    pub collider: ColliderMesh,
}

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
        let Ok(from) = all_posts.get(fence.connection[0]).map(|t| t.translation) else { continue };
        let Ok(to) = all_posts.get(fence.connection[1]).map(|t| t.translation) else { continue };

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

/// Includes helper functions to interact with the barrier placement systems
#[derive(SystemParam)]
struct BarrierPreviewHelper<'w, 's> {
    post_query: Query<'w, 's, (Entity, &'static BarrierPost), With<Preview>>,
    fence_query: Query<'w, 's, (Entity, &'static BarrierFence), With<Preview>>,
    snap_query: Query<'w, 's, &'static SnapPreviewPost, (With<BarrierPost>, With<Preview>)>,
}

impl<'w, 's> BarrierPreviewHelper<'w, 's> {
    /// Returns the barrier previewing system's current status
    fn preview_status(&self) -> BarrierPreviewStatus {
        match (self.post_query.get_single(), self.fence_query.get_single()) {
            (Err(_), Err(_)) => BarrierPreviewStatus::None,
            (Ok((post, _)), Err(_)) => BarrierPreviewStatus::Post { post },
            (Ok((post, _)), Ok((fence, _))) => BarrierPreviewStatus::Connecting { post, fence },
            (Err(_), Ok(_)) => panic!(),
        }
    }

    /// Returns the currently previewed barrier's data, if it exists
    fn data(&self) -> Option<&Handle<BarrierData>> {
        match (self.post_query.get_single(), self.fence_query.get_single()) {
            (Err(_), Err(_)) => None,
            (Ok((_, post)), Err(_)) => Some(&post.data),
            (Ok((_, post)), Ok((_, fence))) => {
                assert_eq!(post.data, fence.data);
                Some(&post.data)
            }
            (Err(_), Ok(_)) => panic!(),
        }
    }

    /// Returns the entity that the preview should snap to, if it exists
    fn snap_post(&self) -> Option<Entity> {
        match self.preview_status() {
            // only snap if there is a connecting fence behind the preview post
            BarrierPreviewStatus::Connecting { .. } => {
                self.snap_query.get_single().map(|snap| snap.to).ok()
            }

            _ => None,
        }
    }
}

/// Describes the active previews within the barrier placement systems
#[derive(Clone, Copy, Debug)]
enum BarrierPreviewStatus {
    /// Not currently previewing anything
    None,
    /// Currently placing a post preview
    Post { post: Entity },
    /// Currently placing a post preview with a fence preview connecting to a permanent post
    Connecting { post: Entity, fence: Entity },
}

/// Component that marks when the preview post should snap to a particular existing post
#[derive(Component)]
struct SnapPreviewPost {
    to: Entity,
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
                ObjectBundle {
                    object: BarrierPost {
                        data: barrier_handle.clone(),
                        fences: Vec::new(),
                    },
                    spatial: SpatialBundle::default(),
                    gltf: RenderGltf {
                        handle: barrier_data.post_model.clone(),
                        mode: RenderGltfMode::Preview,
                    },
                    collider: ColliderMesh {
                        mesh: barrier_data.post_collider.clone(),
                        rb: RigidBody::Fixed,
                        membership: CollisionLayer::None,
                    },
                },
                Preview {
                    cost: barrier_data.post_cost,
                },
            ));

            // barrier fence preview will be spawned as needed later on
        })
    }
}

/// Handles movement of post preview and visibility of both post and fence previews
fn handle_preview_movement(
    mut commands: Commands,
    preview_helper: BarrierPreviewHelper,
    cursor: CursorRaycast,

    mut queries: ParamSet<(
        // preview's spatial components
        Query<(&mut Transform, &mut Visibility)>,
        // all posts
        Query<&Transform, With<BarrierPost>>,
    )>,
) {
    match preview_helper.preview_status() {
        BarrierPreviewStatus::None => (),

        BarrierPreviewStatus::Post { post } => {
            let mut spatial_query = queries.p0();
            let (mut post_transform, mut post_visibility) = spatial_query.get_mut(post).unwrap();

            match cursor.ground_point() {
                Some(position) => {
                    post_transform.translation = position;
                    *post_visibility = Visibility::Visible;
                }

                None => *post_visibility = Visibility::Hidden,
            }
        }

        BarrierPreviewStatus::Connecting { post, fence } => {
            // get the position the post should be placed at
            let position =
                cursor
                    .first_entity()
                    .and_then(|maybe_post| match queries.p1().get(maybe_post) {
                        // if hovering a post, add it as a snapping target and use its transform
                        Ok(snap_transform) => {
                            let snap_component = SnapPreviewPost { to: maybe_post };
                            commands.entity(post).insert(snap_component);

                            Some(snap_transform.translation)
                        }

                        // if not, default to using the cursor ground position
                        Err(_) => {
                            commands.entity(post).remove::<SnapPreviewPost>();
                            cursor.ground_point()
                        }
                    });

            let mut spatial_query = queries.p0();

            // update post preview's transform and visibility
            let (mut post_transform, mut post_visibility) = spatial_query.get_mut(post).unwrap();
            match position {
                Some(position) => {
                    post_transform.translation = position;
                    *post_visibility = Visibility::Visible;
                }

                None => *post_visibility = Visibility::Hidden,
            }

            // only update fence preview's visibility, its transform will be handled by external fence systems
            let (_, mut fence_visibility) = spatial_query.get_mut(fence).unwrap();
            *fence_visibility = match position.is_some() {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            }
        }
    };
}

/// Handles updating the fence's preview cost when its transform changes
fn update_fence_preview_cost(
    preview_helper: BarrierPreviewHelper,
    barriers: Res<Assets<BarrierData>>,
    // mut fence_preview: Query<(&mut Preview, &BarrierFence, &Transform), Changed<Transform>>,
    mut fence_preview: Query<(&mut Preview, &Transform), With<BarrierFence>>,
) {
    match preview_helper.preview_status() {
        // only change fence preview cost if it exists
        BarrierPreviewStatus::Connecting { fence, .. } => {
            let Some(barrier_data_handle) = preview_helper.data() else { return };
            let Some(barrier_data) = barriers.get(barrier_data_handle) else { return };

            let (mut preview, transform) = fence_preview.get_mut(fence).unwrap();

            preview.cost = barrier_data.fence_cost * transform.scale.x;
        }

        _ => (),
    }
}

fn on_preview_place(
    mut commands: Commands,
    mut placements: EventReader<PlacePreview>,
    barriers: Res<Assets<BarrierData>>,

    mut set: ParamSet<(
        BarrierPreviewHelper,
        Query<(&mut BarrierPost, &Transform)>,
        Query<(&mut BarrierFence, &Transform)>,
    )>,
) {
    for _ in placements.iter() {
        let Some(barrier_data_handle) = set.p0().data().cloned() else { continue };
        let Some(barrier_data) = barriers.get(&barrier_data_handle) else { continue };

        let placed_post = commands.spawn_empty().id();
        let placed_fence = commands.spawn_empty().id();

        match set.p0().preview_status() {
            BarrierPreviewStatus::None => (),

            // when placing a single post, spawn in an additional fence preview afterwards
            BarrierPreviewStatus::Post { post } => {
                let mut posts = set.p1();
                let (mut preview_post, preview_post_transform) = posts.get_mut(post).unwrap();

                // spawn permanent post entity
                commands.entity(placed_post).insert(ObjectBundle {
                    object: BarrierPost {
                        data: barrier_data_handle.clone(),
                        fences: vec![],
                    },
                    spatial: SpatialBundle {
                        transform: *preview_post_transform,
                        ..default()
                    },
                    gltf: RenderGltf {
                        handle: barrier_data.post_model.clone(),
                        mode: RenderGltfMode::Regular,
                    },
                    collider: ColliderMesh {
                        mesh: barrier_data.post_collider.clone(),
                        rb: RigidBody::Fixed,
                        membership: CollisionLayer::Object,
                    },
                });

                // spawn preview fence entity
                commands.entity(placed_fence).insert((
                    ObjectBundle {
                        object: BarrierFence {
                            data: barrier_data_handle.clone(),
                            connection: [placed_post, post],
                        },
                        spatial: SpatialBundle::default(),
                        gltf: RenderGltf {
                            handle: barrier_data.fence_model.clone(),
                            mode: RenderGltfMode::Preview,
                        },
                        collider: ColliderMesh {
                            mesh: barrier_data.fence_collider.clone(),
                            rb: RigidBody::Fixed,
                            membership: CollisionLayer::None,
                        },
                    },
                    Preview {
                        cost: 0.0, // will be updated after fence's transform is updated
                    },
                ));

                // finally link preview post with preview fence
                preview_post.fences = vec![placed_fence];
            }

            // when connecting, properly handle snapping
            BarrierPreviewStatus::Connecting { post, fence } => {
                // spawn fence regardless
                let mut fences = set.p2();
                let (mut preview_fence, preview_fence_transform) = fences.get_mut(fence).unwrap();

                // spawn permanent fence entity
                commands.entity(placed_fence).insert(ObjectBundle {
                    object: BarrierFence {
                        data: barrier_data_handle.clone(),
                        connection: [preview_fence.connection[0], placed_post],
                    },
                    spatial: SpatialBundle {
                        transform: preview_fence_transform.clone(),
                        ..default()
                    },
                    gltf: RenderGltf {
                        handle: barrier_data.fence_model.clone(),
                        mode: RenderGltfMode::Regular,
                    },
                    collider: ColliderMesh {
                        mesh: barrier_data.fence_collider.clone(),
                        rb: RigidBody::Fixed,
                        membership: CollisionLayer::Object,
                    },
                });

                // update preview fence's first connection
                preview_fence.connection[0] = placed_post;

                // handle different behaviors if snapping to a placed fence post
                let maybe_snap_post = set.p0().snap_post();
                let mut posts = set.p1();

                match maybe_snap_post {
                    // if a post is to be snapped to, don't spawn in a new post
                    Some(snap_post_entity) => {
                        // don't spawn a new post and despawn the fence preview
                        commands.entity(placed_post).despawn_recursive();
                        commands.entity(fence).despawn_recursive();

                        // update snap post's data to connect with spawned fence
                        let (mut snap_post, _) = posts.get_mut(snap_post_entity).unwrap();
                        snap_post.fences.push(placed_fence);
                    }

                    // if no snap post, spawn a post like regular
                    None => {
                        let (_, preview_post_transform) = posts.get(post).unwrap();

                        commands.entity(placed_post).insert(ObjectBundle {
                            object: BarrierPost {
                                data: barrier_data_handle.clone(),
                                fences: vec![],
                            },
                            spatial: SpatialBundle {
                                transform: *preview_post_transform,
                                ..default()
                            },
                            gltf: RenderGltf {
                                handle: barrier_data.post_model.clone(),
                                mode: RenderGltfMode::Regular,
                            },
                            collider: ColliderMesh {
                                mesh: barrier_data.post_collider.clone(),
                                rb: RigidBody::Fixed,
                                membership: CollisionLayer::Object,
                            },
                        });
                    }
                }
            }
        }
    }
}
