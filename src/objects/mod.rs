use bevy::prelude::*;

mod barrier;
mod prop;

pub mod utility;

pub use barrier::BarrierData;
pub use prop::PropData;

pub struct ObjectPlugin;
impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((prop::PropPlugin, barrier::BarrierPlugin))
            .add_systems(
                Update,
                (
                    utility::handle_mesh_changes,
                    utility::handle_collider_changes,
                ),
            );
    }
}

/// Common data that is required for an object in the world
#[derive(Bundle)]
struct ObjectBundle<O: Component> {
    pub object: O,
    pub spatial: SpatialBundle,
    pub gltf: utility::RenderGltf,
    pub collider: utility::ColliderMesh,
}
