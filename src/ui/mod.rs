use bevy::prelude::*;

mod toolbar;

// TODO - block cursor raycasts that are within ui bounds as needed

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, toolbar::setup_toolbar);
    }
}
