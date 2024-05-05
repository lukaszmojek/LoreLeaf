use bevy::prelude::*;

#[derive(Bundle)]
pub struct FlexContainer {
    node: NodeBundle,
}

impl FlexContainer {
    pub fn new(style: Option<FlexContainerStyle>) -> Self {
        let style = match style {
            Some(style) => style,
            None => FlexContainerStyle::default(),
        };

        let node = NodeBundle {
            style: Style {
                align_items: style.align_items,
                align_content: style.align_content,
                justify_content: style.justify_content,
                flex_wrap: style.flex_wrap,
                width: style.width,
                height: style.height,
                margin: style.margin,
                ..default()
            },
            background_color: style.background_color,
            ..default()
        };

        Self { node }
    }
}

pub struct FlexContainerStyle {
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    pub flex_wrap: FlexWrap,
    pub width: Val,
    pub height: Val,
    pub margin: UiRect,
    pub background_color: BackgroundColor,
}

impl Default for FlexContainerStyle {
    fn default() -> Self {
        Self {
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_wrap: FlexWrap::Wrap,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(0.0)),
            background_color: BackgroundColor::from(Color::rgba_from_array([0.0, 0.0, 0.0, 0.0])), //Transparent background
        }
    }
}
