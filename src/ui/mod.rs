use crate::objects::Currency;
use bevy::{ecs::system::SystemParam, prelude::*};

mod misc;
mod tabs;
mod theme;
mod toolbar;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // theme systems
            .init_resource::<theme::UiTheme>()
            .add_systems(Update, theme::handle_interactions)
            // toolbar systems
            .add_systems(Startup, toolbar::setup_toolbar)
            .add_systems(Update, toolbar::toolbar_interactions)
            .add_systems(Update, toolbar::toolbar_callbacks)
            .add_systems(Update, tabs::tab_group::<toolbar::BuyMenu>)
            // misc systems
            .add_systems(Startup, misc::setup_deselect_prompt)
            .add_systems(Update, misc::deselect_prompt_interactions)
            .add_systems(Update, misc::deselect_prompt_callbacks);
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

// TODO - add commas between thousands, millions, etc.
/// Nicely formats the currency for display
fn formatted_currency(amount: Currency) -> String {
    format!("${:.0}", amount)
}
