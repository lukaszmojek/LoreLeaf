use crate::{chapters::chapter::Chapter, epub::EBook};

pub struct EBookReader {
    book: EBook,
    session: ReadingSession,
}

#[derive(Clone)]
struct ReadingSession {
    current: Chapter,
}

impl EBookReader {
    pub fn new(mut ebook: EBook) -> Self {
        let first_toc_item = ebook.table_of_contents.items.first().unwrap().clone();
        let chapter = Chapter::from_item(first_toc_item, &mut ebook);

        let session = ReadingSession { current: chapter };

        Self {
            book: ebook,
            session,
        }
    }

    pub fn current_chapter(&self) -> Chapter {
        self.session.current.clone()
    }

    pub fn move_to_next_chapter(&mut self) {
        let next_toc_item_to_move_to = self
            .book
            .table_of_contents
            .next_relative(&self.session.current.path);

        if let Some(next_toc_item) = next_toc_item_to_move_to {
            let current_chapter = Chapter::from_item(next_toc_item.clone(), &mut self.book);
            let session = ReadingSession {
                current: current_chapter,
            };
            self.session = session;
        }
    }

    pub fn move_to_previous_chapter(&mut self) {
        let previous_toc_item_to_move_to = self
            .book
            .table_of_contents
            .previous_relative(&self.session.current.path);

        if let Some(previous_toc_item) = previous_toc_item_to_move_to {
            let current_chapter = Chapter::from_item(previous_toc_item.clone(), &mut self.book);
            let session = ReadingSession {
                current: current_chapter,
            };
            self.session = session;
        }
    }
}

#[cfg(test)]
mod reader_tests {
    use std::rc::Rc;

    use crate::{
        chapters::{chapter::Chapter, chapter_node::ChapterNode},
        epub::EBook,
        reader::EBookReader,
    };

    const MOBY_DICK_PATH: &str = "./test_data/epub/moby-dick.epub";

    impl Chapter {
        fn with_path_and_label(path: String, label: String) -> Self {
            Chapter {
                path: path,
                label: label,
                recreated_structure: Rc::new(ChapterNode::new(
                    "tag".to_string(),
                    vec![],
                    "content".to_string(),
                )),
                _raw_content: "raw".to_string(),
            }
        }
    }

    #[test]
    fn should_create_reader() {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let reader = EBookReader::new(book);
        let expected_current_chapter = Chapter::with_path_and_label(
            "OPS/titlepage.xhtml".to_string(),
            "Moby-Dick".to_string(),
        );

        //act
        let session = reader.session;

        //assert
        assert_eq!(expected_current_chapter, session.current);
    }

    #[test]
    fn should_navigate_the_chapters_using_next_and_previous() {
        //arrange
        let book = EBook::read_epub(MOBY_DICK_PATH.to_string()).unwrap();
        let expected_1st_chapter_1st_in_order = Chapter::with_path_and_label(
            "OPS/titlepage.xhtml".to_string(),
            "Moby-Dick".to_string(),
        );

        let expected_2nd_chapter_5th_in_order = Chapter::with_path_and_label(
            "OPS/chapter_001.xhtml".to_string(),
            "Chapter 1. Loomings.".to_string(),
        );

        let expected_3rd_chapter_7th_in_order = Chapter::with_path_and_label(
            "OPS/chapter_003.xhtml".to_string(),
            "Chapter 3. The Spouter-Inn.".to_string(),
        );

        let expected_4th_chapter_6th_in_order = Chapter::with_path_and_label(
            "OPS/chapter_002.xhtml".to_string(),
            "Chapter 2. The Carpet-Bag.".to_string(),
        );

        let expected_5th_chapter_138th_in_order = Chapter::with_path_and_label(
            "OPS/copyright.xhtml".to_string(),
            "Copyright Page".to_string(),
        );

        let mut reader = EBookReader::new(book);

        //act & assert
        //Comparing initial chapter after reader creation
        let current_chapter = reader.current_chapter();
        assert_eq!(current_chapter, expected_1st_chapter_1st_in_order);

        //Trying to move back to previous chapter, should not change the current chapter since reader is on the first one
        reader.move_to_previous_chapter();
        reader.move_to_previous_chapter();
        reader.move_to_previous_chapter();
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

        //Moving to last chapter. There are only 138 chapters in the book, so moving 150 times should land on the last one
        for _ in 0..150 {
            reader.move_to_next_chapter();
        }
        let current_chapter = reader.current_chapter();
        assert_eq!(current_chapter, expected_5th_chapter_138th_in_order);
    }
}
