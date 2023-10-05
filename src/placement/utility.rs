use crate::OBJECTS;
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
    bevy_meshes: Res<Assets<Mesh>>,
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
                    // get material handle based on rendering mode
                    let material = match gltf.mode {
                        // regular rendering mode should just insert material handle
                        RenderGltfMode::Regular => {
                            gltf_primitive.material.clone().unwrap_or_default()
                        }

                        // preview rendering mode should insert material handle with adjusted alpha for transparency
                        RenderGltfMode::Preview => {
                            let base_handle = gltf_primitive.material.clone().unwrap_or_default();
                            let base_material = materials.get(&base_handle).unwrap();

                            // reduce base material's alpha
                            let transparent_color = base_material
                                .base_color
                                .with_a(RenderGltfMode::PREVIEW_ALPHA);

                            materials.add(transparent_color.into())
                        }
                    };

                    // spawn primitive into the world
                    let mut model = parent.spawn((
                        PbrBundle {
                            mesh: gltf_primitive.mesh.clone(),
                            material,
                            ..default()
                        },
                        Collider::from_bevy_mesh(
                            &bevy_meshes.get(&gltf_primitive.mesh).unwrap(),
                            &ComputedColliderShape::TriMesh,
                        )
                        .unwrap(),
                        CollisionGroups::new(OBJECTS, Group::ALL),
                    ));

                    // apply additional options based on rendering mode
                    match gltf.mode {
                        // nothing to do for regular rendering mode
                        RenderGltfMode::Regular => (),

                        // transparent preview mode should not cast shadows
                        RenderGltfMode::Preview => {
                            model.insert(NotShadowCaster);
                        }
                    }
                }
            });
    }
}
