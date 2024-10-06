use bevy::prelude::*;

use crate::text::TEXT_COLOR;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct ButtonConfiguration {
    pub style: Style,
    pub icon_style: Style,
    pub text_style: TextStyle,
}

#[derive(Component)]
pub struct ButtonProperties {
    pub(crate) is_enabled: bool,
    pub(crate) is_hovered: bool,
    pub is_clicked: bool,
    pub(crate) is_currently_selected: bool,
}

impl Default for ButtonProperties {
    fn default() -> Self {
        ButtonProperties {
            is_enabled: true,
            is_hovered: false,
            is_clicked: false,
            is_currently_selected: false,
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
