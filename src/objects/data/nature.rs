use super::{Object, ObjectGroup};
use bevy::prelude::*;

pub fn add(world: &mut World) -> Vec<HandleUntyped> {
    // load all required data
    let asset_server = world.resource::<AssetServer>();
    let objects = vec![
        Object {
            name: "Dark Rock 1",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh0"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Dark Rock 2",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh1"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Dark Rock 3",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh2"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Light Rock 1",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh3"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Light Rock 2",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh4"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Light Rock 3",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh5"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Sandy Rock 1",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh6"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Sandy Rock 2",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh7"),
            image: asset_server.load("test.png"),
            group: ObjectGroup::Rock,
        },
        Object {
            name: "Sandy Rock 3",
            cost: 25.0,
            mesh: asset_server.load("nature/rocks.glb#Mesh8"),
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
