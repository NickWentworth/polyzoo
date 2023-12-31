use crate::{ui::UiDisplay, Currency, CurrencyFormat};
use bevy::{
    gltf::GltfMesh,
    prelude::*,
    reflect::{TypePath, TypeUuid},
};

pub struct PropDataPlugin;
impl Plugin for PropDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<PropData>().init_resource::<PropLoader>();
    }
}

#[derive(TypeUuid, TypePath)]
#[uuid = "6a4cb839-5211-4e4c-a0cf-88df4bf67865"]
pub struct PropData {
    pub name: String,
    pub icon: Handle<Image>,

    pub cost: Currency,
    /// The model that will be rendered for this prop
    pub model: Handle<GltfMesh>,
    /// The single collider mesh for this prop
    pub collider: Handle<Mesh>,
}

impl UiDisplay for PropData {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn image(&self) -> UiImage {
        self.icon.clone().into()
    }

    fn text(&self) -> String {
        self.cost.comma_separated()
    }
}

#[derive(Resource)]
struct PropLoader {
    _props: Vec<Handle<PropData>>,
}

impl FromWorld for PropLoader {
    fn from_world(world: &mut World) -> Self {
        // TODO: read from asset file in json or ron format
        let asset_server = world.resource::<AssetServer>();
        let data = [
            PropData {
                name: "Dark Rock 1".into(),
                icon: asset_server.load("test.png"),
                cost: 20.0,
                model: asset_server.load("nature/rocks.glb#Mesh0"),
                collider: asset_server.load("nature/rocks.glb#Mesh0/Primitive0"),
            },
            PropData {
                name: "Dark Rock 2".into(),
                icon: asset_server.load("test.png"),
                cost: 25.0,
                model: asset_server.load("nature/rocks.glb#Mesh1"),
                collider: asset_server.load("nature/rocks.glb#Mesh1/Primitive0"),
            },
            PropData {
                name: "Dark Rock 3".into(),
                icon: asset_server.load("test.png"),
                cost: 30.0,
                model: asset_server.load("nature/rocks.glb#Mesh2"),
                collider: asset_server.load("nature/rocks.glb#Mesh2/Primitive0"),
            },
            PropData {
                name: "Light Rock 1".into(),
                icon: asset_server.load("test.png"),
                cost: 20.0,
                model: asset_server.load("nature/rocks.glb#Mesh3"),
                collider: asset_server.load("nature/rocks.glb#Mesh3/Primitive0"),
            },
            PropData {
                name: "Light Rock 2".into(),
                icon: asset_server.load("test.png"),
                cost: 25.0,
                model: asset_server.load("nature/rocks.glb#Mesh4"),
                collider: asset_server.load("nature/rocks.glb#Mesh4/Primitive0"),
            },
            PropData {
                name: "Light Rock 3".into(),
                icon: asset_server.load("test.png"),
                cost: 30.0,
                model: asset_server.load("nature/rocks.glb#Mesh5"),
                collider: asset_server.load("nature/rocks.glb#Mesh5/Primitive0"),
            },
            PropData {
                name: "Sandy Rock 1".into(),
                icon: asset_server.load("test.png"),
                cost: 20.0,
                model: asset_server.load("nature/rocks.glb#Mesh6"),
                collider: asset_server.load("nature/rocks.glb#Mesh6/Primitive0"),
            },
            PropData {
                name: "Sandy Rock 2".into(),
                icon: asset_server.load("test.png"),
                cost: 25.0,
                model: asset_server.load("nature/rocks.glb#Mesh7"),
                collider: asset_server.load("nature/rocks.glb#Mesh7/Primitive0"),
            },
            PropData {
                name: "Sandy Rock 3".into(),
                icon: asset_server.load("test.png"),
                cost: 30.0,
                model: asset_server.load("nature/rocks.glb#Mesh8"),
                collider: asset_server.load("nature/rocks.glb#Mesh8/Primitive0"),
            },
        ];

        let mut prop_assets = world.resource_mut::<Assets<PropData>>();
        let props = data.map(|prop| prop_assets.add(prop));

        Self {
            _props: props.into(),
        }
    }
}
