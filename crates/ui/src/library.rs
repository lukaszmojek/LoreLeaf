use bevy::prelude::*;
use directories::UserDirs;
use std::{
    fs::{self, DirEntry},
    path::Path,
};

// const USER_BOOK_DIRECTORY: UserDirs = match UserDirs::new() {
//     Some(v) => v,
//     None => panic!("Couldn't load user directories"),
// };

pub fn library_system(time: Res<Time>, mut timer: ResMut<RefreshLibraryTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        //TODO: Take those values from user configuration
        let user_directories = UserDirs::new().unwrap();
        let documents = user_directories.document_dir();

        if let Some(path) = documents {
            let books = get_all_books_from_path(path);
            println!("{:?}", books);
        }
    }
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
