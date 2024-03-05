mod book_opening_tests {
    use book_analyzer::{read_specific_book, BookDetails};
    use epub::doc::EpubDoc;
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
        #[case] title: String,
        #[case] author: String,
    ) {
        let book_path = format!("./tests/data/{file_name}.epub");
        let expected_details = BookDetails {
            title,
            author,
            file: EpubDoc::new(&book_path).unwrap(),
        };

        let option_result = read_specific_book(book_path);

        assert!(option_result.is_some());
        let details = option_result.unwrap();
        assert_eq!(details, expected_details);
    }

    #[test]
    fn should_return_none_when_book_not_exists() {
        let book_path = "./tests/data/not_existing.epub".to_string();

        let option_result = read_specific_book(book_path);

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
        let book_details = read_specific_book(book_path).unwrap();
        let toc = book_details.file.toc;

        //assert
        assert_eq!(toc.len(), expected_sections.len());

        for index in 0..toc.len() {
            assert_eq!(expected_sections[index], toc[index].label);
        }
    }
}

