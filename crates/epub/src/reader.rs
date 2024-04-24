use crate::{epub::EBook, table_of_contents::TableOfContentsItem};

pub struct EBookReader {
    book: EBook,
    session: ReadingSession,
}

#[derive(Clone)]
struct ReadingSession {
    current: Chapter,
    next: Option<Chapter>,
    previous: Option<Chapter>,
}

impl EBookReader {
    pub fn new(mut ebook: EBook) -> Self {
        let first_toc_item = ebook.table_of_contents.items.first().unwrap().clone();
        let chapter = Chapter::from_item(first_toc_item, &mut ebook);

        let session = ReadingSession {
            current: chapter,
            next: None,
            previous: None,
        };

        Self {
            book: ebook,
            session,
        }
    }

    pub fn current_chapter(&self) -> Chapter {
        self.session.current.clone()
    }

    pub fn move_to_next_chapter(&self) {
        todo!()
    }

    pub fn move_to_previous_chapter(&self) {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Chapter {
    pub path: String,
    pub label: String,
    pub content: String,
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.label == other.label
    }
}

impl Chapter {
    fn from_item(item: TableOfContentsItem, ebook: &mut EBook) -> Chapter {
        //TODO: Consider moving chapter creation to this method invocation, since from this point on TocItem is not itself anymore
        let content = ebook.get_content_by_toc_item(&item).unwrap();

        Chapter {
            path: item.path.clone(),
            label: item.label.clone(),
            content,
        }
    }
}

#[cfg(test)]
mod reader_tests {
    use super::*;

    #[test]
    fn should_create_reader() {
        //arrange
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();
        let reader = EBookReader::new(book);
        let expected_current_chapter = Chapter {
            path: "OPS/titlepage.xhtml".to_string(),
            label: "Moby-Dick".to_string(),
            content: "".to_string(),
        };

        //act
        let session = reader.session;

        //assert
        assert_eq!(expected_current_chapter, session.current);
        assert_eq!(None, session.next);
        assert_eq!(None, session.previous);
    }

    #[test]
    fn should_navigate_the_chapters_using_next_and_previous() {
        //arrange
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();
        let expected_1st_chapter_1st_in_order = Chapter {
            path: "OPS/titlepage.xhtml".to_string(),
            label: "Moby-Dick".to_string(),
            content: "".to_string(),
        };

        let expected_2nd_chapter_5th_in_order = Chapter {
            path: "OPS/chapter_001.xhtml".to_string(),
            label: "Chapter 1. Loomings".to_string(),
            content: "".to_string(),
        };

        let expected_3rd_chapter_7th_in_order = Chapter {
            path: "OPS/chapter_003.xhtml".to_string(),
            label: "Chapter 3. The Spouter-Inn.".to_string(),
            content: "".to_string(),
        };

        let expected_4th_chapter_6th_in_order = Chapter {
            path: "OPS/chapter_002.xhtml".to_string(),
            label: "Chapter 2. The Carpet-Bag.".to_string(),
            content: "".to_string(),
        };

        let reader = EBookReader::new(book);

        //act & assert
        //Comparing initial chapter after reader creation
        let current_chapter = reader.current_chapter();
        assert_eq!(current_chapter, expected_1st_chapter_1st_in_order);

        //Moving a couple of chapters in order to the 5th one
        reader.move_to_next_chapter();
        reader.move_to_next_chapter();
        reader.move_to_next_chapter();
        reader.move_to_next_chapter();
        let current_chapter = reader.current_chapter();
        assert_eq!(current_chapter, expected_2nd_chapter_5th_in_order);

        //Moving a couple of chapters in order to the 7th one
        reader.move_to_next_chapter();
        reader.move_to_next_chapter();
        let current_chapter = reader.current_chapter();
        assert_eq!(current_chapter, expected_3rd_chapter_7th_in_order);

        //Moving to a previous chapter, 6th in order
        reader.move_to_previous_chapter();
        let current_chapter = reader.current_chapter();
        assert_eq!(current_chapter, expected_4th_chapter_6th_in_order);

        //Moving to a previous chapter, then to the next in order, landing on 6th
        reader.move_to_previous_chapter();
        reader.move_to_next_chapter();
        let current_chapter = reader.current_chapter();
        assert_eq!(current_chapter, expected_4th_chapter_6th_in_order);
    }
}

#[cfg(test)]
mod chapter_tests {
    use super::*;

    mod partial_eq {
        use super::*;

        #[test]
        fn partial_eq_should_check_path() {
            //arrange
            let chapter1 = create_chapter("1", "1", "111");
            let chapter2 = create_chapter("2", "1", "111");

            //assert
            assert_ne!(chapter1, chapter2);
        }

        #[test]
        fn partial_eq_should_check_label() {
            //arrange
            let chapter1 = create_chapter("1", "1", "111");
            let chapter2 = create_chapter("1", "2", "111");

            //assert
            assert_ne!(chapter1, chapter2);
        }

        #[test]
        fn partial_eq_should_not_check_content() {
            //arrange
            let chapter1 = create_chapter("1", "1", "111");
            let chapter2 = create_chapter("1", "1", "222");

            //assert
            assert_eq!(chapter1, chapter2);
        }

        fn create_chapter(path: &str, label: &str, content: &str) -> Chapter {
            Chapter {
                path: path.to_string(),
                label: label.to_string(),
                content: content.to_string(),
            }
        }
    }
}
