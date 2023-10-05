use bevy::prelude::*;
use std::marker::PhantomData;

/// A button that is able to control a single panel in a group of tabs
///
/// On this button being clicked, tabs with matching component will be shown
///
/// Mark a tab with the given `Component` type `T`
#[derive(Component)]
pub struct ShowTabButton<T: Component + PartialEq> {
    pub tab: T,
}

/// A button that is able to hide all displayed panels in a group of tabs
///
/// On this button being clicked, all tabs in the group will be hidden
#[derive(Component)]
pub struct HideTabButton<T: Component + PartialEq> {
    phantom: PhantomData<T>,
}

impl<T: Component + PartialEq> Default for HideTabButton<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
        }
    }
}

/// System to control interactions of tab buttons and correctly show/hide tabs
pub fn tab_group<T: Component + PartialEq>(
    hide_interactions: Query<(&Interaction, &HideTabButton<T>), Changed<Interaction>>,
    show_interactions: Query<(&Interaction, &ShowTabButton<T>), Changed<Interaction>>,
    mut panels: Query<(&mut Style, &T)>,
) {
    for (interaction, _) in hide_interactions.iter() {
        if *interaction == Interaction::Pressed {
            for (mut style, _) in panels.iter_mut() {
                style.display = Display::None;
            }
        }
    }

    for (interaction, show_button) in show_interactions.iter() {
        if *interaction == Interaction::Pressed {
            for (mut style, tab) in panels.iter_mut() {
                style.display = match show_button.tab == *tab {
                    true => Display::Flex,
                    false => Display::None,
                };
            }
        }
    }
}
