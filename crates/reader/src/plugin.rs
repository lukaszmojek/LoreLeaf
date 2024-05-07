use bevy::prelude::*;
use common::{
    flex_container::FlexContainer, screens::MainScreenViewData, states::NavigationState,
    text::TEXT_COLOR, utilities::despawn_screen,
};
use library::library::UserLibrary;

use crate::toolbar::ReaderToolbarBundle;

#[derive(Component)]
pub struct OnReaderScreen;

pub struct ReaderPlugin;

impl Plugin for ReaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(NavigationState::Reader), (reader_setup).chain())
            // .add_systems(Update, ().chain().run_if(in_state(NavigationState::Reader)))
            // .add_systems(Update, ().run_if(in_state(NavigationState::Library)))
            .add_systems(
                OnExit(NavigationState::Reader),
                despawn_screen::<OnReaderScreen>,
            );
    }
}

fn reader_setup(
    mut commands: Commands,
    main_screen_view_data: Res<MainScreenViewData>,
    user_library: Res<UserLibrary>,
) {
    let selected_book = user_library.selected_for_reading().clone();

    let reader_screen = commands
        .spawn((FlexContainer::new(None), OnReaderScreen))
        .with_children(|parent| {
            let toolbar_entity = ReaderToolbarBundle::spawn(parent);

            let mut book_title = "Book not found".to_string();

            if let Some(book) = selected_book {
                book_title = book.name.clone();
            }

            parent.spawn(
                TextBundle::from_section(
                    book_title,
                    TextStyle {
                        font_size: 80.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        })
        .id();

    commands
        .entity(main_screen_view_data.container_entity)
        .push_children(&[reader_screen]);
}
