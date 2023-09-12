use crate::{ui::PropUiPlugin, Currency};
use bevy::{asset::Asset, prelude::*};
use std::marker::PhantomData;

mod interaction;
mod object;
mod utility;

pub use object::Object;

pub struct PropPlugin;
impl Plugin for PropPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<interaction::PlayerInteractionState>()
            .add_plugins(SetupPropPlugin::<Object>::default())
            .add_systems(
                Update,
                (
                    utility::handle_mesh_changes,
                    interaction::handle_player_interaction,
                    interaction::on_preview_unset,
                ),
            );
    }
}

pub struct SetupPropPlugin<P: Prop>(PhantomData<P>);

impl<P: Prop> Default for SetupPropPlugin<P> {
    fn default() -> Self {
        Self(PhantomData::default())
    }
}

impl<P: Prop> Plugin for SetupPropPlugin<P> {
    fn build(&self, app: &mut App) {
        app.add_asset::<P>()
            .init_resource::<P::Loader>()
            .add_event::<SetPreviewProp<P>>()
            .add_event::<UnsetPreviewProp>()
            .add_event::<PlaceProp>()
            .add_systems(Update, interaction::on_preview_set::<P>)
            // external plugins for other systems to configure with props
            .add_plugins(PropUiPlugin::<P>::default());

        P::systems(app);
    }
}

pub trait Prop: Asset {
    /// Resource that will store all loaded instances of this object
    type Loader: Resource + FromWorld;

    // common data required from all props
    fn name(&self) -> &'static str;
    fn cost(&self) -> Currency;
    fn icon(&self) -> &Handle<Image>;

    /// Add all required systems to the app
    fn systems(app: &mut App);
}

#[derive(Event)]
pub struct SetPreviewProp<P: Prop> {
    pub handle: Handle<P>,
}

#[derive(Event)]
pub struct UnsetPreviewProp;

#[derive(Event)]
pub struct PlaceProp;
