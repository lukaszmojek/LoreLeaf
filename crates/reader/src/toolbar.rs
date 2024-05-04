use bevy::prelude::*;

#[derive(Bundle)]
pub struct ReaderToolbarBundle {
    pub(crate) container: NodeBundle,
}

impl ReaderToolbarBundle {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ReaderToolbarBundle {
    fn default() -> Self {
        Self {
            container: NodeBundle {
                style: Style {
                    align_items: AlignItems::FlexStart,
                    align_content: AlignContent::FlexStart,
                    justify_content: JustifyContent::FlexStart,
                    flex_wrap: FlexWrap::Wrap,
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                background_color: BackgroundColor::from(Color::YELLOW_GREEN),
                ..default()
            },
        }
    }
}
