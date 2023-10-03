use crate::{ui::UiDisplay, Currency};
use bevy::{
    gltf::GltfMesh,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

pub struct BarrierPlugin;
impl Plugin for BarrierPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<BarrierData>()
            .init_resource::<BarrierLoader>();
    }
}

#[derive(TypeUuid, TypePath)]
#[uuid = "5acc571e-6c1b-4991-b610-dba324bcacd1"]
pub struct BarrierData {
    pub name: String,
    pub icon: Handle<Image>,

    pub post_cost: Currency,
    pub post_mesh: Handle<GltfMesh>,

    /// Per-meter cost of fence
    pub fence_cost: Currency,
    pub fence_mesh: Handle<GltfMesh>,
}

impl UiDisplay for BarrierData {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn image(&self) -> UiImage {
        self.icon.clone().into()
    }

    fn text(&self) -> String {
        format!("${:.0} (${:.0}/m)", self.post_cost, self.fence_cost)
    }
}

#[derive(Resource)]
struct BarrierLoader {
    _barriers: Vec<Handle<BarrierData>>,
}

impl FromWorld for BarrierLoader {
    fn from_world(world: &mut World) -> Self {
        // TODO - read from asset file in json or ron format
        let asset_server = world.resource::<AssetServer>();
        let data = [BarrierData {
            name: "Concrete Barrier".into(),
            icon: asset_server.load("test.png"),
            post_cost: 50.0,
            post_mesh: asset_server.load("barriers/concrete_post.glb#Mesh0"),
            fence_cost: 10.0,
            fence_mesh: asset_server.load("barriers/concrete_fence.glb#Mesh0"),
        }];

        let mut barrier_assets = world.resource_mut::<Assets<BarrierData>>();
        let barriers = data.map(|barrier| barrier_assets.add(barrier));

        Self {
            _barriers: barriers.into(),
        }
    }
}