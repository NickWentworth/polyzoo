use crate::{
    camera::CursorRaycast,
    objects::{Object, ObjectGroup},
};
use bevy::prelude::*;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Placement>()
            .add_systems(Update, handle_preview)
            .add_systems(Update, handle_placement);
    }
}

/// Marker trait for the preview of the currently selected object
#[derive(Component)]
struct Preview;

#[derive(Resource)]
pub struct Placement {
    /// Handle to the object currently being placed, if it exists
    pub object: Option<Handle<Object>>,
}

impl FromWorld for Placement {
    /// Setup the placement resource and insert a preview bundle
    fn from_world(world: &mut World) -> Self {
        world.spawn((
            SceneBundle {
                visibility: Visibility::Hidden,
                ..default()
            },
            Preview,
        ));

        Self { object: None }
    }
}

fn handle_preview(
    placement: Res<Placement>,
    objects: Res<Assets<Object>>,
    cursor_raycast: CursorRaycast,

    mut preview: Query<(&mut Transform, &mut Visibility, &mut Handle<Scene>), With<Preview>>,
) {
    let (mut transform, mut visibility, mut scene_handle) = preview.single_mut();

    match &placement.object {
        Some(handle) => {
            // update object's position and model if needed
            if let (Some(point), Some(object)) = (cursor_raycast.point(), objects.get(&handle)) {
                // show preview of the correct object at mouse position
                transform.translation = point;
                *visibility = Visibility::Visible;
                *scene_handle = object.model.clone();
            } else {
                // else, just hide the model
                *visibility = Visibility::Hidden;
            }
        }

        // no need to update anything else, just ensure preview is hidden
        None => *visibility = Visibility::Hidden,
    }
}

fn handle_placement(
    mut commands: Commands,

    placement: Res<Placement>,
    objects: Res<Assets<Object>>,
    mouse_buttons: Res<Input<MouseButton>>,

    preview: Query<(&Transform, &Visibility), With<Preview>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(object_handle) = &placement.object {
            // copy over some components from preview object
            let (transform, visibility) = preview.single();

            // TODO - more properly verify valid object placement
            if *visibility != Visibility::Hidden {
                let object = objects.get(object_handle).unwrap();

                // spawn in the object at the same location as the preview
                commands.spawn(SceneBundle {
                    scene: object.model.clone(),
                    transform: transform.clone(),
                    ..default()
                });

                // special cases for spaning of various object groups
                match &object.group {
                    // barriers must spawn in fences between posts
                    ObjectGroup::Barrier(fence_model) => {
                        // TODO - actually spawn fence between posts

                        // let midpoint = from.lerp(to, 0.5);
                        // let length = from.distance(to);
                        // let angle = -f32::atan2(to.z - from.z, to.x - from.x);
                    }

                    _ => (),
                }
            }
        }
    }
}
