use bevy::prelude::*;
use common::text::TEXT_COLOR;

use crate::home::NavigationButtonAction;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct ButtonConfiguration {
    pub style: Style,
    pub icon_style: Style,
    pub text_style: TextStyle,
}

#[derive(Bundle)]
pub struct NavigationButtonBundle {
    pub button: ButtonBundle,
    pub properties: ButtonProperties,
    pub action: NavigationButtonAction,
}

#[derive(Component)]
pub struct ButtonProperties {
    pub(crate) is_active: bool,
    pub(crate) is_hovered: bool,
    pub(crate) is_clicked: bool,
}

impl Default for ButtonProperties {
    fn default() -> Self {
        ButtonProperties {
            is_active: false,
            is_hovered: false,
            is_clicked: false,
        }
    }
}

//TODO: Think how to do that differently
impl ButtonConfiguration {
    pub fn instance() -> Self {
        ButtonConfiguration {
            style: Style {
                width: Val::Px(54.0),
                height: Val::Px(54.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            icon_style: Style {
                width: Val::Px(24.0),
                // This takes the icons out of the flexbox flow, to be positioned exactly
                position_type: PositionType::Absolute,
                // The icon will be close to the left border of the button
                left: Val::Px(10.0),
                ..default()
            },
            text_style: TextStyle {
                font_size: 40.0,
                color: TEXT_COLOR,
                ..default()
            },
        }
    }
}

// This system handles changing all buttons color based on mouse interaction
// Query here is a minimal query that should select all buttons
pub fn button_interaction_style_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut border_color) in &mut interaction_query {
        border_color.0 = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_PRESSED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
    }
}
