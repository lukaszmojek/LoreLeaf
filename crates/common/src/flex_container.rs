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
                flex_direction: style.flex_direction,
                flex_wrap: style.flex_wrap,
                align_items: style.align_items,
                align_content: style.align_content,
                justify_content: style.justify_content,
                width: style.width,
                min_width: style.min_width,
                max_width: style.max_width,
                height: style.height,
                min_height: style.min_height,
                max_height: style.max_height,
                margin: style.margin,
                overflow: style.overflow,
                ..default()
            },
            background_color: style.background_color,
            ..default()
        };

        Self { node }
    }
}

pub struct FlexContainerStyle {
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub justify_content: JustifyContent,
    pub width: Val,
    pub min_width: Val,
    pub max_width: Val,
    pub height: Val,
    pub min_height: Val,
    pub max_height: Val,
    pub margin: UiRect,
    pub overflow: Overflow,
    pub background_color: BackgroundColor,
}

impl Default for FlexContainerStyle {
    fn default() -> Self {
        Self {
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            justify_content: JustifyContent::FlexStart,
            width: Val::Percent(100.0),
            min_width: Val::Auto,
            max_width: Val::Auto,
            height: Val::Percent(100.0),
            min_height: Val::Auto,
            max_height: Val::Auto,
            margin: UiRect::all(Val::Px(0.0)),
            overflow: Overflow {
                x: OverflowAxis::Visible,
                y: OverflowAxis::Visible,
            },
            background_color: BackgroundColor::from(Color::rgba_from_array([0.0, 0.0, 0.0, 0.0])), //Transparent background
        }
    }
}
