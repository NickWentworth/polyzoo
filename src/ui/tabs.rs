use bevy::prelude::*;

/// A button that is able to control a single panel in a group of tabs
///
/// On this button storing `Some(Component)` being clicked, tabs with matching component will be shown
///
/// If the button instead stores `None`, all tabs will hide on click
#[derive(Component)]
pub struct TabButton<T: Component + PartialEq>(pub Option<T>);

/// System to control interactions of tab buttons and correctly show/hide tabs
pub fn tab_group<T: Component + PartialEq>(
    interactions: Query<(&Interaction, &TabButton<T>), Changed<Interaction>>,
    mut panels: Query<(&mut Style, &T)>,
) {
    for (interaction, button) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            // turn off all tabs besides the clicked button's tab
            for (mut style, tab) in panels.iter_mut() {
                if let Some(button_tab) = &button.0 {
                    if tab == button_tab {
                        style.display = Display::Flex;
                        continue;
                    }
                }

                // if not the matching tab or button's value is None, hide tabs
                style.display = Display::None;
            }
        }
    }
}
