use super::{PlaceProp, Prop, SetPreviewProp, UnsetPreviewProp};
use crate::{camera::CursorRaycast, Currency};
use bevy::{
    gltf::GltfMesh,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

mod data;

#[derive(TypeUuid, TypePath)]
#[uuid = "6a4cb839-5211-4e4c-a0cf-88df4bf67865"]
pub struct Object {
    pub name: &'static str,
    pub cost: Currency,
    pub icon: Handle<Image>,
    pub mesh: Handle<GltfMesh>,
}

impl Prop for Object {
    type Loader = data::ObjectLoader;

    fn name(&self) -> &'static str {
        self.name
    }
    fn cost(&self) -> Currency {
        self.cost
    }
    fn icon(&self) -> &Handle<Image> {
        &self.icon
    }

    fn systems(app: &mut App) {
        app.add_systems(Update, (handle_movement, on_preview_change, on_place));
    }
}

#[derive(Component)]
struct Preview;

fn handle_movement(
    cursor: CursorRaycast,
    mut previews: Query<(&mut Transform, &mut Visibility), With<Preview>>,
) {
    for (mut transform, mut visibility) in previews.iter_mut() {
        match cursor.ground_point() {
            Some(position) => {
                transform.translation = position;
                *visibility = Visibility::Visible;
            }

            None => *visibility = Visibility::Hidden,
        }
    }
}

fn on_preview_change(
    mut commands: Commands,
    objects: Res<Assets<Object>>,

    mut preview_sets: EventReader<SetPreviewProp<Object>>,
    mut preview_unsets: EventReader<UnsetPreviewProp>,
    previews: Query<Entity, With<Preview>>,
) {
    for _ in preview_unsets.iter() {
        for entity in previews.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    for set in preview_sets.iter() {
        if let Some(object) = objects.get(&set.handle) {
            commands.spawn((SpatialBundle::default(), object.mesh.clone(), Preview));
        }
    }
}

fn on_place(
    mut commands: Commands,

    mut places: EventReader<PlaceProp>,
    previews: Query<(&Transform, &Handle<GltfMesh>), With<Preview>>,
) {
    for _ in places.iter() {
        for (transform, mesh) in previews.iter() {
            commands.spawn((
                SpatialBundle {
                    transform: *transform,
                    ..default()
                },
                mesh.clone(),
            ));
        }
    }
}
