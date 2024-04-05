use bevy::prelude::*;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum NavigationState {
    Home,
    Library,
    Reader,
    LoreExplorer,
    #[default]
    Disabled,
}
