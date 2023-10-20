use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use placement::utility::ColliderMesh;

mod camera;
mod objects;
mod placement;
mod ui;
mod zoo;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
        ))
        .add_plugins((
            camera::ControllableCameraPlugin,
            objects::ObjectPlugin,
            placement::PlacementPlugin,
            ui::UiPlugin,
            zoo::ZooPlugin,
        ))
        .add_systems(Startup, setup_demo_scene)
        .run();
}

/// Common currency type used throughout the game
type Currency = f32;

trait CurrencyFormat {
    /// Format currency with commas separating every 3 digits, such as $12,345,678
    fn comma_separated(self) -> String;
}

impl CurrencyFormat for Currency {
    fn comma_separated(self) -> String {
        let string_repr = format!("{:.0}", self);

        // add commas between every 3 digits
        let with_commas = string_repr
            .chars()
            // break into chunks of 3, starting from end of iterator
            .collect::<Vec<_>>()
            .rchunks(3)
            // then reverse again to the proper order
            .rev()
            .collect::<Vec<_>>()
            // join char slices with a comma
            .join(&',')
            // and collect back into a String
            .iter()
            .collect::<String>();

        // add dollar sign
        format!("${}", with_commas)
    }
}

// collision groups
const GROUND: Group = Group::GROUP_1;
const OBJECTS: Group = Group::GROUP_2;

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
    let ground_mesh = meshes.add(Mesh::from(shape::Plane::from_size(10.0)));
    commands.spawn((
        PbrBundle {
            mesh: ground_mesh.clone(),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        ColliderMesh {
            mesh: ground_mesh,
            rb: RigidBody::Fixed,
            membership: GROUND,
        },
    ));
}
