use crate::OBJECTS;
use bevy::{gltf::GltfMesh, prelude::*};
use bevy_rapier3d::prelude::*;

/// Handles changing the component's model whenever an entity's `Handle<GltfMesh>` component changes
///
/// To hide a component without despawning it, set handle to `Handle::<GltfMesh>::default()`
pub fn handle_mesh_changes(
    mut commands: Commands,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    // all entites that have had their mesh handle changed
    changes: Query<(Entity, &Handle<GltfMesh>), Changed<Handle<GltfMesh>>>,
) {
    for (entity, handle) in changes.iter() {
        commands
            .entity(entity)
            // clear all old mesh children of parent
            .despawn_descendants()
            // spawn in new mesh children
            .with_children(|parent| {
                // get gltf mesh or return early if not found
                let Some(gltf_mesh) = gltf_meshes.get(handle) else { return };

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
                        CollisionGroups::new(OBJECTS, Group::ALL),
                    ));
                }
            });
    }
}
