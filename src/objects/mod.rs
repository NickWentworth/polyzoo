use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

mod barrier;
mod nature;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Object>().init_resource::<ObjectLoader>();
    }
}

/// Resource containing all pre-loaded object data in untyped handles, so that it isn't unloaded
///
/// For now, all objects are also stored in the ui, so data stems from there
#[derive(Resource)]
struct ObjectLoader {
    _data: Vec<HandleUntyped>,
}

impl FromWorld for ObjectLoader {
    fn from_world(world: &mut World) -> Self {
        let mut data = Vec::new();

        data.extend(barrier::add(world));
        data.extend(nature::add(world));

        Self { _data: data }
    }
}

// TODO - barriers should also be objects, find a way to define a barrier as an object with an extra field
/// Base asset for a placeable object
#[derive(TypeUuid, TypePath)]
#[uuid = "5981b7ce-340e-4e24-a004-7139c8860455"]
pub struct Object {
    pub name: &'static str,
    pub cost: f32,
    pub model: Handle<Scene>,
    pub image: Handle<Image>,
    pub group: ObjectGroup,
}

/// Different classifications for placeable objects
#[derive(PartialEq)]
pub enum ObjectGroup {
    Barrier(Handle<Scene>), // stores the tileable fence model connecting posts
    Rock,
}

impl Object {
    // TODO - add commas between thousands, millions, etc.
    /// Return a formatted cost for the barrier
    ///
    /// Ex: 1000 => $1,000
    pub fn formatted_cost(&self) -> String {
        format!("${:.0}", self.cost)
    }
}
