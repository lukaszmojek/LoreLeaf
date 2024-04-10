use bevy::prelude::*;
use common::text::TEXT_COLOR;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct ButtonConfiguration {
    pub style: Style,
    pub icon_style: Style,
    pub text_style: TextStyle,
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
// TODO: Check previous query
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    // println!("{:?}", interaction_query);

    for (interaction, mut border_color) in &mut interaction_query {
        println!("{:?}", interaction);

        border_color.0 = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_PRESSED_BUTTON,
            // Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
    }
}
