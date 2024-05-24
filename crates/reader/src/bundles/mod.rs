use bevy::{
    text::Text,
    ui::node_bundles::{NodeBundle, TextBundle},
};

pub enum ChapterNodeComponent {
    Heading(HeadingComponentBundle),
    Paragraph(ParagraphComponentBundle),
    Image(ImageComponentBundle),
    List(ListComponentBundle),
}

//TODO: Create factory methods for those
pub struct HeadingComponentBundle {
    node: TextBundle,
    pub children: Vec<ChapterNodeComponent>,
}

impl HeadingComponentBundle {
    pub fn new(content: &str, children: Vec<ChapterNodeComponent>) -> Self {
        Self {
            node: TextBundle {
                text: Text::from_section(content, Default::default()),
                style: Default::default(),
                ..Default::default()
            },
            children: children,
        }
    }
}

pub struct ParagraphComponentBundle {
    node: TextBundle,
    children: Vec<ChapterNodeComponent>,
}

impl ParagraphComponentBundle {
    pub fn new(content: &str, children: Vec<ChapterNodeComponent>) -> Self {
        Self {
            node: TextBundle {
                text: Text::from_section(content, Default::default()),
                style: Default::default(),
                ..Default::default()
            },
            children: children,
        }
    }
}

pub struct ImageComponentBundle {
    node: NodeBundle,
    children: Vec<ChapterNodeComponent>,
}

pub struct ListComponentBundle {
    node: NodeBundle,
    children: Vec<ChapterNodeComponent>,
}
