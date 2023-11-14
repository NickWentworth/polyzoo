use super::components::{BarrierFence, BarrierPost, PossibleBarrierCycle};
use crate::{
    camera::CursorRaycast,
    objects::{
        utility::{ColliderMesh, CollisionLayer, RenderGltf, RenderGltfMode},
        BarrierData, ObjectBundle,
    },
    placement::{PlacePreview, Preview, PreviewData},
};
use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier3d::prelude::*;

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

    mut possible_cycles: EventWriter<PossibleBarrierCycle>,

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

        let mut placed_post_bundle = ObjectBundle {
            object: BarrierPost {
                data: barrier_data_handle.clone(),
                fences: vec![],
            },
            spatial: SpatialBundle::default(),
            gltf: RenderGltf {
                handle: barrier_data.post_model.clone(),
                mode: RenderGltfMode::Regular,
            },
            collider: ColliderMesh {
                mesh: barrier_data.post_collider.clone(),
                rb: RigidBody::Fixed,
                membership: CollisionLayer::Object,
            },
        };

        let mut placed_fence_bundle = ObjectBundle {
            object: BarrierFence {
                data: barrier_data_handle.clone(),
                connection: [placed_post, placed_post], // IMPORTANT: overwrite this with whatever the fence should be connected to
            },
            spatial: SpatialBundle::default(),
            gltf: RenderGltf {
                handle: barrier_data.fence_model.clone(),
                mode: RenderGltfMode::Regular,
            },
            collider: ColliderMesh {
                mesh: barrier_data.fence_collider.clone(),
                rb: RigidBody::Fixed,
                membership: CollisionLayer::Object,
            },
        };

        match set.p0().preview_status() {
            BarrierPreviewStatus::None => {
                // shouldn't reach here, but despawn the placed post and fence regardless
                commands.entity(placed_post).despawn_recursive();
                commands.entity(placed_fence).despawn_recursive();
            }

            // when placing a single post, spawn in an additional fence preview afterwards
            BarrierPreviewStatus::Post { post } => {
                let mut posts = set.p1();
                let (mut preview_post, preview_post_transform) = posts.get_mut(post).unwrap();

                // link preview post with preview fence
                preview_post.fences = vec![placed_fence];

                // set permanent placed post's transform to preview's transform
                placed_post_bundle.spatial.transform = *preview_post_transform;

                // set placed fence into preview mode
                commands.entity(placed_fence).insert(Preview { cost: 0.0 });
                placed_fence_bundle.gltf.mode = RenderGltfMode::Preview;
                placed_fence_bundle.collider.membership = CollisionLayer::None;

                // connect placed fence from permanently placed post to preview post
                placed_fence_bundle.object.connection = [placed_post, post];
            }

            // when connecting, properly handle snapping
            BarrierPreviewStatus::Connecting { post, fence } => {
                // always spawn in a permanently placed fence
                let mut fences = set.p2();
                let (mut preview_fence, preview_fence_transform) = fences.get_mut(fence).unwrap();

                placed_fence_bundle.spatial.transform = *preview_fence_transform;
                placed_fence_bundle.object.connection[0] = preview_fence.connection[0];

                // ensure the previous post the preview fence was connected to has its fences updated properly
                // store this for later use before overwriting the preview fence
                let previous_post_entity = preview_fence.connection[0];
                preview_fence.connection[0] = placed_post;

                // connect previous post with the placed permanent fence
                let mut posts = set.p1();
                let (mut previous_post, _) = posts.get_mut(previous_post_entity).unwrap();
                previous_post.fences.push(placed_fence);

                match set.p0().snap_post() {
                    // if there is no post to snap to, spawn a permanent post and fence
                    None => {
                        // connect placed fence to placed post
                        placed_fence_bundle.object.connection[1] = placed_post;

                        // update preview post's fence to be the permanent placed fence
                        let posts = set.p1();
                        let (_, preview_post_transform) = posts.get(post).unwrap();

                        placed_post_bundle.object.fences = vec![placed_fence];
                        placed_post_bundle.spatial.transform = *preview_post_transform;
                    }

                    // if there is a snap post, connect everything as required
                    Some(snap_post_entity) => {
                        // connect placed fence to snap post
                        placed_fence_bundle.object.connection[1] = snap_post_entity;

                        // cleanup preview post's fences
                        let mut posts = set.p1();
                        let (mut preview_post, _) = posts.get_mut(post).unwrap();
                        preview_post.fences = vec![];

                        // add placed fence as a connection to the snap post
                        let (mut snap_post, _) = posts.get_mut(snap_post_entity).unwrap();
                        snap_post.fences.push(placed_fence);

                        // a new cycle could have been formed
                        possible_cycles.send(PossibleBarrierCycle {
                            root: snap_post_entity,
                        });

                        // finally despawn the placed post and fence preview to start a new barrier cycle
                        commands.entity(placed_post).despawn_recursive();
                        commands.entity(fence).despawn_recursive();
                    }
                }
            }
        }

        // finally insert general object bundles into post and fence entities, if they still exist
        commands.try_insert(placed_post, placed_post_bundle);
        commands.try_insert(placed_fence, placed_fence_bundle);
    }
}

trait TryInsertCommand {
    fn try_insert(&mut self, entity: Entity, bundle: impl Bundle);
}

impl<'w, 's> TryInsertCommand for Commands<'w, 's> {
    /// Tries to insert the given `Bundle` into the `Entity`, checking if it has been despawned before inserting
    fn try_insert(&mut self, entity: Entity, bundle: impl Bundle) {
        self.add(move |world: &mut World| {
            if let Some(mut entity_ref) = world.get_entity_mut(entity) {
                entity_ref.insert(bundle);
            }
        })
    }
}
