use bevy::prelude::*;

pub struct Barrier {
    pub name: &'static str,
    pub cost: f32,
    pub model: Handle<Scene>,
}

pub fn init_barriers(asset_server: &AssetServer) -> Vec<Barrier> {
    vec![
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
    ]
}
