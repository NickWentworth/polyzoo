use super::Object;
use bevy::{gltf::GltfMesh, prelude::*};
use bevy_rapier3d::prelude::*;

/// Handles changing the object model whenever an entity's `Handle<Object>` component changes
///
/// To hide an object without despawning it, set handle to `Handle::<Object>::default()`
pub fn handle_object_changes(
    mut commands: Commands,
    objects: Res<Assets<Object>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    // all entites that have had their object handle changed
    changes: Query<(Entity, &Handle<Object>), Changed<Handle<Object>>>,
) {
    for (entity, handle) in changes.iter() {
        commands
            .entity(entity)
            // clear all old mesh children of parent
            .despawn_descendants()
            // spawn in new mesh children
            .with_children(|parent| {
                // get gltf mesh of the object or return early if not found
                let Some(object) = objects.get(handle) else { return };
                let Some(gltf_mesh) = gltf_meshes.get(&object.mesh) else { return };

                // spawn a mesh child for each gltf primitive
                for gltf_primitive in gltf_mesh.primitives.iter() {
                    parent.spawn((
                        PbrBundle {
                            mesh: gltf_primitive.mesh.clone(),
                            material: gltf_primitive.material.clone().unwrap_or(Handle::default()),
                            ..default()
                        },
                        Collider::from_bevy_mesh(
                            &meshes.get(&gltf_primitive.mesh).unwrap(),
                            &ComputedColliderShape::TriMesh,
                        )
                        .unwrap(),
                    ));
                }
            });
    }
}
