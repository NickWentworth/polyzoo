use crate::{
    camera::CursorRaycast,
    objects::{Object, ObjectGroup},
    zoo::ZooBalanceChange,
};
use bevy::prelude::*;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Placement>()
            .add_event::<ChangePreview>()
            .add_systems(
                Update,
                (
                    handle_preview_change,
                    handle_preview_movement,
                    handle_fence_preview_movement,
                    handle_placement,
                ),
            );
    }
}

/// Marker trait for the preview of the currently selected object
#[derive(Component)]
struct Preview;

/// Marker trait for the preview of the currently selected barrier's fence
#[derive(Component)]
struct FencePreview;

#[derive(Component)]
struct Post {
    next: Option<Entity>,
    prev: Option<Entity>,
}

#[derive(Resource)]
pub struct Placement {
    /// Handle to the object currently being placed, if it exists
    object: Option<Handle<Object>>,
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

/// Event for changing the preview object
///
/// Stores `Some(handle)` for the new object, or `None` if deselecting the current object
#[derive(Event)]
pub struct ChangePreview(pub Option<Handle<Object>>);

// TODO - handle based on object group, ex: for barriers, spawn in a fence preview as well
/// Handles the preview's model being changed by `ChangePreview` events
fn handle_preview_change(
    mut commands: Commands,
    mut change_preview_reader: EventReader<ChangePreview>,

    mut placement: ResMut<Placement>,
    objects: Res<Assets<Object>>,

    mut preview: Query<&mut Handle<Scene>, With<Preview>>,
    other_previews: Query<Entity, With<FencePreview>>,
) {
    let mut preview_scene = preview.single_mut();

    for change_event in change_preview_reader.iter() {
        // set the placement resource's object field
        placement.object = change_event.0.clone();

        // despawn all extra preview entities
        for other_entity in other_previews.iter() {
            commands.entity(other_entity).despawn();
        }

        match &change_event.0 {
            Some(object_handle) => {
                // set preview's scene handle to object's model
                let object = objects.get(object_handle).unwrap();
                *preview_scene = object.model.clone();

                // handle special cases for object groups
                match &object.group {
                    ObjectGroup::Barrier(fence_model) => {
                        // spawn in a new fence preview entity
                        commands.spawn((
                            SceneBundle {
                                scene: fence_model.clone(),
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                            FencePreview,
                        ));
                    }

                    _ => (),
                }
            }

            None => *preview_scene = Handle::default(),
        }
    }
}

/// Handles movement of the preview using `CursorRaycast` system param
fn handle_preview_movement(
    cursor_raycast: CursorRaycast,
    mut preview: Query<(&mut Transform, &mut Visibility), With<Preview>>,
) {
    let (mut transform, mut visibility) = preview.single_mut();

    match cursor_raycast.point() {
        Some(point) => {
            transform.translation = point;
            *visibility = Visibility::Visible;
        }

        None => *visibility = Visibility::Hidden,
    }
}

/// Handles movement of fence preview (if it exists) using `CursorRaycast` system param
fn handle_fence_preview_movement(
    cursor_raycast: CursorRaycast,

    posts: Query<(&Post, &Transform), (With<Post>, Without<FencePreview>)>,
    mut fence_preview: Query<
        (&mut Transform, &mut Visibility),
        (With<FencePreview>, Without<Post>),
    >,
) {
    if let Some((mut fence_transform, mut fence_visibility)) = fence_preview.get_single_mut().ok() {
        match cursor_raycast.point() {
            Some(point) => {
                // if there is a previous post, correctly orient the fence preview
                if let Some((_, post_transform)) =
                    posts.iter().find(|(post, _)| post.next.is_none())
                {
                    let from = post_transform.translation;
                    let to = point;

                    let length = from.distance(to);
                    let angle = -f32::atan2(to.z - from.z, to.x - from.x);

                    *fence_transform = Transform {
                        translation: from,
                        rotation: Quat::from_rotation_y(angle),
                        scale: Vec3::new(length, 1.0, 1.0),
                    };
                    *fence_visibility = Visibility::Visible;
                }
            }

            None => *fence_visibility = Visibility::Hidden,
        }
    }
}

/// Handles the placement of objects
fn handle_placement(
    mut commands: Commands,

    placement: Res<Placement>,
    objects: Res<Assets<Object>>,
    mouse_buttons: Res<Input<MouseButton>>,

    mut balance_change: EventWriter<ZooBalanceChange>,

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

                // reduce zoo balance by object's cost
                balance_change.send(ZooBalanceChange {
                    amount: -object.cost,
                });

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
