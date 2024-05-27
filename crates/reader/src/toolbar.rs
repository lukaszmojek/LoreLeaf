use bevy::prelude::*;
use common::{
    flex_container::{FlexContainer, FlexContainerStyle},
    text::TEXT_COLOR,
};

#[derive(Bundle)]
pub struct ReaderToolbarBundle {
    pub(crate) container: FlexContainer,
}

impl ReaderToolbarBundle {
    fn new() -> Self {
        let flex_container_style = FlexContainerStyle {
            background_color: BackgroundColor::from(Color::YELLOW_GREEN),
            height: Val::Percent(6.0),
            min_height: Val::Px(50.0),
            max_height: Val::Px(75.0),
            ..default()
        };

        Self {
            container: FlexContainer::new(Some(flex_container_style)),
        }
    }

    pub fn spawn(parent: &mut ChildBuilder) -> Entity {
        let toolbar_commands = parent
            .spawn(ReaderToolbarBundle::new())
            .with_children(|toolbar| {
                //TODO: Add buttons with actions from epub::EBookReader
                toolbar.spawn(TextBundle::from_section(
                    "TODO: Fill me with reader actions",
                    TextStyle {
                        font_size: 40.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ));
            })
            .id();

        toolbar_commands
    }
}
