use super::PropData;
use bevy::prelude::*;

/// Instance of a prop in the world
#[derive(Component)]
pub struct Prop {
    pub data: Handle<PropData>,
}
