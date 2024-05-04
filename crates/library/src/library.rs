use bevy::prelude::*;
use directories::UserDirs;
use epub::epub::EBook;
use std::{
    fs::{self, DirEntry},
    path::Path,
};

use crate::book_tile::BookTileBundle;

const UNKNOWN: &str = "UNKNOWN";
const BOOK_FORMATS: [&str; 1] = ["epub"];

#[derive(Resource)]
pub struct LibraryViewData {
    pub container_entity: Entity,
}

#[derive(Resource, Debug)]
pub struct UserLibrary {
    pub detected: Vec<Book>,
    pub displayed: Vec<Book>,
    pub to_add: Vec<Book>,
    pub to_remove: Vec<Book>,
    pub selected_for_reading: Option<Book>,
}

impl UserLibrary {
    pub fn empty() -> UserLibrary {
        Self {
            detected: vec![],
            displayed: vec![],
            to_add: vec![],
            to_remove: vec![],
            selected_for_reading: None,
        }
    }

    pub fn set_detected(&mut self, books: Vec<Book>) {
        self.detected = books;
    }

    pub fn set_displayed(&mut self, books: Vec<Book>) {
        self.displayed = books;
    }

    pub fn set_to_add(&mut self, books: Vec<Book>) {
        self.to_add = books;
    }

    pub fn all_added(&mut self) {
        self.displayed.append(&mut self.to_add);
        self.to_add.clear();
    }

    pub fn set_to_remove(&mut self, books: Vec<Book>) {
        self.to_remove = books;
    }

    pub fn set_selected_for_reading(&mut self, book: Book) {
        self.selected_for_reading = Some(book);
    }
}

#[derive(Debug, Clone, Component)]
pub struct Book {
    name: String,
    author: String,
    path: String,
}

impl Book {
    pub fn from_ebook(ebook: EBook) -> Book {
        Self {
            name: ebook.metadata.title.unwrap_or(UNKNOWN.to_string()),
            author: ebook.metadata.creator.unwrap_or(UNKNOWN.to_string()),
            path: ebook.path,
        }
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.author == other.author
    }
}

pub fn detect_books_in_library(
    time: Res<Time>,
    mut timer: ResMut<RefreshLibraryTimer>,
    mut user_library: ResMut<UserLibrary>,
) {
    let should_run_detection =
        timer.0.tick(time.delta()).just_finished() || user_library.displayed.is_empty();

    if should_run_detection {
        //TODO: Take those values from user configuration
        let user_directories = UserDirs::new().unwrap();
        let documents = user_directories.document_dir();

        if let Some(path) = documents {
            let books = get_all_books_from_path(path);
            let books_iterator = books.iter();
            let user_books: Vec<Book> = books_iterator
                .map(|dir_entry| {
                    //TODO: Fix that strange conversion to String
                    let epub_path = dir_entry.path().to_str().unwrap().to_string();

                    EBook::read_epub(epub_path)
                })
                .filter_map(|x| x.ok())
                .map(Book::from_ebook)
                .collect();

            user_library.set_detected(user_books);
        }
    }
}

pub fn compare_books_in_user_library(mut user_library: ResMut<UserLibrary>) {
    let differences = check_differences_in_books_on_ui(&user_library);

    user_library.set_to_add(differences.to_add);
    user_library.set_to_remove(differences.to_remove);
}

struct BookDifference {
    to_add: Vec<Book>,
    to_remove: Vec<Book>,
}

fn check_differences_in_books_on_ui(user_library: &UserLibrary) -> BookDifference {
    let mut to_add: Vec<Book> = vec![];
    let mut to_remove: Vec<Book> = vec![];

    user_library.detected.iter().for_each(|book| {
        let book_tile = book.clone();

        if !user_library.displayed.contains(&book_tile) {
            to_add.push(book_tile);
        }
    });

    user_library.displayed.iter().for_each(|book| {
        if !user_library.detected.contains(&book) {
            to_remove.push(book.clone());
        }
    });

    BookDifference { to_add, to_remove }
}

#[derive(Resource, Deref, DerefMut)]
pub struct RefreshLibraryTimer(pub Timer);

