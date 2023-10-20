use bevy::{gltf::GltfMesh, pbr::NotShadowCaster, prelude::*};
use bevy_rapier3d::prelude::*;

/// Component that allows a glTF mesh to be rendered by supplying its handle and some settings
#[derive(Component)]
pub struct RenderGltf {
    pub handle: Handle<GltfMesh>,
    pub mode: RenderGltfMode,
}

/// Different modes to render the glTF mesh
#[derive(Default, PartialEq)]
pub enum RenderGltfMode {
    /// Default rendering with supplied material (if given)
    #[default]
    Regular,
    /// Render model as slightly transparent
    Preview,
}

impl RenderGltfMode {
    /// Alpha value for previewed models
    const PREVIEW_ALPHA: f32 = 0.6;
}

/// Handles changing the component's model whenever an entity's `RenderGltf` component changes
pub fn handle_mesh_changes(
    mut commands: Commands,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    changes: Query<(Entity, &RenderGltf), Changed<RenderGltf>>,
) {
    for (entity, gltf) in changes.iter() {
        commands
            .entity(entity)
            // clear all old mesh children of parent
            .despawn_descendants()
            // spawn in new mesh children
            .with_children(|parent| {
                // get gltf mesh or return early if not found
                let Some(gltf_mesh) = gltf_meshes.get(&gltf.handle) else { return };

                // spawn a mesh child for each gltf primitive
                for gltf_primitive in gltf_mesh.primitives.iter() {
                    // spawn primitive into the world
                    let mut model = parent.spawn(PbrBundle {
                        mesh: gltf_primitive.mesh.clone(),
                        material: gltf_primitive.material.clone().unwrap_or_default(),
                        ..default()
                    });

                    // apply additional options based on rendering mode
                    match gltf.mode {
                        // nothing to do for regular rendering mode
                        RenderGltfMode::Regular => (),

                        RenderGltfMode::Preview => {
                            // replace base material with slightly transparent preview material
                            let base_handle = gltf_primitive.material.clone().unwrap_or_default();
                            let base_material = materials.get(&base_handle).unwrap();

                            // reduce base material's alpha
                            let transparent_color = base_material
                                .base_color
                                .with_a(RenderGltfMode::PREVIEW_ALPHA);

                            // and insert into model to overwrite base material
                            model.insert(materials.add(transparent_color.into()));

                            // transparent preview mode should not cast shadows
                            model.insert(NotShadowCaster);
                        }
                    }
                }
            });
    }
}

#[derive(Component)]
pub struct ColliderMesh {
    pub mesh: Handle<Mesh>,
    pub rb: RigidBody,
    pub membership: Group,
}

pub fn handle_collider_changes(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,

    changes: Query<(Entity, &ColliderMesh), Changed<ColliderMesh>>,
) {
    for (entity, collider_component) in changes.iter() {
        let Some(collider_mesh) = meshes.get(&collider_component.mesh) else { continue };
        let Some(collider) = Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::ConvexHull) else { continue };

        commands.entity(entity).insert((
            collider_component.rb,
            collider,
            CollisionGroups::new(collider_component.membership, Group::ALL),
        ));
    }
}
