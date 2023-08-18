use bevy::prelude::*;

mod barrier;

pub use self::barrier::Barrier;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Barrier>().init_resource::<ObjectLoader>();
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

        Self { _data: data }
    }
}
