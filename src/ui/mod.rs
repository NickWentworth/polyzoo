use bevy::{ecs::system::SystemParam, prelude::*};

mod tabs;
mod toolbar;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>()
            .add_systems(Startup, toolbar::setup_toolbar)
            .add_systems(Update, tabs::tab_group::<toolbar::BuyMenu>);
    }
}

#[derive(Resource)]
struct UiTheme {
    white: Color,
    medium: Color,
    dark: Color,
    accent: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            white: Color::hex("#E8EDED").unwrap(),
            medium: Color::hex("#4F6367").unwrap(),
            dark: Color::hex("#354345").unwrap(),
            accent: Color::hex("#F26430").unwrap(),
        }
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
