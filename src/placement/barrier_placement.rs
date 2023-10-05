use super::{
    utility::{RenderGltf, RenderGltfMode},
    PreviewData,
};
use crate::{camera::CursorRaycast, objects::BarrierData};
use bevy::prelude::*;

#[derive(Clone)]
pub struct BarrierPlacementPlugin;
impl Plugin for BarrierPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_post_movement,
                handle_fence_movement,
                on_preview_place,
            ),
        );
    }
}

impl PreviewData for Handle<BarrierData> {
    fn spawn_preview(&self, commands: &mut Commands) -> Vec<Entity> {
        commands.spawn_barrier_preview(self).into()
    }
}

trait BarrierCommandsExtension {
    fn spawn_barrier_preview(&mut self, barrier_handle: &Handle<BarrierData>) -> [Entity; 2];
}

impl BarrierCommandsExtension for Commands<'_, '_> {
    fn spawn_barrier_preview(&mut self, barrier_handle: &Handle<BarrierData>) -> [Entity; 2] {
        // spawn in post preview
        let barrier_handle_post = barrier_handle.clone();
        let post = self
            .spawn_empty()
            .add(move |id, world: &mut World| {
                // get the barrier data from assets collection
                let barriers = world.resource::<Assets<BarrierData>>();
                let barrier_data = barriers.get(&barrier_handle_post).unwrap();

                // store any required world-related data before inserting
                let post_mesh = barrier_data.post_mesh.clone();

                // insert all components into created entity
                world.entity_mut(id).insert((
                    SpatialBundle::default(),
                    PostPreview {
                        data: barrier_handle_post,
                    },
                    RenderGltf {
                        handle: post_mesh,
                        mode: RenderGltfMode::Preview,
                    },
                ));
            })
            .id();

        // spawn in fence preview
        let barrier_handle_fence = barrier_handle.clone();
        let fence = self
            .spawn_empty()
            .add(|id, world: &mut World| {
                // get the barrier data from assets collection
                let barriers = world.resource::<Assets<BarrierData>>();
                let barrier_data = barriers.get(&barrier_handle_fence).unwrap();

                // store any required world-related data before inserting
                let fence_mesh = barrier_data.fence_mesh.clone();

                // insert all components into created entity
                world.entity_mut(id).insert((
                    SpatialBundle::default(),
                    FencePreview {
                        data: barrier_handle_fence,
                    },
                    RenderGltf {
                        handle: fence_mesh,
                        mode: RenderGltfMode::Preview,
                    },
                ));
            })
            .id();

        [post, fence]
    }
}

/// Component marking the currently previewed barrier post
#[derive(Component)]
struct PostPreview {
    data: Handle<BarrierData>,
}

/// Component marking the currently previewed barrier fence
#[derive(Component)]
struct FencePreview {
    data: Handle<BarrierData>,
}

fn handle_post_movement(
    cursor: CursorRaycast,
    mut post_preview: Query<(&mut Transform, &mut Visibility), With<PostPreview>>,
) {
    if let Ok((mut transform, mut visibility)) = post_preview.get_single_mut() {
        match cursor.ground_point() {
            Some(position) => {
                transform.translation = position;
                *visibility = Visibility::Visible;
            }

            None => *visibility = Visibility::Hidden,
        }
    }
}

fn handle_fence_movement(
    post_preview: Query<(&Transform, &Visibility), With<PostPreview>>,
    mut fence_preview: Query<
        (&mut Transform, &mut Visibility),
        (With<FencePreview>, Without<PostPreview>),
    >,
) {
    let Ok((post_transform, post_visibility)) = post_preview.get_single() else { return };
    let Ok((mut fence_transform, mut fence_visibility)) = fence_preview.get_single_mut() else { return };

    if *post_visibility == Visibility::Hidden {
        *fence_visibility = Visibility::Hidden;
    } else {
        // TODO - connect fence to some previously-placed post
        // for now, just connect fence from origin to post
        let from = Vec3::ZERO;
        let to = post_transform.translation;

        let midpoint = from.lerp(to, 0.5);
        let angle = -f32::atan2(to.z - from.z, to.x - from.x);
        let distance = from.distance(to);

        *fence_transform = Transform {
            translation: midpoint,
            rotation: Quat::from_rotation_y(angle),
            scale: Vec3::new(distance, 1.0, 1.0),
        };
        *fence_visibility = Visibility::Visible;
    }
}

fn on_preview_place() {
    // TODO - place post and fence into the world
}
