use bevy::prelude::*;

mod barrier;
mod nature;

use super::{Object, ObjectGroup};

/// Resource containing all pre-loaded object data in untyped handles, so that it isn't unloaded
///
/// For now, all objects are also stored in the ui, so data stems from there
#[derive(Resource)]
pub struct ObjectLoader {
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
