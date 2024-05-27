use bevy::{
    prelude::default,
    text::{Text, TextLayoutInfo, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        widget::TextFlags,
    },
};
use common::text::TEXT_COLOR;

#[derive(Debug)]
pub enum ChapterNodeComponent {
    Heading(HeadingComponentBundle),
    Paragraph(ParagraphComponentBundle),
    Image(ImageComponentBundle),
    List(ListComponentBundle),
}

//TODO: Create factory methods for those
#[derive(Debug)]
pub struct HeadingComponentBundle {
    pub node: TextBundle,
}

impl HeadingComponentBundle {
    pub fn new(content: &str) -> Self {
        Self {
            node: TextBundle {
                text: Text::from_section(
                    content,
                    TextStyle {
                        font_size: 60.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ),
                style: Default::default(),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug)]
pub struct ParagraphComponentBundle {
    pub node: TextBundle,
}

impl ParagraphComponentBundle {
    pub fn new(content: &str) -> Self {
        Self {
            node: TextBundle {
                text: Text::from_section(
                    content,
                    TextStyle {
                        font_size: 20.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ),
                style: Default::default(),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug)]
pub struct ImageComponentBundle {
    pub node: NodeBundle,
}

#[derive(Debug)]
pub struct ListComponentBundle {
    pub node: NodeBundle,
}
