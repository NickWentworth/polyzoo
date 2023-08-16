use bevy::{ecs::system::SystemParam, prelude::*};

mod toolbar;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, toolbar::setup_toolbar);
    }
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
