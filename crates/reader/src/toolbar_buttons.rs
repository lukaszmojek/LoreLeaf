use bevy::prelude::*;
use common::buttons::configuration::ButtonProperties;
use library::library::UserLibrary;

#[derive(Bundle)]
pub struct ReaderToolbarButton {
    pub button: ButtonBundle,
    pub properties: ButtonProperties,
    pub action: ReaderToolbarButtonAction,
}

#[derive(Component, Debug)]
pub enum ReaderToolbarButtonAction {
    PreviousChapter,
    NextChapter,
}

pub fn toolbar_button_interaction_system(
    mut interaction_query: Query<(&mut ButtonProperties, &ReaderToolbarButtonAction), With<Button>>,
    user_library: Res<UserLibrary>,
) {
    for (mut button_properties, navigation_button_action) in &mut interaction_query {
        let assigned_navigation_state = match navigation_button_action {
            ReaderToolbarButtonAction::PreviousChapter => NavigationState::Home,
            ReaderToolbarButtonAction::NextChapter => NavigationState::Library,
        };

        if button_properties.is_clicked {
            let selected_book = user_library.selected_for_reading().clone();

            let user_library.set(assigned_navigation_state);
        }
    }
}
