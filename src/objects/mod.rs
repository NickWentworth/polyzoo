use bevy::{
    gltf::GltfMesh,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

mod data;
mod interaction;
mod placement;

pub use interaction::SelectObject;
pub use placement::ChangePlacementObject;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Object>()
            .init_resource::<data::ObjectLoader>();

        placement::build(app);
        interaction::build(app);
    }
}

/// Base asset for a placeable object
#[derive(TypeUuid, TypePath)]
#[uuid = "5981b7ce-340e-4e24-a004-7139c8860455"]
pub struct Object {
    pub name: &'static str,
    pub cost: Currency,
    pub mesh: Handle<GltfMesh>,
    pub image: Handle<Image>,
    pub group: ObjectGroup,
}

/// Type alias for money variables
pub type Currency = f32;

/// Different classifications for placeable objects
#[derive(PartialEq)]
pub enum ObjectGroup {
    Barrier(Handle<GltfMesh>), // stores the fence model connecting posts
    Rock,
}
