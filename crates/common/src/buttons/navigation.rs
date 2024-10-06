use bevy::prelude::*;

use crate::states::NavigationState;

use super::configuration::ButtonProperties;

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
