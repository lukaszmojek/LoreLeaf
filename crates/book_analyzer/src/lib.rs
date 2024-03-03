use epub::doc::EpubDoc;

#[derive(Debug)]
pub struct BookDetails {
    pub title: String,
    pub author: String
}

impl PartialEq for BookDetails {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title &&
            self.author == other.author
    }
}

pub fn read_specific_book(book_path: String) -> Option<BookDetails> {
    let doc = EpubDoc::new(book_path);

    if doc.is_err() {
        return None
    }

    let doc = doc.unwrap();

    //TODO: Improve metadata extraction from book, firstly by list of mappings
    let title = match doc.mdata("title") {
        None => {"Unknown".to_string()}
        Some(title) => { title }
    };

    let author = match doc.mdata("creator") {
        None => {"Unknown".to_string()},
        Some(author) => { author }
    };

    Some(BookDetails {
        title,
        author
    })
}
