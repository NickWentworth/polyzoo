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

#[derive(Component)]
struct Post {
    next: Option<Entity>,
    prev: Option<Entity>,
}

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

// TODO - this can likely be split between handle_preview_movement and handle_preview_model, as model doesn't need to change each frame
// TODO - show fence on preview
fn handle_preview(
    placement: Res<Placement>,
    objects: Res<Assets<Object>>,
    cursor_raycast: CursorRaycast,

    // single entity with preview component
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

    // single entity with preview component
    preview: Query<(&Transform, &Visibility), With<Preview>>,
    // all barrier post components
    mut posts: Query<(&mut Post, &Transform, Entity)>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(object_handle) = &placement.object {
            // copy over some components from preview object
            let (preview_transform, preview_visibility) = preview.single();

            // TODO - more properly verify valid object placement
            if *preview_visibility != Visibility::Hidden {
                let object = objects.get(object_handle).unwrap();

                // spawn in the object at the same location as the preview
                let mut entity = commands.spawn(SceneBundle {
                    scene: object.model.clone(),
                    transform: preview_transform.clone(),
                    ..default()
                });

                // special cases for spaning of various object groups
                match &object.group {
                    // barriers must spawn in fences between posts
                    ObjectGroup::Barrier(fence_model) => {
                        // find the post to connect to by searching for one without next component
                        if let Some((mut previous_post, previous_transform, previous_entity)) =
                            posts.iter_mut().find(|(post, ..)| post.next.is_none())
                        {
                            // update previous post component
                            previous_post.next = Some(entity.id());

                            // attach post component to spawned-in post model
                            entity.insert(Post {
                                next: None,
                                prev: Some(previous_entity),
                            });

                            // TODO - add multiple fence types: tiled, stretched, etc.
                            // spawn in fence connecting two posts
                            let from = previous_transform.translation;
                            let to = preview_transform.translation;

                            let length = from.distance(to);
                            let angle = -f32::atan2(to.z - from.z, to.x - from.x);

                            commands.spawn(SceneBundle {
                                scene: fence_model.clone(),
                                transform: Transform {
                                    translation: from,
                                    rotation: Quat::from_rotation_y(angle),
                                    scale: Vec3::new(length, 1.0, 1.0),
                                },
                                ..default()
                            });
                        } else {
                            // if no previous post, no need to spawn a fence connector
                            entity.insert(Post {
                                next: None,
                                prev: None,
                            });
                        }
                    }

                    _ => (),
                }
            }
        }
    }
}
