use bevy::prelude::*;

use crate::library::Book;

#[derive(Bundle)]
pub struct BookTileBundle {
    button: ButtonBundle,
    book: Book,
}

impl BookTileBundle {
    pub fn new(book: Book) -> BookTileBundle {
        let button = ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(300.0),
                margin: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor::from(Color::GREEN),
            ..default()
        };

        Self { button, book }
    }
}
