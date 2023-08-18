use crate::camera::CursorRaycast;
use bevy::prelude::*;

pub struct PlacementPlugin;
impl Plugin for PlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_placement)
            .add_systems(Update, handle_placement);
    }
}

/// Stores required information to build the border of a habitat
#[derive(Component)]
struct Post {
    prev: Option<Entity>,
    next: Option<Entity>,
}

fn setup_placement(mut commands: Commands, assets: Res<AssetServer>) {
    // spawn the first preview post
    commands.spawn((
        SceneBundle {
            scene: assets.load("barriers/wooden_barrier.glb#Scene0"),
            ..default()
        },
        Post {
            prev: None,
            next: None,
        },
    ));
}

fn handle_placement(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut posts: Query<(&mut Post, &mut Transform, &mut Visibility, Entity)>,

    cursor_raycast: CursorRaycast,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let (mut post, mut transform, mut visibility, entity) = posts
        .iter_mut()
        .find(|(post, ..)| post.next.is_none())
        .unwrap();

    match cursor_raycast.point() {
        // if the mouse is hovering over terrain, show the preview post
        Some(point) => {
            // set it to visible and update translation
            *visibility = Visibility::Visible;
            transform.translation = point;

            // if the place button is pressed, spawn a new post, leaving the old post where it last was
            if mouse_buttons.just_pressed(MouseButton::Left) {
                let new_entity = commands
                    .spawn((
                        SceneBundle {
                            scene: assets.load("barriers/wooden_barrier.glb#Scene0"),
                            transform: Transform::from_translation(point),
                            ..default()
                        },
                        Post {
                            prev: Some(entity),
                            next: None,
                        },
                    ))
                    .id();

                // update newly placed post's next field
                post.next = Some(new_entity);

                // create connection between previous post position and placed post position
                if let Some(previous_entity) = post.prev {
                    let (_, from_transform, ..) = posts.get(previous_entity).unwrap();
                    let from = from_transform.translation;
                    let to = point;

                    let midpoint = from.lerp(to, 0.5);
                    let length = from.distance(to);
                    let angle = -f32::atan2(to.z - from.z, to.x - from.x);

                    commands.spawn(PbrBundle {
                        mesh: meshes.add(shape::Cube::new(1.0).into()),
                        material: materials.add(Color::BLUE.into()),
                        transform: Transform {
                            translation: midpoint,
                            rotation: Quat::from_rotation_y(angle),
                            scale: Vec3::new(length, 3.0, 0.1),
                        },
                        ..default()
                    });
                }
            }
        }

        // if the mouse is not over terrain, hide the preview post
        None => *visibility = Visibility::Hidden,
    }
}
