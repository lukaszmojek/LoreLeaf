use bevy::prelude::*;
use directories::UserDirs;
use epub::EBook;
use std::{
    fs::{self, DirEntry},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Component, Debug)]
pub struct UserLibrary {
    pub books: Vec<Book>,
}

impl UserLibrary {
    pub fn empty() -> UserLibrary {
        Self { books: vec![] }
    }

    pub fn set_books(&mut self, books: Vec<Book>) {
        self.books = books;
    }
}

#[derive(Debug)]
struct Book {
    name: String,
    author: String,
}

const UNKNOWN: &str = "UNKNOWN";

impl Book {
    pub fn from_ebook(ebook: EBook) -> Book {
        Self {
            name: ebook.metadata.title.unwrap_or(UNKNOWN.to_string()),
            author: ebook.metadata.creator.unwrap_or(UNKNOWN.to_string()),
        }
    }
}

pub fn initialize_library(mut commands: Commands) {
    commands.spawn(UserLibrary::empty());
}

pub fn refresh_user_library(
    time: Res<Time>,
    mut timer: ResMut<RefreshLibraryTimer>,
    mut query: Query<&mut UserLibrary>,
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
                    let epub_path = String::from(dir_entry.path().to_str().unwrap().to_string());
                    let epub = EBook::read_epub(epub_path);

                    epub
                })
                .filter(|entry| entry.is_ok())
                .map(|x| x.unwrap())
                .map(|y| Book::from_ebook(y))
                .collect();

            query.single_mut().set_books(user_books);
        }
    }
}

pub fn print_user_library(
    time: Res<Time>,
    mut timer: ResMut<RefreshLibraryTimer>,
    query: Query<&UserLibrary>,
) {
    //TODO: Fix Polish letters not being displayed correctly
    if timer.0.tick(time.delta()).just_finished() {
        for book in &query {
            println!("{:#?}", book);
        }
    }
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

const BOOK_FORMATS: [&str; 1] = ["epub"];

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
