use super::{PlaceProp, Prop, SetPreviewProp, UnsetPreviewProp};
use crate::camera::CursorRaycast;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub enum PlayerInteractionState {
    #[default]
    None,
    Placing,
}

// FIXME - there may be a race condition if on_preview_unset is handled after on_preview_set

pub fn on_preview_set<P: Prop>(
    mut state: ResMut<PlayerInteractionState>,
    mut preview_sets: EventReader<SetPreviewProp<P>>,
) {
    for _ in preview_sets.iter() {
        *state = PlayerInteractionState::Placing;
    }
}

pub fn on_preview_unset(
    mut state: ResMut<PlayerInteractionState>,
    mut preview_unsets: EventReader<UnsetPreviewProp>,
) {
    for _ in preview_unsets.iter() {
        *state = PlayerInteractionState::None;
    }
}

pub fn handle_player_interaction(
    state: ResMut<PlayerInteractionState>,
    mouse_button: Res<Input<MouseButton>>,
    cursor: CursorRaycast,

    mut place_props: EventWriter<PlaceProp>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        match *state {
            // TODO - prop selection
            PlayerInteractionState::None => (),
            PlayerInteractionState::Placing => {
                if cursor.ground_point().is_some() {
                    place_props.send(PlaceProp)
                }
            }
        }
    }
}
