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
            height: Val::Px(75.0),
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
                toolbar.spawn(TextBundle::from_section(
                    "1",
                    TextStyle {
                        font_size: 40.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ));

                toolbar.spawn(TextBundle::from_section(
                    "2",
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
