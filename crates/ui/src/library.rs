use bevy::prelude::*;
use directories::UserDirs;
use std::fs::{self, DirEntry};

pub fn library_system(time: Res<Time>, mut timer: ResMut<RefreshLibraryTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        let books = get_all_books_from_documents();
        println!("{:?}", books);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct RefreshLibraryTimer(pub Timer);

const BOOK_FORMATS: [&str; 1] = ["epub"];

fn get_all_books_from_documents() -> Vec<DirEntry> {
    let user_directories = UserDirs::new().unwrap();
    let documents = user_directories.document_dir();
    let mut found_books: Vec<DirEntry> = vec![];

    match fs::read_dir(documents.unwrap().to_str().unwrap()) {
        Ok(entries) => {
            found_books = entries
                .filter_map(|dir_entry| dir_entry.ok())
                .filter(|dir_entry| {
                    let binding = dir_entry.to_owned().path();
                    let entry_path = binding.to_str().unwrap_or("ERROR");

                    // TODO: Check all possible formats
                    entry_path.ends_with(BOOK_FORMATS[0])
                })
                .collect();
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    found_books
}

#[cfg(test)]
mod tests {
    use bevy::render::render_resource::encase::rts_array::Length;

    use super::*;

    #[test]
    fn test_library_system() {
        //TODO: Make the path configurable
        let books = get_all_books_from_documents();
        println!("{:?}", books);
        assert_eq!(books.length(), 3);
    }
}
