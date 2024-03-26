use directories::UserDirs;
use std::fs::{self, DirEntry};

pub fn library_system() {}

const BOOK_FORMATS: [&str; 1] = ["epub"];

fn get_all_books_from_documents() {
    let user_directories = UserDirs::new().unwrap();
    let documents = user_directories.document_dir();
    let mut found_books: Vec<DirEntry> = vec![];

    match fs::read_dir(documents.unwrap().to_str().unwrap()) {
        Ok(entries) => {
            found_books = entries
                .filter_map(|dirEntry| dirEntry.ok())
                .filter(|dirEntry| dirEntry.path().ends_with(BOOK_FORMATS[0]))
                .collect();
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("{:?}", found_books);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_system() {
        //TODO: Make the path configurable
        get_all_books_from_documents();
        panic!("Test failed")
    }
}