fn get_all_books_from_path(path: &Path) -> Vec<DirEntry> {
    let mut found_books: Vec<DirEntry> = vec![];

    match fs::read_dir(path.to_str().unwrap()) {
        Ok(entries) => {
            found_books = entries
                .filter_map(|dir_entry| dir_entry.ok())
                .filter(|dir_entry| {
                    let binding = dir_entry.to_owned().path();

                    if let Some(entry_path) = binding.to_str() {
                        // TODO: Check all possible formats
                        return entry_path.ends_with(BOOK_FORMATS[0]);
                    }

                    false
                })
                .collect();
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    found_books
}

pub fn refresh_user_library_on_ui(
    mut commands: Commands,
    menu_data: Res<LibraryViewData>,
    mut user_library: ResMut<UserLibrary>,
) {
    //TODO: Try different font since this one is not displaying Polish letters correctly
    for book_to_add in user_library.to_add.iter() {
        let sections = vec![
            TextSection {
                value: "\nName: \n".to_string(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..default()
                },
            },
            TextSection {
                value: book_to_add.name.clone(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..default()
                },
            },
            TextSection {
                value: "\nAuthor: \n".to_string(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..default()
                },
            },
            TextSection {
                value: book_to_add.author.clone(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..default()
                },
            },
        ];

        let entity = commands
            .spawn(BookTileBundle::new(book_to_add.to_owned(), None))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_sections(sections));
            })
            .id();

        commands
            .entity(menu_data.container_entity)
            .push_children(&[entity]);
    }

    user_library.all_added();
}

pub fn book_interaction_system(
    mut interaction_query: Query<(&Interaction, &Book), Changed<Interaction>>,
    mut user_library: ResMut<UserLibrary>,
) {
    for (interaction, book) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            println!("{:?}", &book.path);
            user_library.set_selected_for_reading(book.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use bevy::render::render_resource::encase::rts_array::Length;

    use super::*;

    const TEST_BOOKS_PATH: &str = "test_data/";

    #[test]
    fn test_library_system() {
        let current_directory = env::current_dir().unwrap();
        let path = Path::new(current_directory.to_str().unwrap()).join(TEST_BOOKS_PATH);

        let books = get_all_books_from_path(&path);

        assert_eq!(books.length(), 2);
    }
}

#[cfg(test)]
mod check_differences_in_books_on_ui_tests {
    use super::*;

    #[test]
    fn should_return_empty() {
        let user_library = UserLibrary::empty();

        let book_difference = check_differences_in_books_on_ui(&user_library);

        assert_eq!(book_difference.to_add.len(), 0);
        assert_eq!(book_difference.to_remove.len(), 0);
    }

    #[test]
    fn should_return_1_book_to_add_in_library() {
        let mut user_library = UserLibrary::empty();
        let detected = vec![
            Book {
                name: "Name 1".to_string(),
                author: "Author 1".to_string(),
                path: "".to_string(),
            },
            Book {
                name: "Name 2".to_string(),
                author: "Author 2".to_string(),
                path: "".to_string(),
            },
        ];
        let displayed = vec![Book {
            name: "Name 1".to_string(),
            author: "Author 1".to_string(),
            path: "".to_string(),
        }];
        user_library.set_detected(detected);
        user_library.set_displayed(displayed);

        let book_difference = check_differences_in_books_on_ui(&user_library);

        assert_eq!(book_difference.to_add.len(), 1);
        assert_eq!(book_difference.to_remove.len(), 0);
    }

    #[test]
    fn should_return_1_book_to_remove_from_library() {
        let mut user_library = UserLibrary::empty();
        let detected = vec![Book {
            name: "Name 1".to_string(),
            author: "Author 1".to_string(),
            path: "".to_string(),
        }];
        let displayed = vec![
            Book {
                name: "Name 1".to_string(),
                author: "Author 1".to_string(),
                path: "".to_string(),
            },
            Book {
                name: "Name 2".to_string(),
                author: "Author 2".to_string(),
                path: "".to_string(),
            },
        ];
        user_library.set_detected(detected);
        user_library.set_displayed(displayed);

        let book_difference = check_differences_in_books_on_ui(&user_library);

        assert_eq!(book_difference.to_add.len(), 0);
        assert_eq!(book_difference.to_remove.len(), 1);
    }

    #[test]
    fn should_return_1_book_to_add_and_1_book_to_remove_from_library() {
        let mut user_library = UserLibrary::empty();
        let books = vec![Book {
            name: "Name 2".to_string(),
            author: "Author 2".to_string(),
            path: "".to_string(),
        }];
        let displayed = vec![Book {
            name: "Name 3".to_string(),
            author: "Author 3".to_string(),
            path: "".to_string(),
        }];
        user_library.set_detected(books);
        user_library.set_displayed(displayed);

        let book_difference = check_differences_in_books_on_ui(&user_library);

        assert_eq!(book_difference.to_add.len(), 1);
        assert_eq!(book_difference.to_remove.len(), 1);
    }

    #[test]
    fn should_set_selected_for_reading() {
        let mut user_library = UserLibrary::empty();
        let book_clicked = Book {
            name: "Name".to_string(),
            author: "Author".to_string(),
            path: "./123".to_string(),
        };

        user_library.set_selected_for_reading(book_clicked.clone());
        let selected_book = user_library.selected_for_reading.clone().unwrap();

        assert_eq!(book_clicked, selected_book);
    }
}
