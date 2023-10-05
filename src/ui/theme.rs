use bevy::prelude::*;

pub struct UiThemePlugin;
impl Plugin for UiThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiTheme>()
            .add_systems(Update, handle_button_color_change);
    }
}

/// Component allowing a button to override or fall back to its default color
#[derive(Component)]
pub struct ButtonColor {
    default_color: Color,
    override_color: Option<Color>,
}

impl ButtonColor {
    /// Create a new button color component with given default color
    fn new(default_color: Color) -> Self {
        Self {
            default_color,
            override_color: None,
        }
    }

    /// Update the color of the button to the given color
    pub fn update(&mut self, color: Color) {
        self.override_color = Some(color);
    }

    /// Revert the button's color back to its default
    pub fn revert(&mut self) {
        self.override_color = None;
    }
}

/// Group of colors that are used for styling the ui
#[derive(Resource)]
pub struct UiTheme {
    pub white: Color,
    pub light: Color,
    pub medium: Color,
    pub dark: Color,
    pub accent: Color,

    pub font_regular: Handle<Font>,
    pub font_bold: Handle<Font>,
}

impl FromWorld for UiTheme {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            white: Color::hex("#E8EDED").unwrap(),
            light: Color::hex("#97ACAF").unwrap(),
            medium: Color::hex("#4F6367").unwrap(),
            dark: Color::hex("#354345").unwrap(),
            accent: Color::hex("#F26430").unwrap(),

            font_regular: asset_server.load("fonts/urbanist_regular.ttf"),
            font_bold: asset_server.load("fonts/urbanist_bold.ttf"),
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
                ..default()
            },
            ButtonColor::new(self.dark),
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
                ..default()
            },
            ButtonColor::new(self.light),
        )
    }

    /// Dark text component
    pub fn dark_text(&self, text: &str, font_size: f32) -> TextBundle {
        TextBundle::from_section(
            text,
            TextStyle {
                font: self.font_bold.clone(),
                font_size,
                color: self.dark,
            },
        )
    }

    /// White text component
    pub fn white_text(&self, text: &str, font_size: f32) -> TextBundle {
        TextBundle::from_section(
            text,
            TextStyle {
                font: self.font_bold.clone(),
                font_size,
                color: self.white,
            },
        )
    }

    /// Spawn in a light button with icon
    pub fn spawn_light_icon_button(
        &self,
        parent: &mut ChildBuilder,
        size: Val,
        icon: UiImage,
        bundle: impl Bundle,
    ) {
        parent
            .spawn((self.light_button(), bundle))
            .insert(Style {
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: icon,
                    style: Style {
                        width: size,
                        height: size,
                        ..default()
                    },
                    background_color: self.dark.into(),
                    ..default()
                });
            });
    }
}

fn handle_button_color_change(
    mut buttons: Query<
        (&mut BackgroundColor, &Interaction, &ButtonColor),
        Or<(Changed<Interaction>, Changed<ButtonColor>)>,
    >,
) {
    for (mut background_color, interaction, button_color) in buttons.iter_mut() {
        // if an override color exists, use that
        let base_color = button_color
            .override_color
            .unwrap_or(button_color.default_color);

        *background_color = match interaction {
            // base color regularly
            Interaction::None => base_color,

            // slightly darker on hover
            Interaction::Hovered => base_color * 0.9,

            // even darker on press
            Interaction::Pressed => base_color * 0.7,
        }
        .into();
    }
}
