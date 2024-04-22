use crate::{epub::EBook, table_of_contents::TableOfContentsItem};

pub struct Reader {
    book: EBook,
}

impl Reader {
    pub fn new(epub: EBook) -> Self {
        Self { book: epub }
    }

    pub fn current() -> TableOfContentsItem {
        todo!()
    }

    pub fn next() -> TableOfContentsItem {
        todo!()
    }

    pub fn previous() -> TableOfContentsItem {
        todo!()
    }
}

#[cfg(test)]
mod reader_tests {
    use super::*;

    #[test]
    fn next_should_get_next_toc_item() {
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

        let reader = Reader::new(book);

        todo!("Needs some thought whether the reader should be approached in some other way")
    }
}
