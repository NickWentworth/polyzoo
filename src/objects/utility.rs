use bevy::{ecs::system::SystemParam, gltf::GltfMesh, prelude::*};
use bevy_rapier3d::prelude::*;

use super::Object;

/// Helper system param for spawning, despawning, and managing object entities
#[derive(SystemParam)]
pub struct ObjectUtility<'w, 's> {
    commands: Commands<'w, 's>,
    objects: Res<'w, Assets<Object>>,
    gltf_meshes: Res<'w, Assets<GltfMesh>>,
    meshes: Res<'w, Assets<Mesh>>,
}

impl<'w, 's> ObjectUtility<'w, 's> {
    /// Spawns in a new object entity and inserts given bundle of components
    pub fn spawn_object_with(&mut self, handle: &Handle<Object>, bundle: impl Bundle) -> Entity {
        let object = self.spawn_object(handle);
        self.commands.entity(object).insert(bundle).id()
    }

    /// Spawns in a new object entity with default transform and visibility
    pub fn spawn_object(&mut self, handle: &Handle<Object>) -> Entity {
        let parent = self.commands.spawn(SpatialBundle::default()).id();
        self.set_object(parent, handle)
    }

    /// Updates the entity's object-related children without changing its current components
    pub fn set_object(&mut self, mut parent: Entity, handle: &Handle<Object>) -> Entity {
        // clear children first
        parent = self.clear_object(parent);

        // get gltf mesh of the object
        let Some(object) = self.objects.get(handle) else { return parent };
        let Some(gltf_mesh) = self.gltf_meshes.get(&object.mesh) else { return parent };

        // spawn in new children
        parent = self
            .commands
            .entity(parent)
            .with_children(|parent| {
                for gltf_primitive in gltf_mesh.primitives.iter() {
                    parent.spawn((
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

        // update object handle of parent
        parent = self.commands.entity(parent).insert(handle.clone()).id();

        parent
    }

    /// Despawns all object-related children of the parent entity
    pub fn clear_object(&mut self, parent: Entity) -> Entity {
        self.commands.entity(parent).despawn_descendants().id()
    }

    /// Despawns the entire object and all children
    pub fn despawn_object(&mut self, object: Entity) {
        self.commands.entity(object).despawn_recursive();
    }
}
