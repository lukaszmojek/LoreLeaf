use bevy::prelude::*;
use common::buttons::configuration::ButtonProperties;

use crate::library::Book;

#[derive(Bundle)]
pub struct BookTileBundle {
    button: ButtonBundle,
    button_properties: ButtonProperties,
    book: Book,
}

pub struct BookTileStyle {
    width: Val,
    height: Val,
    margin: UiRect,
    border: UiRect,
}

impl Default for BookTileStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(200.0),
            height: Val::Px(300.0),
            margin: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(5.0)),
        }
    }
}

impl BookTileBundle {
    pub fn new(book: Book, style: Option<BookTileStyle>) -> BookTileBundle {
        let style = match style {
            Some(style) => style,
            None => BookTileStyle::default(),
        };

        let button = ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: style.width,
                height: style.height,
                margin: style.margin,
                border: style.border,
                ..default()
            },
            background_color: BackgroundColor::from(Color::GREEN),
            ..default()
        };

        Self {
            button,
            book,
            button_properties: ButtonProperties::default(),
        }
    }
}
