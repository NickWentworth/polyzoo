use bevy::{
    gltf::GltfMesh,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

mod data;
mod interaction;
mod placement;
mod utility;

pub use interaction::{
    DeselectObject, SelectObject, SellSelectedObject, TransformGizmoSelectedObject,
};
pub use placement::ChangePlacementObject;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Object>()
            .init_resource::<data::ObjectLoader>()
            .add_systems(Update, utility::handle_object_changes);

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

impl Object {
    /// Percentage that selling will refund the original cost
    const SELL_MODIFIER: f32 = 0.7;

    /// The calculated amount this object is worth from selling
    pub fn sell_value(&self) -> Currency {
        self.cost * Self::SELL_MODIFIER
    }
}

/// Type alias for money variables
pub type Currency = f32;

/// Different classifications for placeable objects
#[derive(PartialEq)]
pub enum ObjectGroup {
    BarrierPost(Handle<Object>), // stores the fence object that connects posts
    BarrierFence,
    Rock,
}
