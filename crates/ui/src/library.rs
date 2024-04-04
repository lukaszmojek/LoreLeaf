use bevy::prelude::*;
use directories::UserDirs;
use epub::EBook;
use std::{
    borrow::BorrowMut,
    fs::{self, DirEntry},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::screens::home::OnLibraryScreen;

const UNKNOWN: &str = "UNKNOWN";
const BOOK_FORMATS: [&str; 1] = ["epub"];

#[derive(Resource, Debug)]
pub struct UserLibrary {
    pub detected: Vec<Book>,
    pub displayed: Vec<Book>,
    pub to_add: Vec<Book>,
    pub to_remove: Vec<Book>,
}

impl UserLibrary {
    pub fn empty() -> UserLibrary {
        Self {
            detected: vec![],
            displayed: vec![],
            to_add: vec![],
            to_remove: vec![],
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

    pub fn set_to_remove(&mut self, books: Vec<Book>) {
        self.to_remove = books;
    }
}

#[derive(Debug, Clone)]
struct Book {
    name: String,
    author: String,
}

impl Book {
    pub fn from_ebook(ebook: EBook) -> Book {
        Self {
            name: ebook.metadata.title.unwrap_or(UNKNOWN.to_string()),
            author: ebook.metadata.creator.unwrap_or(UNKNOWN.to_string()),
        }
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.author == other.author
    }
}

pub fn refresh_user_library(
    time: Res<Time>,
    mut timer: ResMut<RefreshLibraryTimer>,
    mut user_library: ResMut<UserLibrary>,
) {
    if timer.0.tick(time.delta()).just_finished() {
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
                    let epub = EBook::read_epub(epub_path);

                    epub
                })
                .filter(|entry| entry.is_ok())
                .map(|x| x.unwrap())
                .map(|y| Book::from_ebook(y))
                .collect();

            user_library.set_detected(user_books);
        }
    }
}

//TODO: Change UserLibrary needs to be a resource, since it is a unique data
//https://bevyengine.org/learn/quick-start/getting-started/resources/
pub fn display_user_library(
    time: Res<Time>,
    mut timer: ResMut<RefreshLibraryTimer>,
    mut user_library: ResMut<UserLibrary>,
    mut commands: Commands,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for book in user_library.detected.iter() {
            //TODO: Fix Polish letters not being displayed correctly
            println!("{:#?}", book);
        }

        let differences = check_differences_in_books_on_ui(&user_library);

        let to_add = differences.to_add;
        let to_remove = differences.to_remove;

        for book in to_add {
            println!("ADD: {:#?}", book);
        }

        for book in to_remove {
            println!("REMOVE: {:#?}", book);
        }
    }
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

fn print_current_time() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_seconds = since_the_epoch.as_secs();

    println!("{:?}", in_seconds);
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

#[cfg(test)]
mod tests {
    use std::env;

    use bevy::render::render_resource::encase::rts_array::Length;

    use super::*;

    #[test]
    fn test_library_system() {
        let current_directory = env::current_dir().unwrap();
        let path = Path::new(current_directory.to_str().unwrap()).join("test_data/");

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
            },
            Book {
                name: "Name 2".to_string(),
                author: "Author 2".to_string(),
            },
        ];
        let displayed = vec![Book {
            name: "Name 1".to_string(),
            author: "Author 1".to_string(),
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
        }];
        let displayed = vec![
            Book {
                name: "Name 1".to_string(),
                author: "Author 1".to_string(),
            },
            Book {
                name: "Name 2".to_string(),
                author: "Author 2".to_string(),
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
        }];
        let displayed = vec![Book {
            name: "Name 3".to_string(),
            author: "Author 3".to_string(),
        }];
        user_library.set_detected(books);
        user_library.set_displayed(displayed);

        let book_difference = check_differences_in_books_on_ui(&user_library);

        assert_eq!(book_difference.to_add.len(), 1);
        assert_eq!(book_difference.to_remove.len(), 1);
    }
}
