use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

#[derive(TypeUuid, TypePath)]
#[uuid = "b0aa393f-5641-480d-a0bf-54e862a50c20"]
pub struct Barrier {
    pub name: &'static str,
    pub cost: f32,
    pub model: Handle<Scene>,
}

/// Add all barriers into the assets collection
pub fn add(world: &mut World) -> Vec<HandleUntyped> {
    // load all required data for barriers
    let asset_server = world.resource::<AssetServer>();
    let barriers = vec![
        Barrier {
            name: "Wooden Barrier",
            cost: 10.0,
            model: asset_server.load("barriers/wooden_barrier.glb#Scene0"),
        },
        // TEMP
        Barrier {
            name: "Test Barrier",
            cost: 20.0,
            model: asset_server.load("barriers/wooden_barrier.glb#Scene0"),
        },
    ];

    // add each barrier into assets collection and map to its handle
    let mut barrier_assets = world.resource_mut::<Assets<Barrier>>();
    barriers
        .into_iter()
        .map(|barrier| barrier_assets.add(barrier).clone_untyped())
        .collect()
}
