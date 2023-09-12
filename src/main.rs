use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod camera;
mod props;
mod ui;
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
            props::PropPlugin,
            ui::UiPlugin,
            zoo::ZooPlugin,
        ))
        .add_systems(Startup, setup_demo_scene)
        .run();
}

/// Common currency type used throughout the game
type Currency = f32;

// collision groups
const GROUND: Group = Group::GROUP_1;
const PROPS: Group = Group::GROUP_2;

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
        PbrBundle {
            mesh: meshes.add(ground_plane.clone()),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Collider::from_bevy_mesh(&ground_plane, &ComputedColliderShape::ConvexHull).unwrap(),
        CollisionGroups::new(GROUND, Group::ALL),
    ));
}
