use bevy::prelude::*;

mod components;
mod data;
mod placement;

pub use components::Prop;
pub use data::PropData;

pub struct PropPlugin;
impl Plugin for PropPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((data::PropDataPlugin, placement::PropPlacementPlugin));
    }
}
