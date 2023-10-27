use bevy::prelude::*;

mod components;
mod data;
mod placement;

pub use data::BarrierData;

pub struct BarrierPlugin;
impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            components::BarrierComponentPlugin,
            data::BarrierDataPlugin,
            placement::BarrierPlacementPlugin,
        ));
    }
}
