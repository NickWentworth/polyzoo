use self::barrier::{init_barriers, Barrier};
use bevy::prelude::*;

mod barrier;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObjectDatabase>();
    }
}

/// Resource containing all pre-loaded object data
#[derive(Resource)]
pub struct ObjectDatabase {
    barriers: Vec<Barrier>,
}

impl FromWorld for ObjectDatabase {
    fn from_world(world: &mut World) -> Self {
        Self {
            barriers: init_barriers(world.resource::<AssetServer>()),
        }
    }
}

impl ObjectDatabase {
    /// Get a slice containing all barrier data
    pub fn barriers(&self) -> &[Barrier] {
        &self.barriers
    }
}
