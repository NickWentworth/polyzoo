use bevy::{
    ecs::system::SystemParam,
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier3d::prelude::*;

pub struct ControllableCameraPlugin;
impl Plugin for ControllableCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, handle_camera);
    }
}

#[derive(Component)]
struct CameraController {
    pivot: Vec3,
}

// constants for camera controlling
const ORBIT: MouseButton = MouseButton::Middle;
const ORBIT_SPEED: f32 = 0.5;

const PAN: MouseButton = MouseButton::Right;
const PAN_SPEED: f32 = 1.0;

const ZOOM_SPEED: f32 = 0.5;
const MIN_ZOOM_DISTANCE: f32 = 1.0;

fn setup_camera(mut commands: Commands) {
    // spawn in camera looking at its pivot point
    let pivot = Vec3::ZERO;

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(pivot, Vec3::Y),
            ..default()
        },
        CameraController { pivot },
    ));
}

fn handle_camera(
    mut camera_query: Query<(&mut CameraController, &mut Transform)>,

    mouse_buttons: Res<Input<MouseButton>>,
    mut mouse_movement: EventReader<MouseMotion>,
    mut mouse_scroll: EventReader<MouseWheel>,

    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    let (mut controller, mut transform) = camera_query.single_mut();

    // TODO - add some smoothing to the movement
    // TODO - prevent camera from orbiting too far and going upside down or inside the floor

    // calculate and apply orbiting
    if mouse_buttons.pressed(ORBIT) {
        // get mouse movement and scale it accordingly
        let mut delta = Vec2::ZERO;
        for movement in mouse_movement.iter() {
            delta += movement.delta;
        }
        delta *= -1.0 * ORBIT_SPEED * time.delta_seconds();

        // horizontal rotation is done around the global y-axis
        transform.rotate_around(controller.pivot, Quat::from_rotation_y(delta.x));

        // vertical rotation is done around the local x-axis
        let axis = transform.right();
        transform.rotate_around(controller.pivot, Quat::from_axis_angle(axis, delta.y));
    }
    // calculate and apply panning (if not orbiting)
    else if mouse_buttons.pressed(PAN) {
        // get mouse movement and convert it to pan values based on camera's looking direction
        let mut pan = Vec3::ZERO;
        for movement in mouse_movement.iter() {
            // horizontal mouse movement is in line with camera's local x-axis
            pan += transform.left() * movement.delta.x;
            // vertical mouse movement is in line with camera's local z-axis projected onto XZ-plane
            pan += transform.left().cross(Vec3::Y) * movement.delta.y;
        }
        pan *= PAN_SPEED * time.delta_seconds();

        // apply panning amount to both camera's transform and controller's pivot
        transform.translation += pan;
        controller.pivot += pan;
    }
    // calculate and apply zooming (if not orbiting or panning)
    else if !mouse_scroll.is_empty() {
        // get scroll amount
        let mut scroll = 0.0;
        for wheel in mouse_scroll.iter() {
            scroll += wheel.y;
        }

        // zoom in by scroll amount, scaled to ensure we don't zoom past the pivot point
        let zoom = transform.forward() * scroll * ZOOM_SPEED;
        let scale =
            ((transform.translation + zoom).distance(controller.pivot) / MIN_ZOOM_DISTANCE).log2();

        transform.translation += zoom * scale;
    }

    // TEMP - to show the location of the controller's pivot point
    gizmos.sphere(controller.pivot, Quat::IDENTITY, 0.1, Color::WHITE);
}

/// System parameter to get the location of the cursor projected into the world
#[derive(SystemParam)]
pub struct CursorRaycast<'w, 's> {
    main_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<CameraController>>,
    main_window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    rapier: Res<'w, RapierContext>,
}

impl<'w, 's> CursorRaycast<'w, 's> {
    /// Cast a ray from the camera at the cursor location into the world
    ///
    /// Returns the first entity hit and the location of the hit, if a hit occurs
    pub fn raycast(&self) -> Option<(Entity, Vec3)> {
        let (camera, camera_transform) = self.main_camera.single();
        let window = self.main_window.single();

        // get ray from camera's perspective to cursor point
        let ray = camera.viewport_to_world(camera_transform, window.cursor_position()?)?;

        // cast the ray
        let (entity, distance) = self.rapier.cast_ray(
            ray.origin,
            ray.direction,
            f32::MAX,
            false,
            QueryFilter::default(),
        )?;

        // if everything worked, return the entity and distance to the collision point
        Some((entity, ray.get_point(distance)))
    }

    /// Do raycast, but only return location of the hit, if it occurs
    pub fn point(&self) -> Option<Vec3> {
        match self.raycast() {
            Some((_, point)) => Some(point),
            None => None,
        }
    }
}
