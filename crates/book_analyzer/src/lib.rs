use epub::doc::EpubDoc;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct BookDetails {
    pub title: String,
    pub author: String,
    pub file: EpubDoc<BufReader<File>>,
}

impl BookDetails {

    pub fn get_spine(&self) -> Vec<String> {
        self.file.spine.clone()
    }
}

impl PartialEq for BookDetails {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

pub fn read_specific_book(book_path: String) -> Option<BookDetails> {
    let doc = EpubDoc::new(book_path);

    if doc.is_err() {
        return None;
    }

    let doc = doc.unwrap();

    //TODO: Improve metadata extraction from book, firstly by list of mappings
    let title = doc.mdata("title").unwrap_or_else(|| "Unknown".to_string());

    let author = doc.mdata("creator").unwrap_or_else(|| "Unknown".to_string());

    Some(BookDetails {
        title,
        author,
        file: doc,
    })
}
