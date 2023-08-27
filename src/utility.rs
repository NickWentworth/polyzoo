use bevy::{ecs::system::SystemParam, gltf::GltfMesh, prelude::*};
use bevy_rapier3d::prelude::*;

/// System param that assists with spawning in and managing meshes as entities in-game
#[derive(SystemParam)]
pub struct MeshUtility<'w, 's> {
    commands: Commands<'w, 's>,
    gltf_meshes: Res<'w, Assets<GltfMesh>>,
    meshes: Res<'w, Assets<Mesh>>,
}

impl<'w, 's> MeshUtility<'w, 's> {
    /// Spawns the necessary parent components with children displaying the given mesh
    pub fn spawn_mesh(&mut self, handle: &Handle<GltfMesh>) -> Entity {
        let parent = self.commands.spawn(SpatialBundle::default()).id();
        self.set_mesh(handle, parent)
    }

    /// Spawns the mesh and inserts the given bundle of components into the entity
    pub fn spawn_mesh_with(&mut self, handle: &Handle<GltfMesh>, bundle: impl Bundle) -> Entity {
        let parent = self.spawn_mesh(handle);
        self.commands.entity(parent).insert(bundle).id()
    }

    /// Sets the children of the given entity to compatible bundles to display the given mesh
    pub fn set_mesh(&mut self, handle: &Handle<GltfMesh>, mut parent: Entity) -> Entity {
        // clear object children first
        parent = self.clear_mesh(parent);

        // then set the children to the new mesh
        let Some(gltf_mesh) = self.gltf_meshes.get(handle) else { return parent; };

        parent = self
            .commands
            .entity(parent)
            .with_children(|p| {
                for gltf_primitive in gltf_mesh.primitives.iter() {
                    p.spawn((
                        PbrBundle {
                            mesh: gltf_primitive.mesh.clone(),
                            material: gltf_primitive.material.clone().unwrap_or(Handle::default()),
                            ..default()
                        },
                        Collider::from_bevy_mesh(
                            &self.meshes.get(&gltf_primitive.mesh).unwrap(),
                            &ComputedColliderShape::TriMesh,
                        )
                        .unwrap(),
                    ));
                }
            })
            .id();

        parent
    }

    // TODO - mark mesh children so that only the mesh-related children are despawned, not all children
    /// Clears all mesh children of the parent
    pub fn clear_mesh(&mut self, parent: Entity) -> Entity {
        self.commands.entity(parent).despawn_descendants().id()
    }
}
