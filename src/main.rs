use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod objects;
mod ui;
mod utility;
mod zoo;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((
            camera::ControllableCameraPlugin,
            objects::ObjectsPlugin,
            ui::UiPlugin,
            zoo::ZooPlugin,
        ))
        .add_systems(Startup, setup_demo_scene)
        .run();
}

/// Marker component for ground plane
#[derive(Component)]
struct Ground;

fn setup_demo_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn in lights
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ground plane
    let ground_plane = Mesh::from(shape::Plane::from_size(10.0));
    commands.spawn((
        Collider::from_bevy_mesh(&ground_plane, &ComputedColliderShape::ConvexHull).unwrap(),
        PbrBundle {
            mesh: meshes.add(ground_plane),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Ground,
    ));
}
