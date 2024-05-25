use bevy::{
    text::Text,
    ui::node_bundles::{NodeBundle, TextBundle},
};

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
                text: Text::from_section(content, Default::default()),
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
                text: Text::from_section(content, Default::default()),
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
