use bevy::prelude::*;

#[derive(Component)]
pub struct DarkButton;

#[derive(Component)]
pub struct LightButton;

/// Group of colors that are used for styling the ui
#[derive(Resource)]
pub struct UiTheme {
    pub white: Color,
    pub light: Color,
    pub medium: Color,
    pub dark: Color,
    pub accent: Color,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            white: Color::hex("#E8EDED").unwrap(),
            light: Color::hex("#97ACAF").unwrap(),
            medium: Color::hex("#4F6367").unwrap(),
            dark: Color::hex("#354345").unwrap(),
            accent: Color::hex("#F26430").unwrap(),
        }
    }
}

impl UiTheme {
    /// Bundled nodes for dark button component
    pub fn dark_button(&self) -> impl Bundle {
        (
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: self.dark.into(),
                ..default()
            },
            DarkButton,
        )
    }

    /// Bundled nodes for light button component
    pub fn light_button(&self) -> impl Bundle {
        (
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: self.light.into(),
                ..default()
            },
            LightButton,
        )
    }

    /// Dark text component
    pub fn dark_text(&self, text: &str, font_size: f32) -> TextBundle {
        TextBundle::from_section(
            text,
            TextStyle {
                font_size,
                color: self.dark,
                ..default()
            },
        )
    }

    /// White text component
    pub fn white_text(&self, text: &str, font_size: f32) -> TextBundle {
        TextBundle::from_section(
            text,
            TextStyle {
                font_size,
                color: self.white,
                ..default()
            },
        )
    }
}

/// All systems needed for component interactivity
pub fn handle_interactions(
    mut interactions: ParamSet<(
        // dark buttons
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<DarkButton>)>,
        // light buttons
        Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<LightButton>)>,
    )>,
    theme: Res<UiTheme>,
) {
    for (interaction, mut background_color) in interactions.p0().iter_mut() {
        *background_color = match interaction {
            Interaction::None => theme.dark,
            Interaction::Hovered => theme.dark * 0.9,
            Interaction::Pressed => theme.dark * 0.7,
        }
        .into();
    }

    for (interaction, mut background_color) in interactions.p1().iter_mut() {
        *background_color = match interaction {
            Interaction::None => theme.light,
            Interaction::Hovered => theme.light * 0.9,
            Interaction::Pressed => theme.light * 0.7,
        }
        .into();
    }
}
