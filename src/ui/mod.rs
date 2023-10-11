use crate::Currency;
use bevy::{ecs::system::SystemParam, prelude::*};

mod components;
// mod selection_panel;
mod tabs;
mod theme;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            theme::UiThemePlugin,
            components::UiComponentsPlugin,
        ));
    }
}

/// Allows a type to be displayed within the ui
pub trait UiDisplay {
    fn name(&self) -> String;
    fn image(&self) -> UiImage;
    fn text(&self) -> String;
}

/// Marker component for ui that should block raycasts from the camera
#[derive(Component)]
struct BlockCameraRaycast;

/// Helper system param for external systems to use when querying ui
#[derive(SystemParam)]
pub struct UiQuery<'w, 's> {
    blocking_nodes:
        Query<'w, 's, (&'static GlobalTransform, &'static Node), With<BlockCameraRaycast>>,
}

impl<'w, 's> UiQuery<'w, 's> {
    /// Detect if the given point is contained within ui nodes that should block raycasts
    pub fn within_blocking_ui(&self, point: Vec2) -> bool {
        for (global_transform, node) in self.blocking_nodes.iter() {
            // generate bounding box for this node
            let (scale, _, transform) = global_transform.to_scale_rotation_translation();
            let size = node.size();

            let bounds = Rect::from_center_size(transform.truncate(), size * scale.truncate());

            // if bounds contains the point, the raycast should be blocked
            if bounds.contains(point) {
                return true;
            }
        }

        false
    }
}
