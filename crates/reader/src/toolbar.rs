use bevy::prelude::*;
use common::flex_container::{FlexContainer, FlexContainerStyle};

#[derive(Bundle)]
pub struct ReaderToolbarBundle {
    pub(crate) container: FlexContainer,
}

impl ReaderToolbarBundle {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ReaderToolbarBundle {
    fn default() -> Self {
        let flex_container_style = FlexContainerStyle {
            background_color: BackgroundColor::from(Color::YELLOW_GREEN),
            height: Val::Px(75.0),
            ..default()
        };

        Self {
            container: FlexContainer::new(Some(flex_container_style)),
        }
    }
}
