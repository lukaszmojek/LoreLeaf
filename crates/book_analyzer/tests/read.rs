mod book_opening_tests_epubrs {
    use book_analyzer::{BookDetails};
    use rstest::rstest;

    //DISCLAIMER: those books are free and downloaded from https://www.gutenberg.org/
    #[rstest]
    #[case::the_wonderful_wizard_of_oz_epub3(
        "the_wonderful_wizard_of_oz_v3",
        "The Wonderful Wizard of Oz",
        "L. Frank Baum"
    )]
    #[case::the_wonderful_wizard_of_oz_epub2(
        "the_wonderful_wizard_of_oz_v2",
        "The Wonderful Wizard of Oz",
        "L. Frank Baum"
    )]
    #[case::the_wonderful_wizard_of_oz_v2_epub2_no_images(
        "the_wonderful_wizard_of_oz_v2_no_images",
        "The Wonderful Wizard of Oz",
        "L. Frank Baum"
    )]
    fn should_return_some_with_details_when_book_exists(
        #[case] file_name: String,
        #[case] expected_title: String,
        #[case] expected_author: String,
    ) {
        let book_path = format!("./tests/data/{file_name}.epub");

        let option_result = BookDetails::create_with_epubrs(book_path);

        assert!(option_result.is_some());
        let details = option_result.unwrap();
        assert_eq!(details.title, expected_title);
        assert_eq!(details.author, expected_author);
    }

    #[test]
    fn should_return_none_when_book_not_exists() {
        let book_path = "./tests/data/not_existing.epub".to_string();

        let option_result = BookDetails::create_with_epubrs(book_path);

        assert!(option_result.is_none());
    }

    #[rstest]
    #[case::the_wonderful_wizard_of_oz_v3("the_wonderful_wizard_of_oz_v3")]
    #[case::the_wonderful_wizard_of_oz_v2("the_wonderful_wizard_of_oz_v2")]
    #[case::the_wonderful_wizard_of_oz_v2_no_images("the_wonderful_wizard_of_oz_v2_no_images")]
    fn toc_should_contain_list_of_all_sections(#[case] file_name: String) {
        //arrange
        let book_path = format!("./tests/data/{file_name}.epub");
        let expected_sections = vec![
            "The Wonderful Wizard of Oz",
            "Contents",
            "Introduction",
            "The Wonderful Wizard of Oz",
            "Chapter I The Cyclone",
            "Chapter II The Council with the Munchkins",
            "Chapter III How Dorothy Saved the Scarecrow",
            "Chapter IV The Road Through the Forest",
            "Chapter V The Rescue of the Tin Woodman",
            "Chapter VI The Cowardly Lion",
            "Chapter VII The Journey to the Great Oz",
            "Chapter VIII The Deadly Poppy Field",
            "Chapter IX The Queen of the Field Mice",
            "Chapter X The Guardian of the Gate",
            "Chapter XI The Wonderful City of Oz",
            "Chapter XII The Search for the Wicked Witch",
            "Chapter XIII The Rescue",
            "Chapter XIV The Winged Monkeys",
            "Chapter XV The Discovery of Oz, the Terrible",
            "Chapter XVI The Magic Art of the Great Humbug",
            "Chapter XVII How the Balloon Was Launched",
            "Chapter XVIII Away to the South",
            "Chapter XIX Attacked by the Fighting Trees",
            "Chapter XX The Dainty China Country",
            "Chapter XXI The Lion Becomes the King of Beasts",
            "Chapter XXII The Country of the Quadlings",
            "Chapter XXIII Glinda The Good Witch Grants Dorothy’s Wish",
            "Chapter XXIV Home Again",
            "THE FULL PROJECT GUTENBERG LICENSE",
        ];

        //act
        let book_details = BookDetails::create_with_epubrs(book_path).unwrap();
        let toc = &book_details.table_of_contents();

        //assert
        assert_eq!(toc.len(), expected_sections.len());

        for index in 0..toc.len() {
            assert_eq!(expected_sections[index], toc[index].label);
        }
    }
}

mod book_opening_tests_rbook {
    use book_analyzer::{BookDetails};
    use rbook::Ebook;
    use rstest::rstest;

