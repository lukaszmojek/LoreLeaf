use bevy::prelude::*;

use super::configuration::{
    ButtonProperties, HOVERED_PRESSED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
};

pub fn handle_button_interaction_system(
    mut interaction_query: Query<(&Interaction, &mut ButtonProperties), With<Button>>,
) {
    for (interaction, mut button_properties) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                button_properties.is_clicked = true;
            }
            Interaction::Hovered => {
                button_properties.is_hovered = true;
            }
            Interaction::None => {
                button_properties.is_clicked = false;
                button_properties.is_hovered = false;
            }
        }
    }
}

pub fn update_button_style_system(
    mut interaction_query: Query<(&mut ButtonProperties, &mut BorderColor), (With<Button>)>,
) {
    for (mut button_properties, mut border_color) in &mut interaction_query {
        if button_properties.is_clicked {
            border_color.0 = PRESSED_BUTTON;
            button_properties.is_clicked = false;
            button_properties.is_hovered = true;
            continue;
        }

        if button_properties.is_hovered {
            border_color.0 = HOVERED_PRESSED_BUTTON;
            continue;
        }

        if button_properties.is_currently_selected {
            border_color.0 = Color::YELLOW;
            continue;
        }

        border_color.0 = NORMAL_BUTTON;
    }
}
