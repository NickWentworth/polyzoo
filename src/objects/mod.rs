use bevy::prelude::*;

mod barrier;
mod prop;

pub use barrier::BarrierData;
pub use prop::PropData;

pub struct ObjectPlugin;
impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((prop::PropPlugin, barrier::BarrierPlugin));
    }
}
