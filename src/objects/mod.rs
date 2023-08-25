use bevy::{
    ecs::system::SystemParam,
    gltf::GltfMesh,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

mod barrier;
mod nature;

pub struct ObjectsPlugin;
impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Object>().init_resource::<ObjectLoader>();
    }
}

/// Resource containing all pre-loaded object data in untyped handles, so that it isn't unloaded
///
/// For now, all objects are also stored in the ui, so data stems from there
#[derive(Resource)]
struct ObjectLoader {
    _data: Vec<HandleUntyped>,
}

impl FromWorld for ObjectLoader {
    fn from_world(world: &mut World) -> Self {
        let mut data = Vec::new();

        data.extend(barrier::add(world));
        data.extend(nature::add(world));

        Self { _data: data }
    }
}

/// Base asset for a placeable object
#[derive(TypeUuid, TypePath)]
#[uuid = "5981b7ce-340e-4e24-a004-7139c8860455"]
pub struct Object {
    pub name: &'static str,
    pub cost: Currency,
    pub mesh: Handle<GltfMesh>,
    pub image: Handle<Image>,
    pub group: ObjectGroup,
}

/// Type alias for money variables
pub type Currency = f32;

/// Different classifications for placeable objects
#[derive(PartialEq)]
pub enum ObjectGroup {
    Barrier(Handle<GltfMesh>), // stores the fence model connecting posts
    Rock,
}

/// Helper system param that assists with spawning in and managing objects as entities in-game
#[derive(SystemParam)]
pub struct ObjectUtility<'w, 's> {
    commands: Commands<'w, 's>,
    objects: Res<'w, Assets<Object>>,
    gltf_meshes: Res<'w, Assets<GltfMesh>>,
}

impl<'w, 's> ObjectUtility<'w, 's> {
    /// Spawns the necessary parent components with children displaying the object's model
    pub fn spawn_object(&mut self, handle: &Handle<Object>) -> Entity {
        let parent = self.commands.spawn(SpatialBundle::default()).id();
        self.set_object(handle, parent)
    }

    /// Sets the children of the given entity to compatible bundles to display the object's model
    pub fn set_object(&mut self, handle: &Handle<Object>, parent: Entity) -> Entity {
        let Some(object) = self.objects.get(handle) else { return parent; };
        self.set_mesh(&object.mesh.clone(), parent)
    }

    /// Spawns the necessary parent components with children displaying the given mesh
    pub fn spawn_mesh(&mut self, handle: &Handle<GltfMesh>) -> Entity {
        let parent = self.commands.spawn(SpatialBundle::default()).id();
        self.set_mesh(handle, parent)
    }

    /// Sets the children of the given entity to compatible bundles to display the given mesh
    pub fn set_mesh(&mut self, handle: &Handle<GltfMesh>, mut parent: Entity) -> Entity {
        // clear object children first
        parent = self.clear_object(parent);

        // then set the children to the new mesh
        let Some(gltf_mesh) = self.gltf_meshes.get(handle) else { return parent; };

        parent = self
            .commands
            .entity(parent)
            .with_children(|p| {
                for gltf_primitive in gltf_mesh.primitives.iter() {
                    p.spawn(PbrBundle {
                        mesh: gltf_primitive.mesh.clone(),
                        material: gltf_primitive.material.clone().unwrap_or(Handle::default()),
                        ..default()
                    });
                }
            })
            .id();

        parent
    }

    // TODO - mark mesh children so that only the mesh-related children are deleted, not all children
    /// Clears all mesh children of the parent
    pub fn clear_object(&mut self, parent: Entity) -> Entity {
        self.commands.entity(parent).despawn_descendants().id()
    }
}
