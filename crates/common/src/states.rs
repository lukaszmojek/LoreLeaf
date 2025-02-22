use bevy::prelude::*;

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum NavigationState {
    #[default]
    Home,
    Library,
    Reader,
    LoreExplorer,
    Exit,
}
