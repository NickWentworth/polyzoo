use super::{Object, ObjectGroup};
use bevy::prelude::*;

pub fn add(world: &mut World) -> Vec<HandleUntyped> {
    // load all required data
    let asset_server = world.resource::<AssetServer>();
    let objects = vec![
        Object {
            name: "Dark Rock 1",
            cost: 25.0,
            model: asset_server.load("nature/dark_rock_1.glb#Scene0"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Dark Rock 2",
            cost: 25.0,
            model: asset_server.load("nature/dark_rock_2.glb#Scene0"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Dark Rock 3",
            cost: 25.0,
            model: asset_server.load("nature/dark_rock_3.glb#Scene0"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
    ];

    // add each object into assets collection and map to its handle
    let mut object_assets = world.resource_mut::<Assets<Object>>();
    objects
        .into_iter()
        .map(|obj| object_assets.add(obj).clone_untyped())
        .collect()
}