    //DISCLAIMER: those books are free and downloaded from https://www.gutenberg.org/
    #[rstest]
    #[case::the_wonderful_wizard_of_oz_epub3(
    "the_wonderful_wizard_of_oz_v3",
    "The Wonderful Wizard of Oz",
    "L. Frank Baum"
    )]
    fn should_return_some_with_details_when_book_exists(
        #[case] file_name: String,
        #[case] expected_title: String,
        #[case] expected_author: String,
    ) {
        let book_path = format!("./tests/data/{file_name}.epub");
        
        let epub = rbook::Epub::new(&book_path).unwrap();
        let mut reader = epub.reader();

        while let Some(Ok(content)) = reader.next_page() {
            println!("{content}")
        }
    }

    #[test]
    fn should_return_none_when_book_not_exists() {
        let book_path = "./tests/data/not_existing.epub".to_string();

        let option_result = BookDetails::create_with_rbook(book_path);

        assert!(option_result.is_none());
    }

    #[rstest]
    #[case::the_wonderful_wizard_of_oz_v3("the_wonderful_wizard_of_oz_v3")]
    #[case::the_wonderful_wizard_of_oz_v2("the_wonderful_wizard_of_oz_v2")]
    #[case::the_wonderful_wizard_of_oz_v2_no_images("the_wonderful_wizard_of_oz_v2_no_images")]
    fn toc_should_contain_list_of_all_sections(#[case] file_name: String) {
        //arrange
        let book_path = format!("./tests/data/{file_name}.epub");
        let expected_sections = vec![
            "The Wonderful Wizard of Oz",
            "Contents",
            "Introduction",
            "The Wonderful Wizard of Oz",
            "Chapter I The Cyclone",
            "Chapter II The Council with the Munchkins",
            "Chapter III How Dorothy Saved the Scarecrow",
            "Chapter IV The Road Through the Forest",
            "Chapter V The Rescue of the Tin Woodman",
            "Chapter VI The Cowardly Lion",
            "Chapter VII The Journey to the Great Oz",
            "Chapter VIII The Deadly Poppy Field",
            "Chapter IX The Queen of the Field Mice",
            "Chapter X The Guardian of the Gate",
            "Chapter XI The Wonderful City of Oz",
            "Chapter XII The Search for the Wicked Witch",
            "Chapter XIII The Rescue",
            "Chapter XIV The Winged Monkeys",
            "Chapter XV The Discovery of Oz, the Terrible",
            "Chapter XVI The Magic Art of the Great Humbug",
            "Chapter XVII How the Balloon Was Launched",
            "Chapter XVIII Away to the South",
            "Chapter XIX Attacked by the Fighting Trees",
            "Chapter XX The Dainty China Country",
            "Chapter XXI The Lion Becomes the King of Beasts",
            "Chapter XXII The Country of the Quadlings",
            "Chapter XXIII Glinda The Good Witch Grants Dorothy’s Wish",
            "Chapter XXIV Home Again",
            "THE FULL PROJECT GUTENBERG LICENSE",
        ];

        //act
        let book_details = BookDetails::create_with_rbook(book_path).unwrap();
        let toc = book_details;

        //assert
        assert_eq!(toc.len(), expected_sections.len());

        for index in 0..toc.len() {
            assert_eq!(expected_sections[index], toc[index].label);
        }
    }
}

// mod book_traversing_tests {
//     use book_analyzer::{BookDetails};
//     use epub::doc::EpubDoc;
//     use rstest::rstest;
// 
//     #[test]
//     fn should_get_first_chapter() {
//         let book_path = "./tests/data/the_wonderful_wizard_of_oz_v3.epub".to_string();
//         let expected_text = "Dorothy lived in the midst of the great Kansas prairies, with Uncle Henry, who was a farmer";
// 
//         let mut book_details = BookDetails::create_with_epubrs(book_path).unwrap();
//         let chapter = book_details.get_chapter(2);
// 
//         // let spine = book_details.get_spine();
// 
//         // let some_chapter = book_details.file.get_resource(spine[0].as_str());
//         // let chapter_content = String::from_utf8(some_chapter.unwrap().0);
// 
//         // assert_eq!(spine.len(), 1);
//         //
//         assert!(chapter.is_some());
//         println!("{:?}", expected_text);
//         assert!(chapter.unwrap().0.contains(expected_text));
//     }
// 
//     #[test]
//     fn should_get_content_at_page() {
//         let book_path = "./tests/data/the_wonderful_wizard_of_oz_v3.epub".to_string();
//         let expected_text = "Dorothy lived in the midst of the great Kansas prairies, with Uncle Henry, who was a farmer";
// 
//         let mut book_details = BookDetails::create_with_epubrs(book_path).unwrap();
//         let chapter = book_details.get_page(2);
// 
//         // let spine = book_details.get_spine();
// 
//         // let some_chapter = book_details.file.get_resource(spine[0].as_str());
//         // let chapter_content = String::from_utf8(some_chapter.unwrap().0);
// 
//         // assert_eq!(spine.len(), 1);
//         //
//         assert!(chapter.is_some());
//         println!("{:?}", String::from_utf8(chapter.unwrap().0));
//         // assert!(chapter.unwrap()..contains(expected_text));
//     }
//     
//     #[test]
//     fn should_get_whole_book_contents() {
//         let book_path = "./tests/data/the_wonderful_wizard_of_oz_v3.epub".to_string();
//         let expected_text = "Dorothy lived in the midst of the great Kansas prairies, with Uncle Henry, who was a farmer";
// 
//         let mut book_details = BookDetails::create_with_epubrs(book_path).unwrap();
//         let contents = book_details.get_book_contents();
// 
//         // let spine = book_details.get_spine();
// 
//         // let some_chapter = book_details.file.get_resource(spine[0].as_str());
//         // let chapter_content = String::from_utf8(some_chapter.unwrap().0);
// 
//         // assert_eq!(spine.len(), 1);
//         // assert!(chapter.unwrap()..contains(expected_text));
// 
//         for content in contents {
//             println!("{:?}", content)
//         }
//     }
// }
