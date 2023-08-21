use super::{Object, ObjectGroup};
use bevy::prelude::*;

/// Add all barriers into the assets collection
pub fn add(world: &mut World) -> Vec<HandleUntyped> {
    // load all required data
    let asset_server = world.resource::<AssetServer>();
    let objects = vec![Object {
        name: "Wooden Barrier",
        cost: 10.0,
        model: asset_server.load("barriers/wooden_barrier.glb#Scene0"),
        image: asset_server.load("test.png"),
        group: ObjectGroup::Barrier(asset_server.load("barriers/wooden_fence.glb#Scene0")),
    }];

    // add each object into assets collection and map to its handle
    let mut object_assets = world.resource_mut::<Assets<Object>>();
    objects
        .into_iter()
        .map(|obj| object_assets.add(obj).clone_untyped())
        .collect()
}
