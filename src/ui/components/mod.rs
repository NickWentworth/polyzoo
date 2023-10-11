use bevy::prelude::*;

mod message_box;
mod toolbar;

use super::*;

pub struct UiComponentsPlugin;
impl Plugin for UiComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((message_box::MessageBoxPlugin, toolbar::ToolbarPlugin));
    }
}
