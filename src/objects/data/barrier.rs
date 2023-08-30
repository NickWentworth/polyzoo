use super::{Object, ObjectGroup};
use bevy::prelude::*;

pub fn add(world: &mut World) -> Vec<HandleUntyped> {
    // load all required data as (post, fence) pairs
    let asset_server = world.resource::<AssetServer>();
    let objects = vec![(
        Object {
            name: "Concrete Barrier Post",
            cost: 20.0,
            mesh: asset_server.load("barriers/concrete_post.glb#Mesh0"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::BarrierPost(Handle::default()),
        },
        Object {
            name: "Concrete Barrier Fence",
            cost: 10.0,
            mesh: asset_server.load("barriers/concrete_fence.glb#Mesh0"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::BarrierFence,
        },
    )];

    // add each object into assets collection and map to its handle
    let mut object_assets = world.resource_mut::<Assets<Object>>();
    objects
        .into_iter()
        .map(|(post, fence)| {
            let fence_handle = object_assets.add(fence);
            let post_handle = object_assets.add(Object {
                // insert fence handle into post data
                group: ObjectGroup::BarrierPost(fence_handle.clone()),
                ..post
            });

            [fence_handle.clone_untyped(), post_handle.clone_untyped()]
        })
        .flatten()
        .collect()
}
