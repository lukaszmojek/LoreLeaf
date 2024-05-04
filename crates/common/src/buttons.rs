use bevy::prelude::*;

use crate::{states::NavigationState, text::TEXT_COLOR};

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

#[derive(Component, Debug)]
pub enum NavigationButtonAction {
    Home,
    Library,
    Reader,
    LoreExplorer,
}

pub struct NavigationButtonProperties {
    pub action: NavigationButtonAction,
}

//TODO: Use those properties in the system and then update view in different system
#[derive(Component)]
pub struct ButtonProperties {
    pub(crate) is_enabled: bool,
    pub(crate) is_hovered: bool,
    pub(crate) is_clicked: bool,
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

pub fn navigation_button_interaction_system(
    mut interaction_query: Query<(&mut ButtonProperties, &NavigationButtonAction), With<Button>>,
    current_navigation_state: Res<State<NavigationState>>,
    mut next_navigation_state: ResMut<NextState<NavigationState>>,
) {
    for (mut button_properties, navigation_button_action) in &mut interaction_query {
        let assigned_navigation_state = match navigation_button_action {
            NavigationButtonAction::Home => NavigationState::Home,
            NavigationButtonAction::Library => NavigationState::Library,
            NavigationButtonAction::Reader => NavigationState::Reader,
            NavigationButtonAction::LoreExplorer => NavigationState::LoreExplorer,
        };

        if button_properties.is_clicked {
            next_navigation_state.set(assigned_navigation_state);
        }

        button_properties.is_currently_selected =
            current_navigation_state.as_ref() == &assigned_navigation_state;
    }
}
