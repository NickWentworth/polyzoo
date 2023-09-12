use super::Object;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ObjectLoader {
    _data: Vec<Handle<Object>>,
}

impl FromWorld for ObjectLoader {
    fn from_world(world: &mut World) -> Self {
        // load all required data
        let asset_server = world.resource::<AssetServer>();
        let objects = vec![
            Object {
                name: "Dark Rock 1",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh0"),
            },
            Object {
                name: "Dark Rock 2",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh1"),
            },
            Object {
                name: "Dark Rock 3",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh2"),
            },
            Object {
                name: "Light Rock 1",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh3"),
            },
            Object {
                name: "Light Rock 2",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh4"),
            },
            Object {
                name: "Light Rock 3",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh5"),
            },
            Object {
                name: "Sandy Rock 1",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh6"),
            },
            Object {
                name: "Sandy Rock 2",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh7"),
            },
            Object {
                name: "Sandy Rock 3",
                cost: 25.0,
                icon: asset_server.load("test.png"),
                mesh: asset_server.load("nature/rocks.glb#Mesh8"),
            },
        ];

        // add each object into assets collection and map to its handle
        let mut object_assets = world.resource_mut::<Assets<Object>>();
        Self {
            _data: objects
                .into_iter()
                .map(|obj| object_assets.add(obj))
                .collect(),
        }
    }
}
